#[derive(Clone, Copy)]
pub struct Command(u8);

impl Command {
    /// Display software reset
    pub const fn reset() -> Self {
        Self(0xE2)
    }

    /// Turn on power
    pub const fn power_on() -> Self {
        Self(0x2F)
    }

    /// Turn off power
    pub const fn power_off() -> Self {
        Self(0x28)
    }

    /// Turn on horizontal mirror
    pub const fn horizontal_flip_on() -> Self {
        Self(0xA1)
    }

    /// Turn off horizontal mirror
    pub const fn horizontal_flip_off() -> Self {
        Self(0xA0)
    }

    /// Turn on vertical mirror
    pub const fn vertical_flip_on() -> Self {
        Self(0xC8)
    }

    /// Turn off vertical mirror
    pub const fn vertical_flip_off() -> Self {
        Self(0xC0)
    }

    /// Enable display test (all pixels active)
    pub const fn display_test_on() -> Self {
        Self(0xA5)
    }

    /// Disable display test (all pixels active)
    pub const fn display_test_off() -> Self {
        Self(0xA4)
    }

    /// Enable inverted pixels (negative image)
    pub const fn invert_on() -> Self {
        Self(0xA7)
    }

    /// Disable inverted pixels (negative image)
    pub const fn invert_off() -> Self {
        Self(0xA6)
    }

    /// Turn on display
    pub const fn display_on() -> Self {
        Self(0xAF)
    }

    /// Turn off display
    pub const fn display_off() -> Self {
        Self(0xAE)
    }

    /// Set display contrast (0 - 31)
    pub const fn set_contrast(contrast: u8) -> Self {
        Self(0x80 | (0b00011111 & contrast))
    }

    /// Set display line (0 - 63)
    pub const fn set_line(line: u8) -> Self {
        Self(0x40 | (0b00111111 & line))
    }

    /// Set page (0 - 9) - y coordinate byte
    pub const fn set_page(page: u8) -> Self {
        Self(0xB0 | (0b00001111 & page))
    }

    /// Set column low 3 bits (0 - 63) - x coordinate
    pub const fn set_column_low(column: u8) -> Self {
        Self(0b00001111 & column)
    }

    /// Set column low 3 bits (0 - 63) - x coordinate
    pub const fn set_column_high(column: u8) -> Self {
        Self(0b00010000 | (column & 0b00000111))
    }

    pub const fn value(&self) -> u8 {
        self.0
    }
}