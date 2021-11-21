# Getting started

## Hardware

Development board (a cheap blue pill clone with STM32F030C8)

https://www.laskarduino.cz/arm-stm32-stm32f030c8-vyvojova-deska/

Programming interface: ST-Link V2 

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

## Flashing

Flash the program using STlink v 2

```
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
```

If MCU has been detected successfully, openocd should print `Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints`. Now it is ready to accept GDB connection.

In case `openocd` returns an error

```
Warn : UNEXPECTED idcode: 0x2ba01477
Error: expected 1 of 1: 0x1ba01477
```

Edit `/usr/share/openocd/scripts/target/stm32f1x.cfg` and replace `set _CPUTAPID 0x1ba01477` with `set _CPUTAPID 0x2ba01477`

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

[.cargo/config](../.cargo/config) - tells Cargo to build for `thumbv7m-none-eabi` architecture and use `gdb-multiarch` for debugging

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

Running the program

```
cargo run demo-blinky
```

## Semihosting

Dependencies `cortex-m-semihosting` and `panic-semihosting` enable sending any
debug prints and panic assertions into openocd console via STLink

Use either `use panic_semihosting as _;` or `use panic_halt as _;`