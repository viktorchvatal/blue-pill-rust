pub trait SwSpi {
    fn hw_reset(&mut self);
    fn mosi_set_high(&mut self);
    fn mosi_set_low(&mut self);
    fn sck_toggle_high_low(&mut self);
    fn long_init_delay(&mut self);
}

pub struct Hx1230SwDriver {

}

impl Hx1230SwDriver {
    pub fn new() -> Self {
        Self { }
    }

    pub fn init(&self, spi: &mut dyn SwSpi) {
        spi.hw_reset();
        spi.long_init_delay();
        self.command(spi, 0b00100001);
        self.command(spi, 0b10010000);
        self.command(spi, 0b00100000);
        self.command(spi, 0b00001001);

        // https://www.sparkfun.com/datasheets/LCD/Monochrome/Nokia5110.pdf
        // https://e2e.ti.com/support/microcontrollers/arm-based-microcontrollers-group/arm-based-microcontrollers/f/arm-based-microcontrollers-forum/401871/tm4c123g6pm-and-initialization-of-the-lcd-with-pcd8544-on-assembly-lang

    }

    // Set contrast to value 0 - 31
    pub fn set_contrast(&self, spi: &mut dyn SwSpi, value: u8) {
        self.command(spi, CONTRAST | (0b00011111 & value));
    }

    // Set start line to value 0 - 63
    pub fn set_line(&self, spi: &mut dyn SwSpi, value: u8) {
        self.command(spi, START_LINE | (0b00111111 & value));
    }

    pub fn set_display_test(&self, spi: &mut dyn SwSpi, value: bool) {
        match value {
            true => self.command(spi, DISPLAY_TEST),
            false => self.command(spi, DISPLAY_NORMAL),
        }
    }

    fn command(&self,  spi: &mut dyn SwSpi, command: u8) {
        // self.output_bit(spi, 0);

        for shift in 0u8..=7u8 {
            self.output_bit(spi, command >> (7 - shift));
        }
    }

    fn output_bit(&self,  spi: &mut dyn SwSpi, bit: u8) {
        if bit & 0x01 == 1 { spi.mosi_set_high() } else { spi.mosi_set_low() };
        spi.sck_toggle_high_low();
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