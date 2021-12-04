#![no_std]
#![deny(unsafe_code)]

pub mod command;
mod driver;
mod encode;

pub use driver::SpiHx1230Driver;