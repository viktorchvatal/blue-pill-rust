#![no_std]

use core::panic::PanicInfo;
use stm32f1xx_hal::{pac, prelude::*};

#[inline(never)]
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    // Steal the peripherals even if another code acquired them before
    // No other code is run after this panic handler so there should
    // be no undefined behavior
    let dp = unsafe { pac::Peripherals::steal() };
    let mut gpiob = dp.GPIOB.split();
    let mut panic_led = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    // Turn on the LED
    panic_led.set_high();
    // Infinite loop at the end so we never return
    loop { }
}