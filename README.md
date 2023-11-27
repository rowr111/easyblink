# easyblink
blinky library built on top of the blinkt Rust library


## how to use
This code assumes you have a strip of APA102 LEDs and a raspberry pi.

--setup details to be inserted here later--


### example code
```
extern crate easyblink;

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
