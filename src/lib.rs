//! A DZN file is represented in the [`DataFile`] struct. It can be parsed from a source with the
//! [`parse`] function.

mod ast;
mod error;
mod parser;

pub use ast::*;
pub use error::*;
pub use parser::*;
