# Rust blue pill learning demo

My personal walk through learning Rust development on STM32 family of microcontrollers, using:

 - Blue Pill development board with STM32F030C8 microcontroller as target device
 - STLink v2 as a programming and debugging interface
 - Debian 11 bullseye and Visual studio code as development environment

## Userful Resources

Following links contain a lot of useful resources about STM32 platform and Rust

 - https://github.com/stm32-rs/stm32f1xx-hal
 - https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/

## Getting Started

[Getting started](doc/getting_started.md) - learn how to install
development tools and flash the first program

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/intro/blinking.gif)

## Panic Handling and Panic LED via GPIO

[Panic handling and panic LED](doc/panic_handling.md) - handling panics
using halt, panic LED and semihosting output

![Panic LED ON](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/panic-handling/panic-led-on-small.jpg)

## Other

[Notes](doc/notes.md)