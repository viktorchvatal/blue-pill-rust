//! Blinks an LED and outputs ON and OFF debug messages via semihosting i/o
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.
//!
//! Original source: https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/blinky.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embedded_hal::spi::{Mode, Phase, Polarity};

use cortex_m_rt::entry;
use stm32f1xx_hal::delay::Delay;
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use lib_common::MiniResultExt;
use lib_display_hx1230::{Driver as LcdDriver, Command as LcdCommand};

pub const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();

    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())  // use external oscillator (8 MHz)
        .sysclk(72.mhz())  // system clock, PLL multiplier should be 6
        .hclk(8.mhz())     // clock used for timers
        .freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut display_cs = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // SPI2, we use only output, so there is no miso input
    let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);

    let mut spi = Spi::spi2(
        dp.SPI2,
        (sck, NoMiso, mosi),
        SPI_MODE,
        4.mhz(),
        clocks,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut display = LcdDriver::new(&mut spi, &mut display_cs);
    display.command(LcdCommand::reset()).check();
    delay.delay_us(100_u16);
    display.init_sequence().check();

    loop {
        led.set_high();
        display.command(LcdCommand::invert_on()).check();
        delay.delay_ms(30_u16);

        led.set_low();
        display.command(LcdCommand::invert_off()).check();
        delay.delay_ms(30_u16);
    }
}

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    let dp = unsafe { pac::Peripherals::steal() };
    let mut gpiob = dp.GPIOB.split();
    let mut panic_led = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);
    panic_led.set_high();
    loop { }
}