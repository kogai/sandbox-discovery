#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
// you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate panic_halt;
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop();
    loop {
        // your code goes here
    }
}
