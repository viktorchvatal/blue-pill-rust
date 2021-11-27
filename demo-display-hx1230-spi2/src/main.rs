#![no_std]
#![no_main]

use display::{init_display, render_display};
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_hal::blocking::spi;

use cortex_m_rt::entry;
use lib_display_buffer::{ArrayDisplayBuffer, DisplayBuffer, draw};
use stm32f1xx_hal::delay::Delay;
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use lib_common::ResultExt;
use lib_display_hx1230::{SpiHx1230Driver, command as lcd_command};
use lib_panic_led as _;

mod display;

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
        .use_hse(8.mhz())  // use external oscillator (8 MHz)
        .sysclk(72.mhz())  // system clock, PLL multiplier should be 6
        .hclk(8.mhz())     // clock used for timers
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

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
    let mut frame_buffer: ArrayDisplayBuffer<96, 9> = ArrayDisplayBuffer::new();

    init_display(&mut spi, &mut display_cs, &mut delay).check();

    loop {
        led.set_high();
        draw::clear_pattern(&mut frame_buffer, &TRIANGLE);
        render_display(&mut spi, &mut display_cs, &frame_buffer).check();
        delay.delay_ms(300_u16);

        led.set_low();
        draw::clear_pattern(&mut frame_buffer, &WAVE);
        render_display(&mut spi, &mut display_cs, &frame_buffer).check();
        delay.delay_ms(300_u16);
    }
}

const TRIANGLE: [u8; 8] = [0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F, 0xFF];

const WAVE : [u8; 8] = [
    0b01001000,
    0b00100100,
    0b00010010,
    0b00010010,
    0b00010010,
    0b00100100,
    0b01001000,
    0b01001000,
];