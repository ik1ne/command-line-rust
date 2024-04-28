#![allow(clippy::single_range_in_vec_init)]
mod bytes;
mod chars;
mod fields;

pub use bytes::extract_bytes;
pub use chars::extract_chars;
pub use fields::extract_fields;
