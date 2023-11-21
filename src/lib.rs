extern crate blinkt;
extern crate num_traits;
use std::time::Duration;
use blinkt::Blinkt;
use blinkt::BlinktSpi;
use std::thread::sleep;
use num_traits::Float;


const RED_HUE: i32 = 0;
const ORANGE_HUE: i32 = 9;
const YELLOW_HUE: i32 = 20;
const GREEN_HUE: i32 = 58;
const BLUE_HUE: i32 = 120;
const PURPLE_HUE: i32 = 133;

pub struct EasyBlinkController {
    blinkt: Blinkt,
    num_leds: usize,
    delay_ms: u64,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Rainbow,
}

#[derive(Debug)]
pub enum Pattern {
    Pulse,
    Chase,
}

impl EasyBlinkController {
    pub fn new(num_leds: usize, delay_ms: u64) -> EasyBlinkController {
        let new_blinkt = Blinkt::with_spi(BlinktSpi::default(), num_leds);
        EasyBlinkController {
            blinkt: new_blinkt,
            num_leds,
            delay_ms,
        }
    }

    pub fn get_num_leds(&self) -> usize {
        self.num_leds
    }

    pub fn set_num_leds(&mut self, num_leds: usize) {
        self.num_leds = num_leds;
    }

    pub fn get_delay_ms(&self) -> u64 {
        self.delay_ms
    }

    pub fn set_delay_ms(&mut self, delay_ms: u64) {
        self.delay_ms = delay_ms;
    }

    pub fn execute_pattern(&mut self, color: Color, pattern: Pattern) {
        match pattern {
            Pattern::Pulse => self.pulse_color(color),
            Pattern::Chase => self.chase_color(color),
        }
    }

    fn pulse_color(&mut self, color: Color) {
        match color {
            Color::Rainbow => {
                self.pulse_solid_color(-1 as i32)
            }
            Color::Red => {
                self.pulse_solid_color(RED_HUE)
            }
            Color::Orange => {
                self.pulse_solid_color(ORANGE_HUE)
            }
            Color::Yellow => {
                self.pulse_solid_color(YELLOW_HUE)
            }
            Color::Green => {
                self.pulse_solid_color(GREEN_HUE)
            }
            Color::Blue => {
                self.pulse_solid_color(BLUE_HUE)
            }
            Color::Purple => {
                self.pulse_solid_color(PURPLE_HUE)
            }
        }
    }

    fn chase_color(&mut self, color: Color) {
        match color {
            Color::Rainbow => {
                for offset in 0..179 {
                    for i in 0..(self.num_leds-1) {
                        let h = ((offset + i) % 179) as i32;
                        let (r, g, b) = hsv_to_rgb(h, 1, 1.0);
        
                        self.blinkt.set_pixel(i as usize, r, g, b);
                    }
            
                    self.blinkt.show().unwrap();
                    sleep(Duration::from_millis(self.delay_ms));
            
                }
            }
            Color::Red => {
                self.chase_solid_color(RED_HUE)
            }
            Color::Orange => {
                self.chase_solid_color(ORANGE_HUE)
            }
            Color::Yellow => {
                self.chase_solid_color(YELLOW_HUE)
            }
            Color::Green => {
                self.chase_solid_color(GREEN_HUE)
            }
            Color::Blue => {
                self.chase_solid_color(BLUE_HUE)
            }
            Color::Purple => {
                self.chase_solid_color(PURPLE_HUE)
            }
        }
    }

    fn pulse_solid_color(&mut self, hue: i32) {
        let max_steps = 100; //arbitrary value..
        for step in 0..max_steps {
            //convert the step to a value in the range 0.0 to 1.0 and back to 0.0
            let midpoint = max_steps / 2;
            let value = if step <= midpoint {
                0.15 + 0.85 * (step as f32 / midpoint as f32)
            } else {
                1.0 - 0.85 * ((step - midpoint) as f32 / midpoint as f32)
            };
        
            if hue == -1 as i32 {
                //rainbow time!
                for i in 0..(self.num_leds-1) {
                    //one rainbow across everything
                    let hue = (i as f32 / (self.num_leds-1) as f32) * 179.0;
                    let (r, g, b) = hsv_to_rgb(hue as i32, 1, value);
                    self.blinkt.set_pixel(i as usize, r, g, b);
                }
            }
            else {
                //solid color
                let (r, g, b) = hsv_to_rgb(hue, 1, value);
        
                for i in 0..(self.num_leds-1) {
                    self.blinkt.set_pixel(i as usize, r, g, b);
                }
            }

            self.blinkt.show().unwrap();
            sleep(Duration::from_millis(self.delay_ms));
        }
    }

    fn chase_solid_color(&mut self, hue: i32) {
        //band of 30 leds each
        let band_size = 30;
        //if there are less than 30 leds, just one band of color - otherwise, party!
        let mut num_bands = 1;
        if self.num_leds > band_size {
            num_bands = self.num_leds / band_size;
        }

        let band_width = self.num_leds / (2 * num_bands); //width of each color band
    
        for step in 0..(self.num_leds-1) {
            for i in 0..(self.num_leds-1) {
                let mut value = 0.0;
        
                for n in 0..num_bands {
                    let band_offset = n * 2 * band_width;
                    let pos_in_band = (i + step + band_offset) % self.num_leds;
    
                    let current_value = if pos_in_band < band_width {
                        //smooth transition from 0 to 1
                        (pos_in_band as f32 / band_width as f32).powf(2.0)
                    } else if pos_in_band < 2 * band_width {
                        //smooth transition from 1 back to 0
                        (1.0 - ((pos_in_band - band_width) as f32 / band_width as f32)).powf(2.0)
                    } else {
                        0.0
                    };
                    value = value.max(current_value);
                }
        
                let (r, g, b) = hsv_to_rgb(hue, 1, value);
                self.blinkt.set_pixel(i as usize, r, g, b);
            }
            self.blinkt.show().unwrap();
            sleep(Duration::from_millis(self.delay_ms));
        }
    }

}

fn hsv_to_rgb(hue: i32, saturation: i32, value: f32) -> (u8, u8, u8) {
    let chroma = value * saturation as f32;
    let x = chroma * (1.0 - ((hue as f32 / 30.0) % 2.0 - 1.0).abs());
    let m = value - chroma;

    let (r1, g1, b1) = 
        if hue >= 0 && hue < 30 {
            (chroma, x, 0.0)
        } else if hue >= 30 && hue < 60 {
            (x, chroma, 0.0)
        } else if hue >= 60 && hue < 90 {
            (0.0, chroma, x)
        } else if hue >= 90 && hue < 120 {
            (0.0, x, chroma)
        } else if hue >= 120 && hue < 150 {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };

    (
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    )
}