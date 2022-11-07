use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use embedded_hal::blocking::delay::DelayUs;
use lib_common::MiniResult;
use hx1230::command::{init_sequence};
use hx1230::{SpiDriver, command, DisplayDriver};

#[inline(never)]
pub fn init_display<SPI, CS, D>(
    spi: &mut SPI,
    cs: &mut CS,
    delay: &mut D,
) -> MiniResult
where SPI: spi::Write<u8>, CS: OutputPin, D: DelayUs<u16> {
    let mut display = SpiDriver::new(spi, cs);
    display.commands(&[command::reset()])?;
    delay.delay_us(100_u16);
    display.commands(init_sequence())?;
    Ok(())
}
