#![no_std]
#![no_main]

use core::fmt::Write;
use arrayvec::ArrayString;
use dht11::{Dht11, Measurement};
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::{ascii::FONT_7X13, ascii::FONT_7X13_BOLD, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_hal::spi::{Mode as SpiMode, Phase, Polarity};
use hx1230::{ArrayDisplayBuffer, DisplayBuffer, SpiDriver, DisplayDriver};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use cortex_m_rt::entry;

use lib_panic_led as _;

pub const SPI_MODE: SpiMode = SpiMode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    cp.DCB.enable_trace();
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
    let mut display = SpiDriver::new(&mut spi, &mut display_cs);
    display.initialize(&mut delay).unwrap();
    print_text(&mut frame_buffer, "Starting up...").unwrap();
    display.send_buffer(&frame_buffer).unwrap();
    delay.delay_ms(200_u16);

    let mut dht11 = Dht11::new(thermo_pin);

    loop {
        led.set_low();
        frame_buffer.clear_buffer(0x00);

        let measurement = dht11.perform_measurement(&mut delay);

        match measurement {
            Err(err) => {
                let mut text = ArrayString::<40>::new();
                let _ = write!(&mut text, "E:{:?}", err);
                print_text(&mut frame_buffer, &text).unwrap();
            },
            Ok(values) => print_measurement(&mut frame_buffer, values).unwrap(),
        }

        display.send_buffer(&frame_buffer).unwrap();

        led.set_high();

        delay.delay_ms(500_u16);
    }
}

const TEMPERATURE: &str = "Temperature:";
const HUMIDITY: &str = "Humidity:";

fn print_text(
    frame_buffer: &mut ArrayDisplayBuffer,
    message: &str,
) -> Result<(), ()> {
    let regular = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    Text::new(&message, Point::new(0, 20), regular).draw(frame_buffer).map_err(|_| ())?;
    Ok(())
}

fn print_measurement(
    frame_buffer: &mut ArrayDisplayBuffer,
    values: Measurement,
) -> Result<(), ()> {
    let mut text = ArrayString::<20>::new();
    let regular = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let bold = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);

    Text::new(&TEMPERATURE, Point::new(0, 15), regular).draw(frame_buffer).map_err(|_| ())?;
    write!(&mut text, "{}.{} C", values.temperature/10, values.temperature%10).map_err(|_| ())?;
    Text::new(&text, Point::new(0, 30), bold).draw(frame_buffer).map_err(|_| ())?;
    Text::new(&HUMIDITY, Point::new(0, 45), regular).draw(frame_buffer).map_err(|_| ())?;
    text.clear();
    write!(&mut text, "{}.{} %", values.humidity/10, values.humidity%10).map_err(|_| ())?;
    Text::new(&text, Point::new(0, 60), bold).draw(frame_buffer).map_err(|_| ())?;

    Ok(())
}