#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};
use tm1637::{ TM1637 };
use lib_panic_led as _;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr.use_hse(8.MHz())  // use external oscillator (8 MHz)
        .sysclk(72.MHz())  // system clock, PLL multiplier should be 6
        .hclk(8.MHz())     // clock used for timers
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_low();

    let mut delay = cp.SYST.delay(&clocks);

    let mut clk = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);
    let mut dio = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);

    {
        let mut display = TM1637::new(&mut clk, &mut dio, &mut delay);
        display.init().unwrap();
        display.clear().unwrap();
        display.set_brightness(7).unwrap();
    }

    let mut index: u16 = 0;

    loop {
        {
            let mut display = TM1637::new(&mut clk, &mut dio, &mut delay);
            display.print_hex(0, &decimal_to_digits(index)).unwrap();
        }

        delay.delay_ms(10_u16);

        index += 1;

        if index == 10000 {
            index = 0;
        }
    }
}

fn decimal_to_digits(decimal: u16) -> [u8; 4] {
    [
        (decimal / 1000) as u8,
        ((decimal % 1000) / 100) as u8,
        ((decimal % 100) / 10) as u8,
        (decimal % 10) as u8
    ]
}
