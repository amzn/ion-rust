//! This module provides the necessary structures and logic to read values from a binary Ion
//! data stream.

pub(crate) mod constants;
pub(crate) mod cursor;
mod header;
mod int;
mod nibbles;
mod type_code;
mod uint;
mod var_int;
mod var_uint;
pub mod writer;

pub(crate) use type_code::IonTypeCode;
