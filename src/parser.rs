use std::{fmt::Debug, io::BufRead, str::FromStr};

use winnow::{
    combinator::{alt, delimited, preceded},
    error::{ContextError, ErrMode},
    token::take_while,
    PResult, Parser,
};

use crate::{value::Value, DataFile, DznParseError, SyntaxElement};

/// Parse the data file from the given source.
pub fn parse<Int>(mut source: impl BufRead) -> Result<DataFile<Int>, DznParseError>
where
    Int: FromStr,
    Int::Err: Debug,
{
    let mut buffer = String::new();
    source.read_to_string(&mut buffer)?;

    let mut values = vec![];
    let mut rest = buffer.trim();

    while !rest.is_empty() {
        let ident = preceded(ws, identifier)
            .parse_next(&mut rest)
            .map_err(|_| DznParseError::InvalidSyntax {
                expected: SyntaxElement::Identifier,
                actual: rest.to_string(),
            })?;

        let _ =
            delimited(ws, "=", ws)
                .parse_next(&mut rest)
                .map_err(|_: ErrMode<ContextError>| DznParseError::InvalidSyntax {
                    expected: SyntaxElement::Equals,
                    actual: rest.to_string(),
                })?;

        let value = value
            .parse_next(&mut rest)
            .map_err(|_| DznParseError::InvalidSyntax {
                expected: SyntaxElement::Identifier,
                actual: rest.to_string(),
            })?;

        let _ =
            delimited(ws, ";", ws)
                .parse_next(&mut rest)
                .map_err(|_: ErrMode<ContextError>| DznParseError::InvalidSyntax {
                    expected: SyntaxElement::SemiColon,
                    actual: rest.to_string(),
                })?;

        values.push((ident, value));
    }

    Ok(DataFile {
        values: values.into_iter().collect(),
    })
}

fn value<Int>(input: &mut &str) -> PResult<Value<Int>>
where
    Int: FromStr,
    Int::Err: Debug,
{
    alt((bool_parser.map(Value::Bool), int.map(Value::Int))).parse_next(input)
}

fn bool_parser(input: &mut &str) -> PResult<bool> {
    alt(("true".map(|_| true), "false".map(|_| false))).parse_next(input)
}

fn int<Int>(input: &mut &str) -> PResult<Int>
where
    Int: FromStr,
    Int::Err: Debug,
{
    take_while(1.., |c: char| c.is_numeric())
        .parse_next(input)
        .map(|s| Int::from_str(s).expect("The given integer type accepts valid integer inputs"))
}

fn identifier(input: &mut &str) -> PResult<String> {
    let ident_start = take_while(1, ('a'..='z', 'A'..='Z', '_'));
    let ident_rest = take_while(.., ('a'..='z', 'A'..='Z', '0'..='9', '_'));

    (ident_start, ident_rest)
        .parse_next(input)
        .map(|(start, end)| format!("{start}{end}"))
}

fn ws<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(0.., |c: char| c.is_ascii_whitespace()).parse_next(input)
}

#[cfg(test)]
mod tests {
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
}
