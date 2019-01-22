#![no_main]
#![no_std]

use cortex_m_rt::entry;

use panic_halt;

use nrf52_hal_example::nrf52832_hal::{
    prelude::*,
    gpio::Level,
    //timer::Timer,
};

use nrf52_hal_example::nrf52832_pac::{Peripherals};
use nrf52_hal_example::Pins;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p.P0.split());

    let mut led1 = pins.led1.into_push_pull_output(Level::Low);
    led1.set_low();

    loop {}
    
    //let mut led2 = pins.led2.into_push_pull_output(Level::Low);
    
    // Alternately flash the red and blue leds
    /*loop {
        led1.set_low();
        led2.set_high();
        delay(&mut timer, 250_000); // 250ms
        led1.set_high();
        led2.set_low();
        delay(&mut timer, 1_000_000); // 1s
    }*/
}
/*
fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: TimerExt,
{
    timer.start(cycles);
    block!(timer.wait());
}*/
