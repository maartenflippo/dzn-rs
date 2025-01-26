use std::{collections::HashSet, fmt::Debug, hash::Hash, io::BufRead, str::FromStr};

use winnow::{
    ascii::multispace0,
    combinator::{alt, delimited, preceded, separated, trace},
    error::{ContextError, ErrMode, ParserError},
    token::take_while,
    PResult, Parser,
};

use crate::{
    value::{ShapedArray, Value, ValueArray},
    DataFile, DznParseError, SyntaxElement,
};

/// Parse the data file from the given source.
pub fn parse<Int>(mut source: impl BufRead) -> Result<DataFile<Int>, DznParseError>
where
    Int: FromStr + Debug + Hash + Eq,
    Int::Err: Debug,
{
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;

    let mut values = vec![];
    let mut arrays_1d = vec![];
    let mut arrays_2d = vec![];

    let mut rest = buffer.trim();

    while !rest.is_empty() {
        let ident = preceded(multispace0, identifier)
            .parse_next(&mut rest)
            .map_err(|_| DznParseError::InvalidSyntax {
                expected: SyntaxElement::Identifier,
                actual: rest.to_string(),
            })?;

        let _ = delimited(multispace0, "=", multispace0)
            .parse_next(&mut rest)
            .map_err(|_: ErrMode<ContextError>| DznParseError::InvalidSyntax {
                expected: SyntaxElement::Equals,
                actual: rest.to_string(),
            })?;

        let value =
            value_or_array
                .parse_next(&mut rest)
                .map_err(|_| DznParseError::InvalidSyntax {
                    expected: SyntaxElement::Identifier,
                    actual: rest.to_string(),
                })?;

        dbg!(&value);

        let _ = delimited(multispace0, ";", multispace0)
            .parse_next(&mut rest)
            .map_err(|_: ErrMode<ContextError>| DznParseError::InvalidSyntax {
                expected: SyntaxElement::SemiColon,
                actual: rest.to_string(),
            })?;

        match value {
            ValueOrArray::Value(value) => values.push((ident, value)),
            ValueOrArray::Array1d(value_array) => arrays_1d.push((ident, value_array)),
            ValueOrArray::Array2d(value_array) => arrays_2d.push((ident, value_array)),
        }
    }

    Ok(DataFile {
        values: values.into_iter().collect(),
        arrays_1d: arrays_1d.into_iter().collect(),
        arrays_2d: arrays_2d.into_iter().collect(),
    })
}

#[derive(Clone, Debug)]
enum ValueOrArray<Int> {
    Value(Value<Int>),
    Array1d(ValueArray<Int, 1>),
    Array2d(ValueArray<Int, 2>),
}

fn value_or_array<Int>(input: &mut &str) -> PResult<ValueOrArray<Int>>
where
    Int: FromStr + Debug + Hash + Eq,
    Int::Err: Debug,
{
    trace(
        "value_or_array",
        alt((
            value.map(ValueOrArray::Value),
            array_1d.map(ValueOrArray::Array1d),
            array_2d.map(ValueOrArray::Array2d),
        )),
    )
    .parse_next(input)
}

fn array_2d<Int>(input: &mut &str) -> PResult<ValueArray<Int, 2>>
where
    Int: FromStr + Debug,
    Int::Err: Debug,
{
    let bool_value_array = value_array_2d("bool", bool_parser).map(ValueArray::Bool);
    let int_value_array = value_array_2d("int", int_parser).map(ValueArray::Int);

    let array = alt((bool_value_array, int_value_array)).parse_next(input)?;

    Ok(array)
}

fn value_array_2d<'src, Output, Error>(
    value_name: &str,
    value_parser: impl Parser<&'src str, Output, Error> + Copy,
) -> impl Parser<&'src str, ShapedArray<Output, 2>, Error>
where
    Error: ParserError<&'src str>,
    Output: Debug,
{
    let row_separator = (multispace0, "|", multispace0);

    trace(
        format!("array2d_{value_name}"),
        delimited(
            ("[|", multispace0),
            separated(.., value_list(value_parser), row_separator),
            (multispace0, "]"),
        )
        .verify_map(|mut elements: Vec<Vec<Output>>| {
            // The last row is empty. This is because the 2d array ends with '|]', and the parser
            // thinks the '|' starts a new row. We therefore remove it.
            elements.swap_remove(elements.len() - 1);

            let d1 = elements.len();
            let d2 = elements.first().map(|row| row.len()).unwrap_or_default();

            if elements.iter().any(|row| row.len() != d2) {
                // Not all rows in the 2d matrix are equal length.
                return None;
            }

            let elements = elements.into_iter().flatten().collect();

            Some(ShapedArray {
                shape: [d1, d2],
                elements,
            })
        }),
    )
}

fn array_1d<Int>(input: &mut &str) -> PResult<ValueArray<Int, 1>>
where
    Int: FromStr + Hash + Eq,
    Int::Err: Debug,
{
    let bool_value_array = value_array_1d("bool", bool_parser).map(ValueArray::Bool);
    let int_value_array = value_array_1d("int", int_parser).map(ValueArray::Int);
    let set_int_value_array =
        value_array_1d("set of int", value_set("int", int_parser::<Int>)).map(ValueArray::SetOfInt);

    let array = alt((bool_value_array, int_value_array, set_int_value_array)).parse_next(input)?;

    Ok(array)
}

