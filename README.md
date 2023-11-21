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
    // declare a controller with the number of leds and a time delay (will move this later to execute_pattern)
    let mut controller = EasyBlinkController::new(120, 20);

    // execute a pattern in the color of your choosing.  must be in a loop otherwise it won't keep going..
    loop {
        controller.execute_pattern(Color::Rainbow, Pattern::Chase);
        // et voila!
    }
}
```
