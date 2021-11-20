# Rust blue pill learning demo

## Resources

https://github.com/stm32-rs/stm32f1xx-hal

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

In case `openocd` returns an error

```
Warn : UNEXPECTED idcode: 0x2ba01477
Error: expected 1 of 1: 0x1ba01477
```

Edit `/usr/share/openocd/scripts/target/stm32f1x.cfg` and replace 

```
set _CPUTAPID 0x1ba01477
``` 

with 

```
set _CPUTAPID 0x2ba01477
```

In case openocd writes
```
Info : Previous state query failed, trying to reconnect
Error: jtag status contains invalid mode value - communication failure
Polling target stm32f1x.cpu failed, trying to reexamine
Examination failed, GDB will be halted. Polling again in 700ms
```

Start openocd with `reset` putton active

## Running

Enable loading `.gdbinit`

```
echo "set auto-load safe-path $(pwd)" >> ~/.gdbinit
```

