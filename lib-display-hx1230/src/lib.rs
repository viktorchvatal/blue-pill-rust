#![no_std]
#![deny(unsafe_code)]

pub mod command;
mod driver;

pub use driver::SpiDriver;