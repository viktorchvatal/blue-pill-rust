#![no_std]
#![deny(unsafe_code)]

pub mod command;
mod driver;
mod encode;
mod interface;

pub use interface::Hx1230Driver;
pub use driver::SpiHx1230Driver;