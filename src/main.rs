#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate f3;
extern crate panic_halt;

mod aux5;

use cortex_m_rt::entry;
use f3::hal::delay::Delay;
use f3::hal::prelude::*;
use f3::led::Leds;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let half_period = 250u16;

    loop {
        for led in leds.iter_mut() {
            led.on();
            delay.delay_ms(half_period);
            led.off();
            delay.delay_ms(half_period);
        }
    }
}
