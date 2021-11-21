//! Blinks an LED and outputs ON and OFF debug messages via semihosting i/o
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.
//!
//! Original source: https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/blinky.rs

#![deny(unsafe_code)]
#![no_std]
#![no_main]

mod hx1230_driver;
mod hx1230_sw_driver;

use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_hal::blocking::spi;

use hx1230_sw_driver::{Hx1230SwDriver, SwSpi};
use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting as sh;
use stm32f1xx_hal::delay::Delay;
use stm32f1xx_hal::gpio::{Alternate, CRH, Output, Pin, PushPull};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use hx1230_driver::Hx1230Driver;

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
    let mut display_reset = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // SPI2, we use only output, so there is no miso input
    let sck = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let mosi = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut sw_spi = DisplaySpi {
        mosi,
        sck,
        reset: display_reset,
        delay,
        delay_us: 1,
        delay_init_us: 100,
    };

    let display = Hx1230SwDriver::new();

    display.init(&mut sw_spi);

    loop {
        led.set_low();

        display.init(&mut sw_spi);

        display.set_display_test(&mut sw_spi, true);
        // sw_spi.delay.delay_ms(100_u16);

        led.set_high();

        display.init(&mut sw_spi);

        display.set_display_test(&mut sw_spi, true);
        // sw_spi.delay.delay_ms(100_u16);
    }
}

struct DisplaySpi {
    sck: Pin<Output<PushPull>, CRH, 'B', 13>,
    mosi: Pin<Output<PushPull>, CRH, 'B', 15>,
    reset: Pin<Output<PushPull>, CRH, 'B', 12>,
    delay: Delay,
    delay_us: u16,
    delay_init_us: u16,
}

impl SwSpi for DisplaySpi {
    fn hw_reset(&mut self) {
        self.reset.set_low();
        self.delay.delay_us(self.delay_init_us);
        self.reset.set_high();
    }

    fn mosi_set_high(&mut self) {
        self.mosi.set_high();
    }

    fn mosi_set_low(&mut self) {
        self.mosi.set_low();
    }

    fn sck_toggle_high_low(&mut self) {
        self.delay.delay_us(self.delay_us);
        self.sck.set_high();
        self.delay.delay_us(self.delay_us);
        self.sck.set_low();
        self.delay.delay_us(self.delay_us);
    }

    fn long_init_delay(&mut self) {
        self.delay.delay_us(self.delay_init_us);
    }
}