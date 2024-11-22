use crate::numbers::Integer;

#[derive(Clone, Debug)]
pub enum Value<Int> {
    Bool(bool),
    Int(Int),
}

pub trait GetValue<T> {
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

#[derive(Clone, Debug)]
pub enum ValueArray<Int, const DIM: usize> {
    Bool(ShapedArray<bool, DIM>),
    Int(ShapedArray<Int, DIM>),
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

#[derive(Clone, Debug)]
pub struct ShapedArray<T, const DIM: usize> {
    pub(crate) shape: [usize; DIM],
    pub(crate) elements: Vec<T>,
}

impl<T, const D: usize> ShapedArray<T, D> {
    pub fn shape(&self) -> &[usize; D] {
        &self.shape
    }

    /// Get a value from the array with the given path.
    ///
    /// The path is an iterator of indices into each of the dimensions of the array.
    pub fn get(&self, path: [usize; D]) -> Option<&T> {
        let mut strides = vec![1]; // Start with the last dimension having stride 1
        let mut total_stride = 1;

        // Compute strides in reverse order (cumulative product of shape lengths)
        for &dim_size in self.shape.iter().rev().skip(1) {
            total_stride *= dim_size;
            strides.push(total_stride);
        }

        strides.reverse();

        // Compute the flattened index by multiplying each index by its corresponding stride
        let flattened_index: usize = path
            .iter()
            .zip(strides.iter())
            .map(|(i, stride)| i * stride)
            .sum();

        self.elements.get(flattened_index)
    }
}
