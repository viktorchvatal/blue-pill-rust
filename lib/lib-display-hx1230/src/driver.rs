use core::cmp::min;

use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use lib_common::MiniResult;

use crate::{Hx1230Driver};

pub struct SpiHx1230Driver<'a, SPI, CS> {
    spi: &'a mut SPI,
    cs: &'a mut CS,
}

impl<'a, SPI, CS> SpiHx1230Driver<'a, SPI, CS>
where SPI: spi::Write<u8>, CS: OutputPin {
    /// Create a new driver instance using the borrowed CS pin and SPI interface
    ///
    /// This is a cheap operation so that driver can be constructed multiple
    /// times whenever it is needed and released after use so it does not
    /// block exclusive access to CS and SPI interfaces.
    pub fn new(spi: &'a mut SPI, cs: &'a mut CS) -> Self {
        Self { spi, cs, }
    }

    /// Write large data block containing multiples of 64 bits
    /// using 72bits (9 bytes) emitted through SPI (chip SPI uses 8-bit
    /// interface, but display requires 9-bit interface)
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
        let output_length = encode_control_bit(block, &mut buffer, flag);
        let output = &buffer[0..output_length];
        self.cs.set_low().map_err(|_| ())?;
        self.spi.write(output).map_err(|_| ())?;
        self.cs.set_high().map_err(|_| ())
    }
}

impl<'a, SPI, CS> Hx1230Driver for SpiHx1230Driver<'a, SPI, CS>
where SPI: spi::Write<u8>, CS: OutputPin {
    fn data(&mut self, data: &[u8]) -> MiniResult {
        self.transmit(data, false)
    }

    fn commands(&mut self, commands: &[u8]) -> MiniResult {
        self.transmit(commands, true)
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