fn value_array_1d<'src, Output, Error>(
    value_name: &str,
    value_parser: impl Parser<&'src str, Output, Error>,
) -> impl Parser<&'src str, ShapedArray<Output, 1>, Error>
where
    Error: ParserError<&'src str>,
{
    trace(
        format!("array1d_{value_name}"),
        delimited("[", value_list(value_parser), "]"),
    )
    .map(|elements| ShapedArray {
        shape: [elements.len()],
        elements,
    })
}

fn value_set<'src, Output, Error>(
    value_name: &str,
    value_parser: impl Parser<&'src str, Output, Error>,
) -> impl Parser<&'src str, HashSet<Output>, Error>
where
    Error: ParserError<&'src str>,
    Output: Hash + Eq,
{
    trace(
        format!("set_{value_name}"),
        delimited(
            (multispace0, "{", multispace0),
            value_list(value_parser),
            (multispace0, "}", multispace0),
        ),
    )
    .map(|elements| elements.into_iter().collect())
}

fn value_list<'src, Output, Error>(
    value_parser: impl Parser<&'src str, Output, Error>,
) -> impl Parser<&'src str, Vec<Output>, Error>
where
    Error: ParserError<&'src str>,
{
    let separator = trace("array_separator", (",", multispace0));

    trace("value_list", separated(.., value_parser, separator))
}

fn value<Int>(input: &mut &str) -> PResult<Value<Int>>
where
    Int: FromStr + Hash + Eq,
    Int::Err: Debug,
{
    trace(
        "value",
        alt((
            bool_parser.map(Value::Bool),
            int_parser.map(Value::Int),
            value_set("int", int_parser).map(Value::SetOfInt),
        )),
    )
    .parse_next(input)
}

fn bool_parser(input: &mut &str) -> PResult<bool> {
    trace(
        "bool_value",
        delimited(
            multispace0,
            alt(("true".map(|_| true), "false".map(|_| false))),
            multispace0,
        ),
    )
    .parse_next(input)
}

fn int_parser<Int>(input: &mut &str) -> PResult<Int>
where
    Int: FromStr,
    Int::Err: Debug,
{
    trace(
        "int_value",
        delimited(
            multispace0,
            take_while(1.., |c: char| c.is_numeric()),
            multispace0,
        ),
    )
    .parse_next(input)
    .map(|s| Int::from_str(s).expect("The given integer type accepts valid integer inputs"))
}

fn identifier(input: &mut &str) -> PResult<String> {
    let ident_start = take_while(1, ('a'..='z', 'A'..='Z', '_'));
    let ident_rest = take_while(.., ('a'..='z', 'A'..='Z', '0'..='9', '_'));

    trace("identifier", (ident_start, ident_rest))
        .parse_next(input)
        .map(|(start, end)| format!("{start}{end}"))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, hash::Hash};

    use proptest::{proptest, strategy::Strategy};

    use super::*;

    fn ident() -> impl Strategy<Value = String> {
        proptest::string::string_regex("[A-Za-z][A-Za-z0-9_]*").expect("valid regex")
    }

    proptest! {
        #[test]
        fn test_ident(variable_name in ident()) {
            let mut rest = variable_name.as_str();
            let result = identifier(&mut rest);

            assert_eq!(Ok(variable_name.as_str()), result.as_deref());
            assert_eq!("", rest);
        }
    }

    #[test]
    fn test_integers_are_parsed() {
        let source = r#"
        x1 = 5;
        x2 = 6;
        "#;

        let data_file = parse::<i32>(source.as_bytes()).expect("valid dzn");
        assert_eq!(Some(6), data_file.get::<i32>("x2").copied());
    }

    #[test]
    fn test_whitespace_around_array_is_handled() {
        let source = r#"
        x1 = [ 1, 22, 3 ];
        "#;

        let data_file = parse::<i32>(source.as_bytes()).expect("valid dzn");
        let array = data_file
            .array_1d::<i32>("x1", 3)
            .expect("array with key exists");

        assert_eq!([3], *array.shape());

        for (idx, value) in [1, 22, 3].into_iter().enumerate() {
            assert_eq!(value, array.get([idx]).copied().unwrap());
        }
    }

    #[test]
    fn test_set_of_int_parsed() {
        let source = r#"
        set = {1, 2, 3};
        set2 = { 5, 6 };
        set3 = [ {1}, { 5, 6 } ];
        "#;

        let data_file = parse::<i32>(source.as_bytes()).expect("valid dzn");

        let set = data_file.get::<HashSet<i32>>("set").expect("key exists");
        let set2 = data_file.get::<HashSet<i32>>("set2").expect("key exists");

        assert_eq!(&make_set([1, 2, 3]), set);
        assert_eq!(&make_set([5, 6]), set2);

        let set3 = data_file
            .array_1d::<HashSet<i32>>("set3", 2)
            .expect("array with key exists");

        for (idx, value) in [make_set([1]), make_set([5, 6])].into_iter().enumerate() {
            assert_eq!(&value, set3.get([idx]).unwrap());
        }
    }

    #[test]
    fn test_empty_set() {
        let source = r#"
        x1 = [ {}, { }, {  } ];
        "#;

        let data_file = parse::<i32>(source.as_bytes()).expect("valid dzn");
        let array = data_file
            .array_1d::<HashSet<i32>>("x1", 3)
            .expect("array with key exists");

        assert_eq!([3], *array.shape());

        for idx in 0..3 {
            assert!(array.get([idx]).unwrap().is_empty());
        }
    }

    fn make_set<T: Hash + Eq>(data: impl IntoIterator<Item = T>) -> HashSet<T> {
        data.into_iter().collect()
    }
}
