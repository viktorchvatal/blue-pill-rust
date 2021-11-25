use core::cmp::min;

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
        self.transmit(INIT_COMMANDS, true)
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
            self.send_data(&[0; 8])?;
        }

        self.reset_position()
    }

    #[inline(never)]
    pub fn command(&mut self, command: Command) -> MiniResult {
        self.transmit(&[command.value()], true)
    }

    pub fn send_data(&mut self, data: &[u8]) -> MiniResult {
        self.transmit(data, false)
    }

    pub fn send_commands(&mut self, commands: &[u8]) -> MiniResult {
        self.transmit(commands, true)
    }

    /// Write 64 bits of data using 72bits (9 bytes) emitted through SPI
    #[inline(never)]
    fn transmit(&mut self, data: &[u8], is_command: bool) -> MiniResult {
        let flag = (!is_command) as u8;
        let data_len = data.len();

        let max: usize = data_len/8 + (data_len % 8 > 0) as usize;

        for block_id in 0..max {
            let block_start = min(block_id*8, data.len());
            let block_end = min(block_id*8+8, data.len());
            let block = &data[block_start..block_end];

            let len = block.len();
            let mut buffer = [0u8; 9];

            for shift in 0..len {
                buffer[shift] |= flag << (7 - shift);

                if shift == 7 {
                    buffer[shift + 1] = block[shift];
                } else {
                    buffer[shift] |= block[shift] >> (shift + 1);
                    buffer[shift + 1] |= block[shift] << (7 - shift);
                }
            }

            let output = if len == 8 { &buffer[..] } else { &buffer[0..(len+1)] };

            self.cs.set_low().map_err(|_| ())?;
            self.spi.write(output).map_err(|_| ())?;
            self.cs.set_high().map_err(|_| ())?;
        }

        Ok(())
    }
}

const INIT_COMMANDS: &[u8] = &[
    Command::power_on().value(),
    Command::set_contrast(30).value(),
    Command::display_test_off().value(),
    Command::horizontal_flip_off().value(),
    Command::vertical_flip_off().value(),
    Command::invert_off().value(),
    Command::display_on().value(),
    Command::set_column_low(0).value(),
    Command::set_column_high(0).value(),
    Command::set_page(0).value(),
];