use std::io::Read;

use crate::{DataFile, DznParseError};

/// Parse the data file from the given source.
pub fn parse<Int>(_source: impl Read) -> Result<DataFile<Int>, DznParseError> {
    todo!()
}
