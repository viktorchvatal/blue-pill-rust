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

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/panic-handling/panic-led-schematic.png)

A [demo-panic-led](../demo-panic-led/src/main.rs) can be started
using `cargo run --bin demo-panic-led`

Oh, did our program really panic?

![stlink v2 photo](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/panic-handling/panic-led-on.jpg)

The LED clearly indicates a panic state, but to find out the cause, we need
to connect the debugger and enable the semihosting panic setup.

## Panic semihosting

