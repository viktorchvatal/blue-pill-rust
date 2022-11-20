#![no_std]
#![no_main]

use cortex_m_rt::entry;
use max7219::MAX7219;
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};
use lib_panic_led as _;
use embedded_hal::{
    spi::{Mode, Phase, Polarity},
};

pub const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

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
        .use_hse(8.MHz())  // use external oscillator (8 MHz)
        .sysclk(72.MHz())  // system clock, PLL multiplier should be 6
        .hclk(8.MHz())     // clock used for timers
        .freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // SPI2, we use only output, so there is no miso input
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let mut delay = cp.SYST.delay(&clocks);

    let spi = Spi::spi1(dp.SPI1, (sck, NoMiso, mosi), &mut afio.mapr, SPI_MODE, 1.MHz(), clocks);
    let mut display = MAX7219::from_spi(1, spi).unwrap();

    let mut counter: usize = 1;

    loop {
        led.set_low();
        display.power_on().unwrap();
        display.write_str(0, b"Hello   ", 0xff).unwrap();
        display.set_intensity(0, 0x0f).unwrap();
        counter += 1;
        led.set_high();

        delay.delay_ms(100_u16);
    }
}
