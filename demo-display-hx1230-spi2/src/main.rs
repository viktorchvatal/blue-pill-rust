#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_hal::blocking::spi;

use cortex_m_rt::entry;
use lib_display_buffer::{ArrayDisplayBuffer, DisplayBuffer, draw};
use stm32f1xx_hal::delay::Delay;
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use lib_common::ResultExt;
use lib_display_hx1230::{SpiDriver as LcdDriver, command as lcd_command};

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

    let mut display = LcdDriver::new(&mut spi, &mut display_cs);
    display.command(lcd_command::reset()).check();
    delay.delay_us(100_u16);
    display.init_sequence().check();
    display.clear_data().check();

    let mut frame_buffer: ArrayDisplayBuffer<96, 9> = ArrayDisplayBuffer::new();

    loop {
        led.set_high();
        draw::clear_pattern(&mut frame_buffer, &TRIANGLE);
        draw_display_buffer(&mut frame_buffer, &mut display);
        delay.delay_ms(300_u16);

        led.set_low();
        draw::clear_pattern(&mut frame_buffer, &WAVE);
        draw_display_buffer(&mut frame_buffer, &mut display);
        delay.delay_ms(300_u16);
    }
}

fn draw_display_buffer<SPI, CS>(
    input: &dyn DisplayBuffer,
    driver: &mut LcdDriver<SPI, CS>,
) where SPI: spi::Write<u8>, CS: OutputPin {
    driver.reset_position().check();

    for line_id in 0..input.line_count() {
        if let Some(ref line) = input.get_line(line_id) {
            driver.send_data(line).check();
        }
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