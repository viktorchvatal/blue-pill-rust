use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use lib_common::MiniResult;

use crate::command::Command;

pub struct Driver<'a, SPI, CS> {
    spi: &'a mut SPI,
    cs: &'a mut CS,
}

impl<'a, SPI: spi::Write<u8>, CS: OutputPin> Driver<'a, SPI, CS> {
    pub fn new(spi: &'a mut SPI, cs: &'a mut CS) -> Self {
        Self { spi, cs, }
    }

    pub fn init_sequence(&mut self) -> MiniResult {
        self.command(Command::power_on())?;
        self.command(Command::set_contrast(30))?;
        self.command(Command::display_test_off())?;
        self.command(Command::horizontal_flip_off())?;
        self.command(Command::vertical_flip_off())?;
        self.command(Command::invert_off())?;
        self.command(Command::display_on())?;
        self.reset_position()
    }

    pub fn set_column(&mut self, column: u8) -> MiniResult {
        self.command(Command::set_column_low(column))?;
        self.command(Command::set_column_high(column))
    }

    pub fn reset_position(&mut self) -> MiniResult {
        self.set_column(0)?;
        self.command(Command::set_page(0))
    }

    pub fn clear_data(&mut self) -> MiniResult {
        self.reset_position()?;

        for _ in 0..12*9 {
            self.multiple_data(&[0; 8])?;
        }

        self.reset_position()
    }

    #[inline(never)]
    pub fn command(&mut self, command: Command) -> MiniResult {
        self.cs.set_low().map_err(|_| ())?;
        let val = command.value();
        let _ = self.spi.write(&[val >> 1, (val << 7) & 0x80]).map_err(|_| ())?;
        let _ = self.cs.set_high().map_err(|_| ())?;
        Ok(())
    }

    /// Write one byte of data, data written to display have to be 9 bits,
    /// but 16 bits (including padding bits) are emitted with SPI interface as
    /// it does not support 9 bit output
    #[inline(never)]
    pub fn data(&mut self, data: u8) -> MiniResult {
        self.cs.set_low().map_err(|_| ())?;

        self.spi
            .write(&[0x80 | (data >> 1), (data << 7) & 0x80])
            .map_err(|_| ())?;

        self.cs.set_high().map_err(|_| ())?;
        Ok(())
    }

    /// Write 64 bits of data using 72bits (9 bytes) emitted through SPI
    #[inline(never)]
    pub fn multiple_data(&mut self, data: &[u8; 8]) -> MiniResult {
        self.cs.set_low().map_err(|_| ())?;

        let output: [u8; 9] = [
            (1 << 7) | (data[0] >> 1),
            (1 << 6) | (data[0] << 7) | data[1] >> 2,
            (1 << 5) | (data[1] << 6) | data[2] >> 3,
            (1 << 4) | (data[2] << 5) | data[3] >> 4,
            (1 << 3) | (data[3] << 4) | data[4] >> 5,
            (1 << 2) | (data[4] << 3) | data[5] >> 6,
            (1 << 1) | (data[5] << 2) | data[6] >> 7,
            (1 << 0) | (data[6] << 1),
                       (data[7] << 0),
        ];

        let _ = self.spi.write(&output).map_err(|_| ())?;
        let _ = self.cs.set_high().map_err(|_| ())?;
        Ok(())
    }
}