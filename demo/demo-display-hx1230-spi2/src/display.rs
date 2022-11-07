use embedded_hal::{blocking::spi, digital::v2::OutputPin};
use embedded_hal::blocking::delay::DelayUs;
use lib_common::MiniResult;
use lib_display_buffer::DisplayBuffer;
use lib_display_hx1230::command::{init_sequence, set_position};
use lib_display_hx1230::{SpiHx1230Driver, command, Hx1230Driver};

#[inline(never)]
pub fn init_display<SPI, CS, D>(
    spi: &mut SPI,
    cs: &mut CS,
    delay: &mut D,
) -> MiniResult
where SPI: spi::Write<u8>, CS: OutputPin, D: DelayUs<u16> {
    let mut display = SpiHx1230Driver::new(spi, cs);
    display.commands(&[command::reset()])?;
    delay.delay_us(100_u16);
    display.commands(init_sequence())?;
    Ok(())
}

#[inline(never)]
pub fn render_display<SPI, CS>(
    spi: &mut SPI,
    cs: &mut CS,
    input: &dyn DisplayBuffer,
) -> MiniResult
where SPI: spi::Write<u8>, CS: OutputPin {
    let mut driver = SpiHx1230Driver::new(spi, cs);
    driver.commands(&set_position(0, 0))?;

    for line_id in 0..input.line_count() {
        if let Some(ref line) = input.get_line(line_id) {
            driver.data(line)?;
        }
    }

    Ok(())
}