# LED display with TM1637 Driver

Example code: [demo-display-max7219/src/main.rs](../app/demo-display-max7219/src/main.rs)

![BMP280 Pressure and Temperature sensor](https://raw.githubusercontent.com/viktorchvatal/blue-pill-rust-assets/master/max7219-7segment/max7219-7segment.gif)

TODO:
 - communication did not work until CS pin was used (connecting CS to GND with a resistor did not work)
 - display did not correctly initialize on pewer up until there was 200ms delay before initializing (100ms was not enough), this may be a power issue