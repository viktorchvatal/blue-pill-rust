#![no_std]
#![no_main]

use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_hal::blocking::{delay::DelayUs};

use cortex_m_rt::entry;
use lib_display_hx1230::command::set_position;
use lib_display_hx1230::{SpiHx1230Driver, Hx1230Driver, command};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use lib_common::ResultExt;
use lib_panic_led as _;

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

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())  // use external oscillator (8 MHz)
        .sysclk(72.MHz())  // system clock, PLL multiplier should be 6
        .hclk(8.MHz())     // clock used for timers
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
        4.MHz(),
        clocks,
    );

    let mut delay = cp.SYST.delay(&clocks);

    let mut display = SpiHx1230Driver::new(&mut spi, &mut display_cs);
    display.send_commands(&[command::reset()]).check();
    delay.delay_us(100_u16);
    display.send_commands(command::init_sequence()).check();
    display.clear_data().check();

    let mut phase = 0;

    loop {
        led.set_low();
        display.send_commands(&set_position(0, 0)).check();

        for line in 0..9 {
            let shift = if line % 2 == 1 { line + phase } else { line + 8 - phase };
            draw_wave_line(shift, &mut display).check();
        }

        led.set_high();
        phase = (phase + 1) % 8;
        delay.delay_ms(30_u16);
    }
}

fn draw_wave_line(phase: usize, display: &mut dyn Hx1230Driver) -> Result<(), ()> {
    let mut data = [0u8; 96];

    for index in 0..96 {
        data[index] = WAVE[(index + phase) % WAVE.len()];
    }

    display.send_data(&data)
}

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