use std::{collections::HashMap, marker::PhantomData};

use crate::{
    numbers::Integer,
    value::{GetValue, ShapedArray, Value, ValueArray},
};

/// The top-level structure which represents a data file.
///
/// A data file is a key-value store, where the key is a MiniZinc identifier, and the value is one
/// of:
/// - `int`
/// - `bool`
/// - `array of` one of the above
///
/// Conceptually, the integers in the MiniZinc specification are unbounded, which means the scalar
/// signed integers not model the DZN integers well. However, from a practical standpoint, many
/// uses of DZN files do only deal with [`i32`] or others. Therefore, [`DataFile`] is generic over
/// the integer type to allow the user to decide how big the integers can be.
#[derive(Clone, Debug, Default)]
pub struct DataFile<Int> {
    pub(crate) values: HashMap<String, Value<Int>>,
    pub(crate) arrays_1d: HashMap<String, ValueArray<Int, 1>>,
    pub(crate) arrays_2d: HashMap<String, ValueArray<Int, 2>>,
}

impl<Int: Integer> DataFile<Int> {
    /// Get a value from the data file with the given `key`.
    ///
    /// When attempting to get a specific type, this method does not discriminate to the key not
    /// existing at all, or whether the value is a different type. In either situation, [`None`] is
    /// returned.
    pub fn get<T>(&self, key: &str) -> Option<&T>
    where
        Value<Int>: GetValue<T>,
    {
        self.values.get(key).and_then(|value| value.try_get())
    }

    /// Get a 1-dimensional array from the data file with the given `key` and `length`.
    pub fn array_1d<T>(&self, key: &str, length: usize) -> Option<&ShapedArray<T, 1>>
    where
        ValueArray<Int, 1>: GetValue<ShapedArray<T, 1>>,
    {
        self.arrays_1d
            .get(key)
            .and_then(|array| array.try_get())
            .filter(move |&array| array.shape == [length])
    }

    /// Get a 2-dimensional array from the data file with the given `key`.
    ///
    /// The array shape should match `shape`.
    pub fn array_2d<T>(&self, key: &str, shape: [usize; 2]) -> Option<&ShapedArray<T, 2>>
    where
        ValueArray<Int, 2>: GetValue<ShapedArray<T, 2>>,
    {
        self.arrays_2d
            .get(key)
            .and_then(|array| array.try_get())
            .filter(move |&array| array.shape == shape)
    }
}

pub struct Array<'a, T> {
    a: PhantomData<&'a ()>,
    b: PhantomData<T>,
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
                values: values.iter().map(|(k, v)| (k.clone(), Value::Int(*v))).collect(),
                ..Default::default()
            };

            for (key, value) in values.iter() {
                assert_eq!(Some(value), data_file.get::<i32>(key.as_str()));
            }
        }
    }

    proptest! {
        #[test]
        fn nonexistent_integer_values_return_none(label in ident()) {
            let data_file: DataFile<i32> = DataFile { values: [].into(), ..Default::default() };

            assert_eq!(None, data_file.get::<i32>(&label));
        }
    }
}
