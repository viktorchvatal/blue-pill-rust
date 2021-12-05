#![no_std]
#![no_main]

use display::{init_display, render_display};
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Primitive, Point};
use embedded_graphics::primitives::{PrimitiveStyle, Circle};
use embedded_hal::spi::{Mode, Phase, Polarity};

use cortex_m_rt::entry;
use lib_display_buffer::{ArrayDisplayBuffer, draw};
use stm32f1xx_hal::delay::Delay;
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
        .use_hse(8.mhz())  // use external oscillator (8 MHz)
        .sysclk(72.mhz())  // system clock, PLL multiplier should be 6
        .hclk(8.mhz())     // clock used for timers
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
        4.mhz(),
        clocks,
    );

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut frame_buffer: ArrayDisplayBuffer<96, 9> = ArrayDisplayBuffer::new();

    init_display(&mut spi, &mut display_cs, &mut delay).check();

    let mut diameter = 1;

    loop {
        led.set_low();
        draw::clear_pattern(&mut frame_buffer, &[0; 8]);

        Circle::new(Point::new(48 - diameter/2, 32 - diameter/2), diameter as u32)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
            .draw(&mut frame_buffer).check();        

        render_display(&mut spi, &mut display_cs, &frame_buffer).check();

        diameter = (diameter + 2) % 100;

        led.set_high();

        delay.delay_ms(30_u16);
    }
}
