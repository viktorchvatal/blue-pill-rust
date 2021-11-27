# Getting started

## Hardware

Development board (a cheap blue pill clone with STM32F030C8)

![blue pill photo](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/intro/intro-blue-pill.jpg)

https://www.laskarduino.cz/arm-stm32-stm32f030c8-vyvojova-deska/

Programming interface: ST-Link V2

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/intro/intro-stlinkv2.jpg)

## Dependencies

Install Debian 11 packages needed for development

```
sudo apt install openocd gdb-multiarch
```

Install rust if not present

```
https://www.rust-lang.org/tools/install
```

Install Rust target

```
rustup target install thumbv7m-none-eabi
```

## Building

```
cargo build
```

## Starting the debugger

Connet the ST Link debugger to the blue pill development board and
to the computer using USB port. Pins used are as follows:

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/intro/stlink-pinout.jpg)

Warning: always check the pin out - there are many cheap ST Link
debuggers out there and even if they look the same, connertor pin signals
may be absolutely different

![STLink different pinouts](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/intro/stlink-different-pinouts.jpg)

My chosen colors for signals are:

 - green: GND
 - yellow: SWCLK
 - orange: SWDIO
 - red: 3.3V

Open the debugger using STlink v 2

```
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
```

Note: on older debian 10 system, use `interface/stlink-v2.cfg`

If MCU has been detected successfully, openocd should print
`Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints`.
Now it is ready to accept GDB connection.

In case `openocd` returns an error

```
Warn : UNEXPECTED idcode: 0x2ba01477
Error: expected 1 of 1: 0x1ba01477
```

That means that you got one of those chinese STM32F030C8 clones instead of
a genuine STM microcontroller. It should still work fine, but you need to edit
`/usr/share/openocd/scripts/target/stm32f1x.cfg` and replace
`set _CPUTAPID 0x1ba01477` with `set _CPUTAPID 0x2ba01477`

In case `openocd` prints
```
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f1x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 700ms
```

Start openocd with `reset` button active

## Cargo and linker configuration

Source codes already contain following files needed to compile and run the project

[memory.x](../memory.x) - defines microcontroller Flash and RAM size

```
/* Linker script for the STM32F103C8T6 */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
```

[.cargo/config](../.cargo/config) - tells Cargo to build for
`thumbv7m-none-eabi` architecture and use `gdb-multiarch` for debugging

```
[target.thumbv7m-none-eabi]
runner = 'gdb-multiarch'
rustflags = [
  "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7m-none-eabi"
```

## Running

Enable loading `.gdbinit`

```
echo "set auto-load safe-path $(pwd)" >> ~/.gdbinit
```

Running the program - blinking demo with semihosting debug output, but would not
run without the debugger

```
cargo run --bin demo-blinky-semihosting
```
Green LED should start flashing, 1 second ON, 1 second OFF

![Blinking LED](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/intro/blinking.gif)

openocd console should show the following output at the same time:

```
OFF
ON
OFF
ON
OFF
ON
OFF
```

There is also standalone blinking demo that runs without active debug
session, but does not have any semihosting output

```
cargo run --bin demo-blinky-standalone
```

## Flashing

Install cargo flash program

```
cargo install cargo-flash
```

Note: When you see `Unable to find libusb-1.0:`, you may need to install
libusb 1.0 development package as well: `sudo apt install libusb-1.0-0-dev`

Flash the program

```
cd demo-blinky-standalone
cargo flash --chip stm32f103C8 --release
```

Note: If you see `Error Failed to open the debug probe`, you have to close
the running openocd session first.

## Semihosting

Dependencies `cortex-m-semihosting` and `panic-semihosting` enable sending any
debug prints and panic assertions into openocd console via STLink

Use either `use panic_semihosting as _;` or `use panic_halt as _;`