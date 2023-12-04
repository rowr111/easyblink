extern crate blinkt;
extern crate rand;
use std::time::Duration;
use blinkt::Blinkt;
use blinkt::BlinktSpi;
use std::thread::sleep;
use rand::{thread_rng, Rng};


const RED_HUE: i32 = 0;
const ORANGE_HUE: i32 = 18;
const YELLOW_HUE: i32 = 40;
const GREEN_HUE: i32 = 116;
const BLUE_HUE: i32 = 240;
const PURPLE_HUE: i32 = 266;

pub struct EasyBlinkController {
    blinkt: Blinkt,
    num_leds: usize,
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
    Sparkle,
    KnightRider,
}

impl EasyBlinkController {
    pub fn new(num_leds: usize) -> EasyBlinkController {
        let new_blinkt = Blinkt::with_spi(BlinktSpi::default(), num_leds);
        EasyBlinkController {
            blinkt: new_blinkt,
            num_leds,
        }
    }

    pub fn get_num_leds(&self) -> usize {
        self.num_leds
    }

    pub fn set_num_leds(&mut self, num_leds: usize) {
        self.num_leds = num_leds;
    }

    pub fn execute_pattern(&mut self, color: Color, pattern: Pattern, delay_ms: u64) {
        match pattern {
            Pattern::Pulse => self.pulse(color, delay_ms),
            Pattern::Chase => self.chase(color, delay_ms),
            Pattern::Sparkle => self.sparkle(color, delay_ms),
            Pattern::KnightRider => self.knightrider(color, delay_ms),
        }
    }

    fn pulse(&mut self, color: Color, delay_ms: u64) {
        match color {
            Color::Rainbow => {
                self.pulse_color(-1 as i32, delay_ms)
            }
            Color::Red => {
                self.pulse_color(RED_HUE, delay_ms)
            }
            Color::Orange => {
                self.pulse_color(ORANGE_HUE, delay_ms)
            }
            Color::Yellow => {
                self.pulse_color(YELLOW_HUE, delay_ms)
            }
            Color::Green => {
                self.pulse_color(GREEN_HUE, delay_ms)
            }
            Color::Blue => {
                self.pulse_color(BLUE_HUE, delay_ms)
            }
            Color::Purple => {
                self.pulse_color(PURPLE_HUE, delay_ms)
            }
        }
    }

    fn chase(&mut self, color: Color, delay_ms: u64) {
        match color {
            Color::Rainbow => {

                for offset in 0..360 {
                    for i in 0..self.num_leds {
                        let hue = (offset as f32 + (360.0 / self.num_leds as f32) * i as f32) % 360.0;
                        let (r, g, b) = hsv_to_rgb(hue as i32, 1, 1.0);
                        self.blinkt.set_pixel(i as usize, r, g, b);
                    }
            
                    self.blinkt.show().unwrap();
                    sleep(Duration::from_millis(delay_ms));
            
                }
            }
            Color::Red => {
                self.chase_color(RED_HUE, delay_ms)
            }
            Color::Orange => {
                self.chase_color(ORANGE_HUE, delay_ms)
            }
            Color::Yellow => {
                self.chase_color(YELLOW_HUE, delay_ms)
            }
            Color::Green => {
                self.chase_color(GREEN_HUE, delay_ms)
            }
            Color::Blue => {
                self.chase_color(BLUE_HUE, delay_ms)
            }
            Color::Purple => {
                self.chase_color(PURPLE_HUE, delay_ms)
            }
        }
    }

    fn sparkle(&mut self, color: Color, delay_ms: u64) {
        match color {
            Color::Rainbow => {
                self.sparkle_color(-1, delay_ms)
            }
            Color::Red => {
                self.sparkle_color(RED_HUE, delay_ms)
            }
            Color::Orange => {
                self.sparkle_color(ORANGE_HUE, delay_ms)
            }
            Color::Yellow => {
                self.sparkle_color(YELLOW_HUE, delay_ms)
            }
            Color::Green => {
                self.sparkle_color(GREEN_HUE, delay_ms)
            }
            Color::Blue => {
                self.sparkle_color(BLUE_HUE, delay_ms)
            }
            Color::Purple => {
                self.sparkle_color(PURPLE_HUE, delay_ms)
            }
        }
    }

    fn knightrider(&mut self, color: Color, delay_ms: u64) {
        match color {
            Color::Rainbow => {
                self.knightrider_color(-1, delay_ms)
            }
            Color::Red => {
                self.knightrider_color(RED_HUE, delay_ms)
            }
            Color::Orange => {
                self.knightrider_color(ORANGE_HUE, delay_ms)
            }
            Color::Yellow => {
                self.knightrider_color(YELLOW_HUE, delay_ms)
            }
            Color::Green => {
                self.knightrider_color(GREEN_HUE, delay_ms)
            }
            Color::Blue => {
                self.knightrider_color(BLUE_HUE, delay_ms)
            }
            Color::Purple => {
                self.knightrider_color(PURPLE_HUE, delay_ms)
            }
        }
    }

