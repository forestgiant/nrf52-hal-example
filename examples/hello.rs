#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use nrf52832_pac;
use panic_halt;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    loop {}
}
