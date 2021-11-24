use core::marker::PhantomData;
use core::fmt::Debug;

use cortex_m::prelude::{_embedded_hal_blocking_delay_DelayUs};
use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use stm32f1xx_hal::{delay::Delay, gpio::{Output, Pin, PushPull}};

pub struct Hx1230Driver<SPI, E, CS> {
    _phantom_spi: PhantomData<SPI>,
    _phantom_err: PhantomData<E>,
    _phantom_cs: PhantomData<CS>,
}

impl<SPI, E, CS> Hx1230Driver<SPI, E, CS>
where SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E>,
      E: Debug,
      CS: OutputPin {
    pub fn new(_spi: &SPI, _cs: &CS) -> Self {
        Self {
            _phantom_spi: PhantomData,
            _phantom_err: PhantomData,
            _phantom_cs: PhantomData,
        }
    }

    pub fn init(&self, spi: &mut SPI, cs: &mut CS, delay: &mut Delay) {
        self.command(spi, cs, SW_RESET);
        delay.delay_us(100_u16);
        self.command(spi, cs, POWER_ON);
        self.set_contrast(spi, cs, 30);
        self.command(spi, cs, INVERT_OFF);
        self.command(spi, cs, DISPLAY_NORMAL);
        self.command(spi, cs, SEG_NORMAL);
        self.command(spi, cs, COM_NORMAL);
        self.command(spi, cs, DISPLAY_ON);
        self.set_line(spi, cs, 0);

    }

    // Set contrast to value 0 - 31
    pub fn set_contrast(&self, spi: &mut SPI, cs: &mut CS, value: u8) {
        self.command(spi, cs, CONTRAST | (0b00011111 & value));
    }

    // Set start line to value 0 - 63
    pub fn set_line(&self, spi: &mut SPI, cs: &mut CS, value: u8) {
        self.command(spi, cs, START_LINE | (0b00111111 & value));
    }

    pub fn set_display_test(&self, spi: &mut SPI, cs: &mut CS, value: bool) {
        match value {
            true => self.command(spi, cs, DISPLAY_TEST),
            false => self.command(spi, cs, DISPLAY_NORMAL),
        }
    }

    fn command(&self, spi: &mut SPI, cs: &mut CS, command: u8) {
        cs.set_low();
        spi.write(&[command >> 1, (command << 7) & 0x80]).unwrap();
        cs.set_high();
    }
}

const POWER_ON        : u8 = 0x2F; // internal power supply on
const POWER_OFF       : u8 = 0x28; // internal power supply off
const CONTRAST        : u8 = 0x80; // 0x80 + (0~31)
const SEG_NORMAL      : u8 = 0xA0; // SEG remap normal
const SEG_REMAP       : u8 = 0xA1; // SEG remap reverse (flip horizontal)
const DISPLAY_NORMAL  : u8 = 0xA4; // display ram contents
const DISPLAY_TEST    : u8 = 0xA5; // all pixels on
const INVERT_OFF      : u8 = 0xA6; // not inverted
const INVERT_ON       : u8 = 0xA7; // inverted
const DISPLAY_ON      : u8 = 0xAF; // display on
const DISPLAY_OFF     : u8 = 0xAE; // display off
const START_LINE      : u8 = 0x40; // 0x40 + (0~63)
const COM_NORMAL      : u8 = 0xC0; // COM remap normal
const COM_REMAP       : u8 = 0xC8; // COM remap reverse (flip vertical)
const SW_RESET        : u8 = 0xE2; // connect RST pin to GND to rely on software reset