#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let tick_length = 50_u16;
    let ticks_to_next_step = 1_u8;
    let step_length = 3_u8;
    

    let num_ticks = 8 * ticks_to_next_step;
    let mut tick = 0_u8;
    loop {
        if tick < step_length {
            leds[0].on().ok();
        } else {
            leds[0].off().ok();
        }

        if tick >= ticks_to_next_step && tick < ticks_to_next_step + step_length {
            leds[1].on().ok();
        } else {
            leds[1].off().ok();
        }
        
        if tick >= 2*ticks_to_next_step && tick < 2*ticks_to_next_step + step_length {
            leds[2].on().ok();
        } else {
            leds[2].off().ok();
        }
        
        if tick >= 3*ticks_to_next_step && tick < 3*ticks_to_next_step + step_length {
            leds[3].on().ok();
        } else {
            leds[3].off().ok();
        }

        if tick >= 4*ticks_to_next_step && tick < 4*ticks_to_next_step + step_length {
            leds[4].on().ok();
        } else {
            leds[4].off().ok();
        }

        if tick >= 5*ticks_to_next_step && tick < 5*ticks_to_next_step + step_length {
            leds[5].on().ok();
        } else {
            leds[5].off().ok();
        }

        if tick >= 6*ticks_to_next_step && tick < 6*ticks_to_next_step + step_length {
            leds[6].on().ok();
        } else {
            leds[6].off().ok();
        }

        if tick >= 7*ticks_to_next_step && tick < 7*ticks_to_next_step + step_length {
            leds[7].on().ok();
        } else {
            leds[7].off().ok();
        }

        tick = (tick+1) % num_ticks;
        delay.delay_ms(tick_length);
    }
}
