#![no_std]
#![no_main]

use core::fmt::Write;
use arrayvec::ArrayString;
use bmp280_rs::{BMP280, I2CAddress, Config};
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::{ascii::FONT_7X13, ascii::FONT_7X13_BOLD, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_hal::spi::{Mode as SpiMode, Phase, Polarity};
use hx1230::{ArrayDisplayBuffer, DisplayBuffer, SpiDriver, DisplayDriver};
use stm32f1xx_hal::i2c::{BlockingI2c, Mode, DutyCycle};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use cortex_m_rt::entry;

use lib_panic_led as _;

pub const SPI_MODE: SpiMode = SpiMode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut afio = dp.AFIO.constrain();
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

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let mut i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 100.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let mut delay = cp.SYST.delay(&clocks);

    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();
    let mut display = SpiDriver::new(&mut spi, &mut display_cs);
    display.initialize(&mut delay).unwrap();
    print_text(&mut frame_buffer, "Starting up...").unwrap();
    display.send_buffer(&frame_buffer).unwrap();
    delay.delay_ms(200_u16);

    let config = Config::handheld_device_dynamic();

    let mut bmp = BMP280::new(&mut i2c, I2CAddress::SdoGrounded, config).unwrap();

    loop {
        led.set_low();
        frame_buffer.clear_buffer(0x00);

        bmp.trigger_measurement(&mut i2c).unwrap();
        let pressure = bmp.read_pressure(&mut i2c).unwrap();
        let temperature = bmp.read_temperature(&mut i2c).unwrap();

        print_measurement(&mut frame_buffer, pressure, temperature).unwrap();

        display.send_buffer(&frame_buffer).unwrap();

        led.set_high();

        delay.delay_ms(500_u16);
    }
}

const TEMPERATURE: &str = "Temperature:";
const PRESSURE: &str = "Pressure:";

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
    raw_pressure: i32,
    raw_temp: i32,
) -> Result<(), ()> {
    let mut text = ArrayString::<20>::new();
    let regular = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let bold = MonoTextStyle::new(&FONT_7X13_BOLD, BinaryColor::On);
    let pressure = raw_pressure/256;

    Text::new(&TEMPERATURE, Point::new(0, 15), regular).draw(frame_buffer).map_err(|_| ())?;
    write!(&mut text, "{}.{} C", raw_temp/100, raw_temp % 100).map_err(|_| ())?;
    Text::new(&text, Point::new(0, 30), bold).draw(frame_buffer).map_err(|_| ())?;
    Text::new(&PRESSURE, Point::new(0, 45), regular).draw(frame_buffer).map_err(|_| ())?;
    text.clear();
    write!(&mut text, "{}.{} hPa", pressure/100, pressure % 100).map_err(|_| ())?;
    Text::new(&text, Point::new(0, 60), bold).draw(frame_buffer).map_err(|_| ())?;

    Ok(())
}