use std::collections::HashSet;

use crate::numbers::Integer;

/// A primitive MiniZinc value.
#[derive(Clone, Debug)]
pub enum Value<Int> {
    Bool(bool),
    Int(Int),
    SetOfInt(HashSet<Int>),
}

/// Helper trait to extract values from enums.
pub trait GetValue<T> {
    /// If the enum is `T`, then return the value. Otherwise, return `None`.
    fn try_get(&self) -> Option<&T>;
}

impl<Int> GetValue<bool> for Value<Int> {
    fn try_get(&self) -> Option<&bool> {
        match self {
            Value::Bool(boolean) => Some(boolean),
            _ => None,
        }
    }
}

impl<Int: Integer> GetValue<Int> for Value<Int> {
    fn try_get(&self) -> Option<&Int> {
        match self {
            Value::Int(int) => Some(int),
            _ => None,
        }
    }
}

impl<Int: Integer> GetValue<HashSet<Int>> for Value<Int> {
    fn try_get(&self) -> Option<&HashSet<Int>> {
        match self {
            Value::SetOfInt(set) => Some(set),
            _ => None,
        }
    }
}

/// The possible arrays of values.
///
/// `DIM` is either 1 or 2, depending on whether we have a 1 dimensional or 2-dimensional array.
#[derive(Clone, Debug)]
pub enum ValueArray<Int, const DIM: usize> {
    Bool(ShapedArray<bool, DIM>),
    Int(ShapedArray<Int, DIM>),
    SetOfInt(ShapedArray<HashSet<Int>, DIM>),
}

impl<Int: Integer, const DIM: usize> GetValue<ShapedArray<bool, DIM>> for ValueArray<Int, DIM> {
    fn try_get(&self) -> Option<&ShapedArray<bool, DIM>> {
        match self {
            ValueArray::Bool(array) => Some(array),
            _ => None,
        }
    }
}

impl<Int, const DIM: usize> GetValue<ShapedArray<Int, DIM>> for ValueArray<Int, DIM> {
    fn try_get(&self) -> Option<&ShapedArray<Int, DIM>> {
        match self {
            ValueArray::Int(array) => Some(array),
            _ => None,
        }
    }
}

impl<Int, const DIM: usize> GetValue<ShapedArray<HashSet<Int>, DIM>> for ValueArray<Int, DIM> {
    fn try_get(&self) -> Option<&ShapedArray<HashSet<Int>, DIM>> {
        match self {
            ValueArray::SetOfInt(set) => Some(set),
            _ => None,
        }
    }
}

/// 1d or 2d array of values.
#[derive(Clone, Debug)]
pub struct ShapedArray<T, const DIM: usize> {
    pub(crate) shape: [usize; DIM],
    pub(crate) elements: Vec<T>,
}

impl<T, const D: usize> ShapedArray<T, D> {
    /// Get the shape of the multi-dimensional array.
    pub fn shape(&self) -> &[usize; D] {
        &self.shape
    }

    /// Get a value from the array with the given path.
    ///
    /// The path is an iterator of indices into each of the dimensions of the array.
    pub fn get(&self, path: [usize; D]) -> Option<&T> {
        let index = match D {
            1 => path[0],
            2 => path[0] * self.shape[1] + path[1],
            _ => panic!("Arrays with higher than 2 dimensions are not supported"),
        };

        self.elements.get(index)
    }
}
