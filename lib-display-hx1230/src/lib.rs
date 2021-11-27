#![no_std]
#![deny(unsafe_code)]

pub mod command;
mod driver;

pub use driver::SpiHx1230Driver;