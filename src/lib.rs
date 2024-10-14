//! A DZN file is represented in the [`DataFile`] struct. It can be parsed from a source with the
//! [`parse`] function.
//!
//! # Example
//! ```
//! let source = r#"
//! int_param = 5;
//! bool_param = true;
//! array_1d = [1, 3, 5];
//! array_2d = [| true, false
//!             | false, true |];
//! "#;
//!
//! let data_file = dzn_rs::parse::<i32>(source.as_bytes()).expect("valid dzn");
//!
//! assert_eq!(Some(&5), data_file.get::<i32>("int_param"));
//! assert_eq!(Some(&true), data_file.get::<bool>("bool_param"));
//! ```

mod ast;
mod error;
mod numbers;
mod parser;
mod value;

pub use ast::*;
pub use error::*;
pub use parser::*;
