use core::marker::PhantomData;
use core::fmt::Debug;

use cortex_m::prelude::{_embedded_hal_blocking_delay_DelayUs};
use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use stm32f1xx_hal::{delay::Delay, gpio::{Output, Pin, PushPull}};

pub struct Hx1230Driver<'a, SPI, E, CS> {
    spi: &'a mut SPI,
    cs: &'a mut CS,
    _phantom_err: PhantomData<E>,
}

impl<'a, SPI, E, CS> Hx1230Driver<'a, SPI, E, CS>
where SPI: spi::Transfer<u8, Error = E> + spi::Write<u8, Error = E>,
      E: Debug,
      CS: OutputPin {
    pub fn new(spi: &'a mut SPI, cs: &'a mut CS) -> Self {
        Self {
            spi,
            cs,
            _phantom_err: PhantomData,
        }
    }

    pub fn sw_reset(&mut self) {
        self.command(SW_RESET);
    }

    pub fn init_sequence(&mut self) {
        self.command(POWER_ON);
        self.set_contrast(30);
        self.command(INVERT_OFF);
        self.command(DISPLAY_NORMAL);
        self.command(SEG_NORMAL);
        self.command(COM_NORMAL);
        self.command(DISPLAY_ON);
        self.set_line(0);

    }

    // Set contrast to value 0 - 31
    pub fn set_contrast(&mut self, value: u8) {
        self.command(CONTRAST | (0b00011111 & value));
    }

    // Set start line to value 0 - 63
    pub fn set_line(&mut self, value: u8) {
        self.command(START_LINE | (0b00111111 & value));
    }

    pub fn set_display_test(&mut self, value: bool) {
        match value {
            true => self.command(DISPLAY_TEST),
            false => self.command(DISPLAY_NORMAL),
        }
    }

    fn command(&mut self, command: u8) {
        self.cs.set_low();
        self.spi.write(&[command >> 1, (command << 7) & 0x80]).unwrap();
        self.cs.set_high();
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