    fn pulse_color(&mut self, hue: i32, delay_ms: u64) {
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
                for i in 0..self.num_leds {
                    //one rainbow across everything
                    let hue = (i as f32 / self.num_leds as f32) * 359.0;
                    let (r, g, b) = hsv_to_rgb(hue as i32, 1, value);
                    self.blinkt.set_pixel(i as usize, r, g, b);
                }
            }
            else {
                //solid color
                let (r, g, b) = hsv_to_rgb(hue, 1, value);
        
                for i in 0..self.num_leds {
                    self.blinkt.set_pixel(i as usize, r, g, b);
                }
            }

            self.blinkt.show().unwrap();
            sleep(Duration::from_millis(delay_ms));
        }
    }

    fn chase_color(&mut self, hue: i32, delay_ms: u64) {
        //band of 30 leds each
        let band_size = 30;
        //if there are less than 30 leds, just one band of color - otherwise, party!
        let mut num_bands = 1;
        if self.num_leds > band_size {
            num_bands = self.num_leds / band_size;
        }

        let band_width = self.num_leds / (2 * num_bands); //width of each color band
    
        for step in 0..self.num_leds {
            for i in 0..self.num_leds {
                let mut value: f32 = 0.0;
        
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
            sleep(Duration::from_millis(delay_ms));
        }
    }

    fn sparkle_color(&mut self, hue: i32, delay_ms: u64) {
        let mut rng = thread_rng();

        //dim out pixels
        for pixel in &mut self.blinkt {
            let (r, g, b) = pixel.rgb();
            let (h, s, v) = rgb_to_hsv(r, g, b);
            let (r_new, g_new, b_new) = hsv_to_rgb(h, s, v*0.75);
            pixel.set_rgb(r_new, g_new, b_new);
        }
    
        let num_sparks = if self.num_leds < 10 {
            //always insure sparks even if led count is low
            rng.gen_range(1..=self.num_leds) 
        } else {
            rng.gen_range(1..=self.num_leds / 10)
        };
    
        for _ in 0..num_sparks {
            let index = rng.gen_range(0..self.num_leds);
            let value = rng.gen_range(0.5..=1.0);

            let mut final_hue = hue;
            if hue == -1 as i32 {
                final_hue = ((index as f32 / self.num_leds as f32) * 359.0) as i32;
            }
            let (r, g, b) = hsv_to_rgb(final_hue, 1, value);
            self.blinkt.set_pixel(index, r, g, b);
        }

        self.blinkt.show().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }

    fn knightrider_color(&mut self, hue: i32, delay_ms: u64) {
        //40% of the total width
        let mut tail_length = (self.num_leds as f32 * 0.4) as usize;
        if tail_length < 1 as usize {
            tail_length = 1;
        }

        let mut position = 0;
        let mut direction = 1; //1 for forwards, -1 for backwards
    
        //total steps needed for one full bounce
        let total_steps = 2 * self.num_leds - 1;
        for _ in 0..total_steps {

            for i in 0..self.num_leds {
                let mut final_hue = hue;
                if hue == -1 as i32 {
                    final_hue = ((i as f32 / self.num_leds as f32) * 359.0) as i32;
                }
    
                let distance = (position as i32 - i as i32).abs() as usize;
                let brightness = if distance <= tail_length {
                    1.0 - (distance as f32 / tail_length as f32)
                } else {
                    0.0
                };
    
                let (r, g, b) = hsv_to_rgb(final_hue, 1, brightness);
                self.blinkt.set_pixel(i, r, g, b);
            }
          
            //update position and reverse direction at the ends
            position = (position as i32 + direction as i32) as i32;
            if position <= 0 - tail_length as i32 || position >= self.num_leds as i32 - 1 {
                direction *= -1;
            }

            self.blinkt.show().unwrap();
            sleep(Duration::from_millis(delay_ms));
        }
    }
}

fn hsv_to_rgb(hue: i32, saturation: i32, value: f32) -> (u8, u8, u8) {
    let chroma = value * saturation as f32;
    let x = chroma * (1.0 - ((hue as f32 / 60.0) % 2.0 - 1.0).abs());
    let m = value - chroma;

    let (r1, g1, b1) = 
        if hue >= 0 && hue < 60 {
            (chroma, x, 0.0)
        } else if hue >= 60 && hue < 120 {
            (x, chroma, 0.0)
        } else if hue >= 120 && hue < 180 {
            (0.0, chroma, x)
        } else if hue >= 180 && hue < 240 {
            (0.0, x, chroma)
        } else if hue >= 240 && hue < 300 {
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

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (i32, i32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Hue calculation
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    // Saturation calculation
    let saturation = if max == 0.0 {
        0.0
    } else {
        delta / max
    };

    // Value calculation
    let value = max;

    (hue as i32, saturation as i32, value)
}