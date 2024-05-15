use std::marker::PhantomData;

/// The top-level structure which represents a data file.
///
/// Conceptually, the integers in the MiniZinc specification are unbounded, which means the scalar
/// signed integers not model the DZN integers well. However, from a practical standpoint, many
/// uses of DZN files do only deal with [`i32`] or others. Therefore, [`DataFile`] is generic over
/// the integer type to allow the user to decide how big the integers can be.
#[derive(Clone, Debug)]
pub struct DataFile<Int> {
    int: PhantomData<Int>,
}
