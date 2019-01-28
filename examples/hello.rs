#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // let mut stdout = hio::hstdout().unwrap();
    // writeln!(stdout, "Hello, world!").unwrap();

    // debug::exit(debug::EXIT_SUCCESS);
    hprintln!("Hello, world").unwrap();

    loop {}
}
