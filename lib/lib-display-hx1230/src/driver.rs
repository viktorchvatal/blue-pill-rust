use core::cmp::min;

use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use lib_common::MiniResult;

use crate::{encode::encode_control_bit, Hx1230Driver};

pub struct SpiHx1230Driver<'a, SPI, CS> {
    spi: &'a mut SPI,
    cs: &'a mut CS,
}

impl<'a, SPI, CS> SpiHx1230Driver<'a, SPI, CS>
where SPI: spi::Write<u8>, CS: OutputPin {
    pub fn new(spi: &'a mut SPI, cs: &'a mut CS) -> Self {
        Self { spi, cs, }
    }

    /// Write 64 bits of data using 72bits (9 bytes) emitted through SPI
    #[inline(never)]
    fn transmit(&mut self, data: &[u8], is_command: bool) -> MiniResult {
        let data_len = data.len();

        if data_len == 0 {
            return Ok(())
        }

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

impl<'a, SPI, CS> Hx1230Driver for SpiHx1230Driver<'a, SPI, CS>
where SPI: spi::Write<u8>, CS: OutputPin {
    #[inline(never)]
    fn send_data(&mut self, data: &[u8]) -> MiniResult {
        self.transmit(data, false)
    }

    #[inline(never)]
    fn send_commands(&mut self, commands: &[u8]) -> MiniResult {
        self.transmit(commands, true)
    }
}