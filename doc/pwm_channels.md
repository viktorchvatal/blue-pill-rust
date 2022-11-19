# PWN Channels and colored LEDs [in progress]

Example code: [demo-pwm/src/main.rs](../app/demo-pwm/src/main.rs)

![PWM LEDs](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/pwm-channels/pwm-leds.gif)

Setting up PWM channels is quite straightforward

Set up pins as outputs

```rust
let p1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
let p2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
let p3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
let p4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);
```

Enable PWM

```rust
let mut afio = dp.AFIO.constrain();
let pins = (p1, p2, p3, p4);

let mut pwm = dp
    .TIM2
    .pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 500.Hz(), &clocks);

pwm.enable(Channel::C1);
pwm.enable(Channel::C2);
pwm.enable(Channel::C3);
pwm.enable(Channel::C4);
```

And set duty for any PWM channel as needed (`micromath` library can be used
for fast approximation of many float operations like trigonometric functions
and more )

```rust
let max_duty = pwm.get_max_duty() as f32;

let duty_1 = max_duty;
let duty_2 = max_duty/2.0;
let duty_3 = max_duty/4.0;
let duty_4 = max_duty/8.0;

pwm.set_duty(Channel::C1, duty_1);
pwm.set_duty(Channel::C2, duty_2);
pwm.set_duty(Channel::C3, duty_3);
pwm.set_duty(Channel::C4, duty_4);
```