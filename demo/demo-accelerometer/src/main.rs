#![no_std]
#![no_main]

use core::fmt::Write;
use arrayvec::ArrayString;
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::{ascii::FONT_5X7, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_hal::spi::{Mode as SpiMode, Phase, Polarity};
use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use embedded_hal::blocking::delay::DelayUs;
use lib_common::MiniResult;
use hx1230::command::{init_sequence};
use hx1230::{ArrayDisplayBuffer, DisplayBuffer, SpiDriver, command, DisplayDriver};

use cortex_m_rt::entry;
use stm32f1xx_hal::i2c::{BlockingI2c, Mode as I2CMode, DutyCycle};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};
use mpu6050::*;

use lib_common::ResultExt;
use lib_panic_led as _;

pub const SPI_MODE: SpiMode = SpiMode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut afio = dp.AFIO.constrain();
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

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        I2CMode::Fast {
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
    delay.delay_ms(100_u16);

    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();

    init_display(&mut spi, &mut display_cs, &mut delay).check();

    let mut diameter = 1;
    let text_style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);

    let mut mpu = Mpu6050::new_with_addr(i2c, 0x72);
    let mut text = ArrayString::<32>::new();

    let _ = write!(&mut text, "Start");
    Text::new(&text, Point::new(0, 5), text_style).draw(&mut frame_buffer).check();
    let mut driver = SpiDriver::new(&mut spi, &mut display_cs);
    driver.send_buffer(&frame_buffer).check();

    if let Err(err) = mpu.init(&mut delay) {
        clear(&mut frame_buffer);
        text.clear();
        let _ = write!(&mut text, "ERROR:\n{:?}", err);
        Text::new(&text, Point::new(0, 5), text_style).draw(&mut frame_buffer).check();
        let mut driver = SpiDriver::new(&mut spi, &mut display_cs);
        driver.send_buffer(&frame_buffer).check();
        loop {}
    }

    loop {
        led.set_low();
        clear(&mut frame_buffer);

        clear_line(&mut frame_buffer, 0);
        clear_line(&mut frame_buffer, 1);

        let angles = mpu.get_acc_angles().unwrap();
        let temp = mpu.get_temp().unwrap();
        let gyro = mpu.get_gyro().unwrap();
        let acc = mpu.get_acc().unwrap();

        text.clear();
        let _ = write!(&mut text, "ANG {:?}", angles);
        Text::new(&text, Point::new(0, 5), text_style).draw(&mut frame_buffer).check();

        text.clear();
        let _ = write!(&mut text, "T {:?}", temp);
        Text::new(&text, Point::new(0, 12), text_style).draw(&mut frame_buffer).check();

        text.clear();
        let _ = write!(&mut text, "G {:?}", gyro);
        Text::new(&text, Point::new(0, 19), text_style).draw(&mut frame_buffer).check();

        text.clear();
        let _ = write!(&mut text, "ACC {:?}", acc);
        Text::new(&text, Point::new(0, 26), text_style).draw(&mut frame_buffer).check();

        let mut driver = SpiDriver::new(&mut spi, &mut display_cs);
        driver.send_buffer(&frame_buffer).check();

        diameter = diameter + 1;

        led.set_high();

        delay.delay_ms(100_u16);
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