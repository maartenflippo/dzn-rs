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
//!             | false, true
//!             | false, false |];
//! "#;
//!
//! let data_file = dzn_rs::parse::<i32>(source.as_bytes()).expect("valid dzn");
//!
//! assert_eq!(Some(&5), data_file.get::<i32>("int_param"));
//! assert_eq!(Some(&true), data_file.get::<bool>("bool_param"));
//!
//! let array_1d = data_file.array_1d::<i32>("array_1d", 3)
//!     .expect("key exists with requested length");
//! assert_eq!(&[3], array_1d.shape());
//! for (idx, value) in [1, 3, 5].iter().enumerate() {
//!     assert_eq!(Some(value), array_1d.get([idx]));
//! }
//!
//! let array_2d = data_file.array_2d::<bool>("array_2d", [3, 2])
//!     .expect("key exists with requested shape");
//! assert_eq!(&[3, 2], array_2d.shape());
//!
//! dbg!(&array_2d);
//! assert_eq!(Some(&true), array_2d.get([0, 0]));
//! assert_eq!(Some(&false), array_2d.get([0, 1]));
//! assert_eq!(Some(&false), array_2d.get([1, 0]));
//! assert_eq!(Some(&true), array_2d.get([1, 1]));
//! assert_eq!(Some(&false), array_2d.get([2, 0]));
//! assert_eq!(Some(&false), array_2d.get([2, 1]));
//! ```

mod ast;
mod error;
mod numbers;
mod parser;
mod value;

pub use ast::*;
pub use error::*;
pub use parser::*;
pub use value::*;
