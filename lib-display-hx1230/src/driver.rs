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
        self.command(Command::set_line(0))?;
        Ok(())

    }

    #[inline(never)]
    pub fn command(&mut self, command: Command) -> MiniResult {
        self.cs.set_low().map_err(|_| ())?;
        let val = command.value();
        let _ = self.spi.write(&[val >> 1, (val << 7) & 0x80]).map_err(|_| ())?;
        let _ = self.cs.set_high().map_err(|_| ())?;
        Ok(())
    }
}