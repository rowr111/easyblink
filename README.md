# EasyBlink LED controller library
 
The project provides a simple way to control a strip of APA102 LEDs using a Raspberry Pi. It is built on top of the [blinkt Rust library](https://github.com/golemparts/blinkt).
 
It is meant at least in part to provide some ideas and code examples for people interested in getting started writing lighting patterns, although it also works well as an easy-to-use way to have some colorful blinky lights in your life.

It would be awesome if you have an idea for a new pattern for the library or anything else that could improve this library - please make a pull request!

## How to Use - Physical Setup
This code assumes you have a linear strip of APA102 LEDs and a raspberry pi.

Check the [blinkt docs](https://docs.golemparts.com/blinkt/0.7.1/blinkt/) for some details abut raspberry pi pins etc., but here's the TL;DR:

- use `raspi-config` to enable SPI ports, under "Interface Options->SPI" (you should see an entry for `/dev/spidev0.0` if successful)
- connect the APA102 clock pin to physical pin 23 ("GPIO 11") and data to physical pin 19 ("GPIO 10")
- there is a convenient ground at physical pin 20

## Simple Code Example:
```
extern crate easyblink;

use easyblink::{EasyBlinkController, ColorwayPattern};

fn main() {
    let mut controller = EasyBlinkController::new(120);

    loop {
        controller.execute_colorway_pattern(ColorwayPattern::Fireplace, 40);
    }

}
```