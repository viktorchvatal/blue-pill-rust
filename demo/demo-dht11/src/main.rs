#![no_std]
#![no_main]

use core::fmt::Write;
use arrayvec::ArrayString;
use dht11::Dht11;
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::{ascii::FONT_7X13, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_hal::spi::{Mode as SpiMode, Phase, Polarity};
use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use embedded_hal::blocking::delay::DelayUs;
use lib_common::MiniResult;
use hx1230::command::{init_sequence};
use hx1230::{ArrayDisplayBuffer, DisplayBuffer, SpiDriver, command, DisplayDriver};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use cortex_m_rt::entry;

use lib_common::ResultExt;
use lib_panic_led as _;

pub const SPI_MODE: SpiMode = SpiMode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    cp.DWT.enable_cycle_counter();

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

    let thermo_pin = gpiob.pb7.into_open_drain_output(&mut gpiob.crl);

    let mut delay = cp.SYST.delay(&clocks);

    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();
    init_display(&mut spi, &mut display_cs, &mut delay).check();
    let text_style = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let mut text = ArrayString::<100>::new();

    let mut dht11 = Dht11::new(thermo_pin);

    loop {
        led.set_low();
        clear(&mut frame_buffer);

        clear_line(&mut frame_buffer, 0);
        clear_line(&mut frame_buffer, 1);

        let measurement = dht11.perform_measurement(&mut delay);
        text.clear();

        match measurement {
            Err(err) => { let _ = write!(&mut text, "E:{:?}", err);},
            Ok(values) => {
                let _ = write!(
                    &mut text, "Temperature:\n{}.{} C\nHumidity:\n{}.{} %",
                    values.temperature/10, values.temperature%10,
                    values.humidity/10, values.humidity%10,
                );
            }
        }

        Text::new(&text, Point::new(0, 20), text_style).draw(&mut frame_buffer).check();
        let mut driver = SpiDriver::new(&mut spi, &mut display_cs);
        driver.send_buffer(&frame_buffer).check();

        led.set_high();

        delay.delay_ms(1000_u16);
    }
}

#[inline(never)]
pub fn init_display<SPI, CS, D>(
    spi: &mut SPI,
    cs: &mut CS,
    delay: &mut D,
) -> MiniResult
where SPI: spi::Write<u8>, CS: OutputPin, D: DelayUs<u16> {
    let mut display = SpiDriver::new(spi, cs);
    display.send_commands(&[command::reset()])?;
    delay.delay_us(100_u16);
    display.send_commands(init_sequence())?;
    Ok(())
}

fn clear(buffer: &mut ArrayDisplayBuffer) {
    for y in 0..buffer.line_count() {
        clear_line(buffer, y);
    }
}

fn clear_line(buffer: &mut ArrayDisplayBuffer, y: usize) {
    if let Some(line) = buffer.get_line_mut(y) {
        line.iter_mut().for_each(|pixel| *pixel = 0);
    }
}