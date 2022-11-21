# Rust blue pill learning demo

My personal walk through learning Rust development on STM32 family of microcontrollers, using:

 - Blue Pill development board with STM32F103C8 microcontroller as target device
 - STLink v2 as a programming and debugging interface
 - Debian 10/11 bullseye and Visual studio code as development environment
 - **Pros:** cheap board ($5), small-factor, breadboard-friendly, good Rust support
 - **Cons:** market is flooded with counterfeit chips, harder to find high quality boards

## Motivation

After years spent with hobby project build using Atmel AVR Tiny and Mega
microcontrollers using AVR assembler od C programming environments, I wanted
to enter the world of ARM microcontroller development. 32-bit ARM devices usually
provide better CPU performance (compared to 8-bit AVD devices) as well as order
of magnitude of RAM and Flash capacity, but for similar price. As a Rust
developer, I wanted a brand with good Rust community support.

I did not want to buy pricey large and well equipped development boards,
but as a hobbyist I wanted to find a small breadboard-friendly development
board that could be used in both the prototype and the final device itself.

As of 2019, best fit was the $5 Blue Pill board featuring STM32F103C8 CPU
with 20K RAM and 64K ROM. Advantages are small size, low price and great Rust
support, which made this board one of the best to start with, but low
quality and cheap counterfeit chip clones, that flooded the market make this
option less appealing, but it is still reasonable option to start with.

## Other Boards

After playing with Blue Pill, i also tried its more expensive, but much more
capable counter part Black Pill - more demos and examples for Black
Pill development board can be found in the
[black-pill-rust](https://github.com/viktorchvatal/black-pill-rust) repository

During 2020, another $5 alternative to the Blue Pill emerged - a Raspberry PI
Pico board, carrying two computing cores and tons of RAM and 2MB of onboard Flash
memory. My experiments with the RPi Pico are in the [rpi-pico-rust](https://github.com/viktorchvatal/rpi-pico-rust) repository.

Most drivers used in all the demos across different boards use platform-agnostic
device drivers written on top of `embedded_hal` crate, so than can be easily
transferred across from one board to another.

## Userful Resources

Following links contain a lot of useful resources about STM32 platform and Rust

 - https://github.com/stm32-rs/stm32f1xx-hal
 - https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/

## Getting Started

[Getting started](doc/getting_started.md) - learn how to install
development tools and flash the first program

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/intro/blinking-small.gif)

## Panic Handling and Panic LED via GPIO

[Panic handling and panic LED](doc/panic_handling.md) - handling panics
using halt, panic LED and semihosting output

![Panic LED ON](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/panic-handling/panic-led-on-small.jpg)

## PWM Channels and colored LEDS

[PWM Channels and colored LEDS](doc/pwm_channels.md) - changing brightness
of multipe LEDS using PWM channels and `micromath` fast approximation library

![PWM LEDs](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/pwm-channels/pwm-leds-small.gif)

## Connecting TM1637 LED Display

[Connecting a TM1637 LED display](doc/display_tm1637.md) - connecting a LED
display driven by TM1637 circuit using `tm1637` crate

![LED Display connected](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/display-tm1637/connected-display-small.jpg)

## HX1230 Matrix Graphical Display [in progress]

[HX1230 Graphical display demo](doc/display_hx1230.md) - communicating
with the HX1230 graphical display

![HX1230 Display](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/display-hx1230/hx1230-small.gif)

## DHT11 Temperature and humidity sensor [in progress]

[DHT11 Temperature and humidity sensor demo](doc/temperature-dht11.md) - communicating
with the HX1230 graphical display

![HX1230 Display](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/temperature-dht11/dht11-small.jpg)

## BMP280 Pressure and Temperature sensor [in progress]

[BMP280 Pressure and Temperature sensor](doc/bmp280.md) - communicating
with the HX1230 graphical display

![BMP280 Pressure and Temperature sensor](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/pressure-bmp280/bmp280-small.jpg)

## Using MAX7219 to drive 7-segment 8-character display [in progress]

[MAX7219 display](doc/display_max7219.md)

![BMP280 Pressure and Temperature sensor](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/max7219-7segment/max7219-7segment-small.gif)

## Other

[Notes](doc/notes.md)