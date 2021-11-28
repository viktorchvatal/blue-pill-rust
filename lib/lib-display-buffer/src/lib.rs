#![no_std]
#![deny(unsafe_code)]

mod buffer;
pub mod draw;

pub use buffer::{DisplayBuffer, ArrayDisplayBuffer};
