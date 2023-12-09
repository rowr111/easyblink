# easyblink
blinky library built on top of the [blinkt Rust library](https://github.com/golemparts/blinkt)


## how to use
This code assumes you have a strip of APA102 LEDs and a raspberry pi.

Check the [blinkt docs](https://docs.golemparts.com/blinkt/0.7.1/blinkt/) for some details abut raspberry pi pins etc., but here's the TL;DR:

- use `raspi-config` to enable SPI ports, under "Interface Options->SPI" (you should see an entry for `/dev/spidev0.0` if successful)
- connect the APA102 clock pin to physical pin 23 ("GPIO 11") and data to physical pin 19 ("GPIO 10")
- there is a convenient ground at physical pin 20

### example code
```
use easyblink::{EasyBlinkController, Color, Pattern};

fn main() {
    // declare a controller with the number of leds
    let mut controller = EasyBlinkController::new(120);

    // execute a pattern in the color of your choosing.  must be in a loop otherwise it won't keep going..  last value is the delay_ms, play around to find what works for you
    loop {
        controller.execute_pattern(Color::Rainbow, Pattern::Chase, 20);
        // et voila!
    }
}
```
