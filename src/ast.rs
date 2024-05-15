use std::collections::HashMap;

/// The top-level structure which represents a data file.
///
/// Conceptually, the integers in the MiniZinc specification are unbounded, which means the scalar
/// signed integers not model the DZN integers well. However, from a practical standpoint, many
/// uses of DZN files do only deal with [`i32`] or others. Therefore, [`DataFile`] is generic over
/// the integer type to allow the user to decide how big the integers can be.
#[derive(Clone, Debug)]
pub struct DataFile<Int> {
    pub(crate) values: HashMap<String, Int>,
}

impl<Int> DataFile<Int>
where
    Int: Copy,
{
    pub fn get_int(&self, name: &str) -> Option<Int> {
        self.values.get(name).copied()
    }
}

#[cfg(test)]
mod tests {
    use proptest::{proptest, strategy::Strategy};

    use super::*;

    fn ident() -> impl Strategy<Value = String> {
        proptest::string::string_regex("[A-Za-z][A-Za-z0-9_]*").expect("valid regex")
    }

    fn int_map() -> impl Strategy<Value = HashMap<String, i32>> {
        proptest::collection::hash_map(ident(), proptest::prelude::any::<i32>(), 1..5)
    }

    proptest! {
        #[test]
        fn integers_are_found(values in int_map()) {
            let data_file = DataFile {
                values: values.clone(),
            };

            for (key, &value) in values.iter() {
                assert_eq!(Some(value), data_file.get_int(key));
            }
        }
    }

    proptest! {
        #[test]
        fn nonexistent_integer_values_return_none(label in ident()) {
            let data_file: DataFile<i32> = DataFile { values: [].into() };

            assert_eq!(None, data_file.get_int(&label));
        }
    }
}
