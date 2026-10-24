#![no_std]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

//! # Annotated characters
//!
//! ```
//! use anch::{Annotated, AnnotatedChar, LineEnding};
//!
//! let mut chars = "Lorem\r\nipsum".chars().annotated();
//! assert_eq!(AnnotatedChar::Character('L', 1, 1), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('o', 1, 2), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('r', 1, 3), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('e', 1, 4), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('m', 1, 5), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::LineEnding(LineEnding::CrLf, 1, 6), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('i', 2, 1), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('p', 2, 2), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('s', 2, 3), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('u', 2, 4), chars.next().unwrap());
//! assert_eq!(AnnotatedChar::Character('m', 2, 5), chars.next().unwrap());
//! ```

mod annotated;
mod line_ending;

pub use annotated::{Annotated, AnnotatedChar};
pub use line_ending::LineEnding;
