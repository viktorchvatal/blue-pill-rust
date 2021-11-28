#![no_std]
#![no_main]

use core::f32::consts::FRAC_PI_2;

use cortex_m_rt::entry;
use stm32f1xx_hal::pwm::Channel;
use stm32f1xx_hal::timer::{Tim2NoRemap, Timer};
use stm32f1xx_hal::{pac, prelude::*};
use lib_panic_led as _;
use micromath::F32Ext;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr.use_hse(8.mhz())  // use external oscillator (8 MHz)
        .sysclk(72.mhz())  // system clock, PLL multiplier should be 6
        .hclk(8.mhz())     // clock used for timers
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();

    let p1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let p2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let p3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let p4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);

    let mut afio = dp.AFIO.constrain();

    let mut pwm = Timer::tim2(dp.TIM2, &clocks)
        .pwm::<Tim2NoRemap, _, _, _>((p1, p2, p3, p4), &mut afio.mapr, 500.hz());

    // Enable clock on each of the channels
    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    pwm.enable(Channel::C3);
    pwm.enable(Channel::C4);

    pwm.set_period(500.hz());

    let mut phase: f32 = 0.0;

    let max_gy = pwm.get_max_duty() as f32;
    let max_r = max_gy/4.0; // red is too bright, decrase duty in software
    let max_b = max_r/4.0; // blue is even brighter, dim it even more

    loop {
        pwm.set_duty(Channel::C1, compute_duty(phase, C1_SHIFT, max_gy));
        pwm.set_duty(Channel::C2, compute_duty(phase, C2_SHIFT, max_gy));
        pwm.set_duty(Channel::C3, compute_duty(phase, C3_SHIFT, max_r));
        pwm.set_duty(Channel::C4, compute_duty(phase, C4_SHIFT, max_b));

        phase += 0.001;
    }
}

fn maxf(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

fn compute_duty(phase: f32, shift: f32, max: f32) -> u16 {
    (sqr(maxf(0.0, (phase + shift).sin()))*max) as u16
}

fn sqr(value: f32) -> f32 {
    value*value
}

const C1_SHIFT: f32 = FRAC_PI_2*0.0;
const C2_SHIFT: f32 = FRAC_PI_2*1.0;
const C3_SHIFT: f32 = FRAC_PI_2*2.0;
const C4_SHIFT: f32 = FRAC_PI_2*3.0;