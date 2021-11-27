# Panic handling

There are several possibilities how to configure panic behavior in case of some
non-recoverable error.

## Halt on panic

The simplest option is just halt the program on any panic. A `panic-halt`
library does exactly this, it ends the program in the infinite loop on the
panic.

To use `panic-halt`, set the dependency in
[Cargo.toml](../demo-blinky-standalone/Cargo.toml)

```
[dependencies]
panic-halt = "0.2.0"
```

and import the library in [main.rs](../demo-blinky-standalone/src/main.rs)

```
use panic_halt as _;
```

## Custom panic handler and panic LED

A panic handler can be set by defining a function with signature

```
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> !
```

An exclamation mark at the end indicates that the function never returns, so
it is the last code that is run after the panic.

I have connected a LED to PC14 GPIO port to clearly indicate a panic state
during development. An advantage of the LED is that it works even when
debugger is not connected.

Panic handler to set PC14 to high state is defined as

```rust
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    // Steal the peripherals even if another code acquired them before
    // No other code is run after this panic handler so there should
    // be no undefined behavior
    let dp = unsafe { pac::Peripherals::steal() };
    let mut gpiob = dp.GPIOC.split();
    let mut panic_led = gpiob.pc14.into_push_pull_output(&mut gpiob.crh);
    // Turn on the LED
    panic_led.set_high();
    // Infinite loop at the end so we never return
    loop { }
}
```

Panic LED is connected to the PC14 pin and through a 100 ohm resistor to the
ground

![Panic led connection schematic](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/panic-handling/panic-led-schematic.png)

A [demo-panic-led](../demo-panic-led/src/main.rs) can be started
using `cargo run --bin demo-panic-led`

Oh, did our program really panic?

![Panic LED ON](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/panic-handling/panic-led-on.jpg)

The LED clearly indicates a panic state, but to find out the cause, we need
to connect the debugger and enable the semihosting panic setup.

## Panic semihosting

To enable panic semihosting, add semihosting support and panic handling library
into the dependencies (or see
[demo-blinky-semihosting](../demo-blinky-semihosting/src/main.rs) how to do it)

```rust
cortex-m-semihosting = "0.3.3"
panic-semihosting = "0.5.6"

```

Use the libraries in import

```rust
use cortex_m_semihosting as sh;
use panic_semihosting as _;
```

And disable our LED panic handler

```rust
//#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    ...
```

Now it is possible to run `cargo run --bin demo-panic-led` again with
semihosting enabled

GDB should stop at the PreInit function so that we can manually
continue the program execution

```
Reading symbols from target/thumbv7m-none-eabi/debug/demo-panic-led...done.
0x08000b24 in stm32f1xx_hal::gpio::gpioc::<impl stm32f1xx_hal::gpio::GpioExt for stm32f1::stm32f103::GPIOC>::split (self=...)
    at /home/*/.cargo/git/checkouts/stm32f1xx-hal-bb9d214e810c7b47/e790b27/src/gpio.rs:375
375                         $GPIOX::enable(rcc);
semihosting is enabled
Loading section .vector_table, size 0x130 lma 0x8000000
Loading section .text, size 0x3b58 lma 0x8000130
Loading section .rodata, size 0xd58 lma 0x8003c90
Start address 0x8000130, load size 18912
Transfer rate: 39 KB/sec, 6304 bytes/write.
Single stepping until exit from function Reset,
which has no line number information.
Note: automatically using hardware breakpoints for read-only addresses.
cortex_m_rt::DefaultPreInit () at /home/*/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.7.0/src/lib.rs:942
942     pub unsafe extern "C" fn DefaultPreInit() {}
```

After hitting `c` to continue, another breakpoint will be hit

```
(gdb) c
Continuing.

Program received signal SIGTRAP, Trace/breakpoint trap.
lib::__bkpt () at asm/lib.rs:49
49      asm/lib.rs: No such file or directory.
```

Now switch to the running `openocd` session, and there should be the whole
panic message

```
panicked at 'assertion failed: rvr < (1 << 24)', /home/*/.cargo/git/checkouts/stm32f1xx-hal-bb9d214e810c7b47/e790b27/src/timer.rs:247:9
```

Timer could not be initialized, because timer clock is too high for the counter
to establish 1 Hz frequency (required value does not fit into
the 16-bit timer/counter).

Now we can fix our program by configuring the `hclk` timer to run at 8 MHz

```
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())  // use external oscillator (8 MHz)
        .sysclk(72.mhz())  // system clock, PLL multiplier should be 6
        .hclk(8.mhz())     // clock used for timers
        .freeze(&mut flash.acr);
```

and the program does not panic any more