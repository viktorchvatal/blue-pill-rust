use core::cmp::min;

use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use lib_common::MiniResult;

use crate::command;

pub struct SpiHx1230Driver<'a, SPI, CS> {
    spi: &'a mut SPI,
    cs: &'a mut CS,
}

impl<'a, SPI, CS> SpiHx1230Driver<'a, SPI, CS>
where SPI: spi::Write<u8>, CS: OutputPin {
    pub fn new(spi: &'a mut SPI, cs: &'a mut CS) -> Self {
        Self { spi, cs, }
    }

    pub fn init_sequence(&mut self) -> MiniResult {
        self.transmit(INIT_COMMANDS, true)
    }

    pub fn set_column(&mut self, column: u8) -> MiniResult {
        self.transmit_block(
            &[
                command::set_column_low(column),
                command::set_column_high(column),
            ],
            true
        )
    }

    pub fn reset_position(&mut self) -> MiniResult {
        self.set_column(0)?;
        self.command(command::set_page(0))
    }

    pub fn clear_data(&mut self) -> MiniResult {
        self.reset_position()?;

        for _ in 0..12*9 {
            self.send_data(&[0; 8])?;
        }

        self.reset_position()
    }

    #[inline(never)]
    pub fn command(&mut self, command: u8) -> MiniResult {
        self.transmit_block(&[command], true)
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
        let data_len = data.len();
        let max: usize = data_len/8 + (data_len % 8 > 0) as usize;

        for block_id in 0..max {
            let block_start = min(block_id*8, data.len());
            let block_end = min(block_id*8+8, data.len());
            let block = &data[block_start..block_end];
            self.transmit_block(block, is_command)?;
        }

        Ok(())
    }

    /// Write a block of data no longer than 64 bits using 72bits (9 bytes)
    /// emitted through SPI (input data can be shorter, but not longer than
    /// 8 bytes)
    #[inline(never)]
    fn transmit_block(&mut self, data: &[u8], is_command: bool) -> MiniResult {
        let flag = (!is_command) as u8;
        let block = &data[0..min(data.len(), 8)];
        let mut buffer = [0u8; 9];
        let outuput_length = encode_control_bit(block, &mut buffer, flag);
        let output = &buffer[0..outuput_length];
        self.cs.set_low().map_err(|_| ())?;
        self.spi.write(output).map_err(|_| ())?;
        self.cs.set_high().map_err(|_| ())
    }
}

#[inline(never)]
fn encode_control_bit(data: &[u8], output: &mut [u8; 9], bit: u8) -> usize {
    let data = &data[0..min(data.len(), 8)];
    let len = data.len();

    for shift in 0..len {
        output[shift] |= bit << (7 - shift);

        if shift == 7 {
            output[shift + 1] = data[shift];
        } else {
            output[shift] |= data[shift] >> (shift + 1);
            output[shift + 1] |= data[shift] << (7 - shift);
        }
    }

    if len == 8 { 9 } else { len + 1 }
}

const INIT_COMMANDS: &[u8] = &[
    command::power_on(),
    command::set_contrast(30),
    command::display_test_off(),
    command::horizontal_flip_off(),
    command::vertical_flip_off(),
    command::invert_off(),
    command::display_on(),
    command::set_column_low(0),
    command::set_column_high(0),
    command::set_page(0),
];