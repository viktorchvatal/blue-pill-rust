# LED display with TM1637 Driver

OK, during the first attempt, i tried to write interfacing code myself,
but communication needed to change data Pin state between push-pull mode
and input mode, and i did not find a way to write a generic routine
doing this.

There is either `Pin<Dynamic>` in the `stm32f1xx-hal` that is difficult
to be consumed as generic parameter of a function, and `IoPin` from
the `embedded_hal` crate does not seem to be provided by used
`stm32f1xx-hal` create.

In the end i realized that there already is a perfect `tm1637` crate
providing easy to use driver that worked perfectly on the first attempt.

Interestingly, the `tm1637` uses open drain outputs to drive both the
clock and data on the display so it does not need to change the pin mode.

I have used `PB9` and `PB8` to drive the display

```
    let mut clk = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);
    let mut dio = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
```

Example code of the `tm1637` crate usage is in the
[demo-display-tm1637](../demo-display-tm1637/src/main.rs) crate.

![LED Display connected](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/display-tm1637/connected-display.jpg)


Signals on open drain output pins seems to be pushed at their limits,
there seems to be no room to increase the frequency, but the display needs
just a little of the data, so it is a no issue.

![LED Display connected](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/display-tm1637/clock_and_data.png)

I did not have connected any pull up resistors, but there must be some,
either in the blue pill module or inside the TM1637 display board.

