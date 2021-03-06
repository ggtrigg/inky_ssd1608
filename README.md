# Inky ePaper (SSD1608) Display Driver

Rust [embedded-graphics](https://crates.io/crates/embedded-graphics) driver for the Pimoroni Inky pHat black/white ePaper display controller.

**Note:** This driver for the black/white display uses the SSD1608 display driver which is different to the black/white/red display which uses the SSD1675 display driver (which is supported by a different rust [crate](https://crates.io/crates/ssd1675)).
![inky_test_image](https://user-images.githubusercontent.com/32510770/164960611-16d501d8-a92b-49d8-baf7-376e00c83ef1.jpg)

## Tested Devices

The library has been tested and confirmed working on these devices:

* Black/White [Inky pHAT] version 2 on Raspberry Pi Zero

## Examples

**Note:** To build the examples the `examples` feature needs to be enabled. E.g.

    cargo build --release --examples --features examples
