# Interfacing HX1230 Graphical Display

Working example: [demo-display-hx1230](../app/demo-display-hx1230/src/main.rs)

![HX1230 Display](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/display-hx1230/hx1230.gif)

HX1320 is very cheap and low power 96x72 pixel matrix LCD display.
It features optional backlight and configurable contrast. The display is
readable, but not great.

When I tried to use the display first time, I did not find a Rust driver
supporting it, but with the help of other drivers like
[this micropython one](https://github.com/mcauser/micropython-hx1230)
I was able to write my own, yet simple, but working. The driver is
now published as `hx1230` crate.

## Connection

| MCU Board   |     Other     | HX1230 Board | Note         |
| ----------- | ------------- | ------------ | ------------ |
| -           | GND           | GND          |              |
| -           | 100R pull up  | BL           | Backlight    |
| -           | VCC           | VCC          |              |
| PB13 (SCK)  | GND           | CLK          |              |
| PB15 (MOSI) | GND           | DIN          |              |
| -           | GND           | N/C          |              |
| PB12        | GND           | CE           | Chip enable  |
| -           | 5k pull up    | RST          |              |

Notes:
 - backlight can be either connected to VCC via a resistor, or driver
   with a PWM output
 - hardware reset can be permanently inactive using a pull up resistor as
   the display also features software reset using a command