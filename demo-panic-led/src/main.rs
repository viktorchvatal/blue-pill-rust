#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())  // use external oscillator (8 MHz)
        .sysclk(72.mhz())  // system clock, PLL multiplier should be 6
        .freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high();

        block!(timer.wait()).unwrap();
        led.set_low();
    }
}

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