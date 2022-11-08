#![no_std]
#![no_main]

use core::fmt::Write;
use arrayvec::ArrayString;
use display::{init_display};
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Circle};
use embedded_graphics::mono_font::{ascii::FONT_6X13, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_hal::spi::{Mode, Phase, Polarity};

use cortex_m_rt::entry;
use hx1230::{ArrayDisplayBuffer, DisplayBuffer, SpiDriver, DisplayDriver};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};

use lib_common::ResultExt;
use lib_panic_led as _;

mod display;

pub const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())  // use external oscillator (8 MHz)
        .sysclk(72.MHz())  // system clock, PLL multiplier should be 6
        .hclk(8.MHz())     // clock used for timers
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut display_cs = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // SPI2, we use only output, so there is no miso input
    let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);

    let mut spi = Spi::spi2(
        dp.SPI2,
        (sck, NoMiso, mosi),
        SPI_MODE,
        4.MHz(),
        clocks,
    );

    let mut delay = cp.SYST.delay(&clocks);
    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();

    init_display(&mut spi, &mut display_cs, &mut delay).check();

    let mut diameter = 1;
    let text_style = MonoTextStyle::new(&FONT_6X13, BinaryColor::On);

    loop {
        led.set_low();
        clear(&mut frame_buffer);

        draw_circle(48, 40, (diameter + 10) % 80, &mut frame_buffer);
        draw_circle(20, 20, (diameter +  0) % 60, &mut frame_buffer);
        draw_circle(60, 20, (diameter + 20) % 60, &mut frame_buffer);
        draw_circle(80, 50, (diameter + 30) % 60, &mut frame_buffer);
        draw_circle(20, 60, (diameter + 40) % 60, &mut frame_buffer);

        clear_line(&mut frame_buffer, 0);
        clear_line(&mut frame_buffer, 1);

        let mut text = ArrayString::<14>::new();
        let _ = write!(&mut text, "Bubbles {}", diameter);

        Text::new(&text, Point::new(0, 12), text_style)
            .draw(&mut frame_buffer)
            .check();


        let mut driver = SpiDriver::new(&mut spi, &mut display_cs);
        driver.send_buffer(&frame_buffer).check();

        diameter = diameter + 1;

        led.set_high();

        delay.delay_ms(100_u16);
    }
}

fn draw_circle<D>(x: i32, y: i32, diameter: i32, frame_buffer: &mut D)
where D: DrawTarget<Color = BinaryColor> {
    Circle::new(Point::new(x - diameter/2, y - diameter/2), diameter as u32)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
        .draw(frame_buffer)
        .check();
}

fn clear(buffer: &mut ArrayDisplayBuffer) {
    for y in 0..buffer.line_count() {
        clear_line(buffer, y);
    }
}

fn clear_line(buffer: &mut ArrayDisplayBuffer, y: usize) {
    if let Some(line) = buffer.get_line_mut(y) {
        line.iter_mut().for_each(|pixel| *pixel = 0);
    }
}