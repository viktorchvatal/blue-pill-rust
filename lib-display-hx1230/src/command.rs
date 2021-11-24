#[derive(Clone, Copy)]
pub struct Command(u8);

impl Command {
    /// Display software reset
    pub fn reset() -> Self {
        Self(0xE2)
    }

    /// Turn on power
    pub fn power_on() -> Self {
        Self(0x2F)
    }

    /// Turn off power
    pub fn power_off() -> Self {
        Self(0x28)
    }

    /// Turn on horizontal mirror
    pub fn horizontal_flip_on() -> Self {
        Self(0xA1)
    }

    /// Turn off horizontal mirror
    pub fn horizontal_flip_off() -> Self {
        Self(0xA0)
    }

    /// Turn on vertical mirror
    pub fn vertical_flip_on() -> Self {
        Self(0xC8)
    }

    /// Turn off vertical mirror
    pub fn vertical_flip_off() -> Self {
        Self(0xC0)
    }

    /// Enable display test (all pixels active)
    pub fn display_test_on() -> Self {
        Self(0xA5)
    }

    /// Disable display test (all pixels active)
    pub fn display_test_off() -> Self {
        Self(0xA4)
    }

    /// Enable inverted pixels (negative image)
    pub fn invert_on() -> Self {
        Self(0xA7)
    }

    /// Disable inverted pixels (negative image)
    pub fn invert_off() -> Self {
        Self(0xA6)
    }

    /// Turn on display
    pub fn display_on() -> Self {
        Self(0xAF)
    }

    /// Turn off display
    pub fn display_off() -> Self {
        Self(0xAE)
    }

    /// Set display contrast (0 - 31)
    pub fn set_contrast(contrast: u8) -> Self {
        Self(0x80 | (0b00011111 & contrast))
    }

    /// Set display line (0 - 63)
    pub fn set_line(line: u8) -> Self {
        Self(0x40 | (0b00111111 & line))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}