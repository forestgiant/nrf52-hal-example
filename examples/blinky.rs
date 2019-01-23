#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52_hal_example::nb::block;

use panic_halt;

use nrf52_hal_example::nrf52832_hal::{gpio::Level, prelude::*, timer::Timer};

use nrf52_hal_example::nrf52832_pac::Peripherals;
use nrf52_hal_example::Pins;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p.P0.split());

    let mut led1 = pins.led1.into_push_pull_output(Level::Low);
    let mut led2 = pins.led2.into_push_pull_output(Level::Low);
    let mut led3 = pins.led3.into_push_pull_output(Level::Low);
    let mut led4 = pins.led4.into_push_pull_output(Level::Low);

    let mut timer = p.TIMER0.constrain();

    loop {
        led1.set_low();
        led2.set_high();
        led3.set_high();
        led4.set_high();

        delay(&mut timer, 250_000);

        led1.set_high();
        led2.set_low();
        led3.set_high();
        led4.set_high();

        delay(&mut timer, 250_000);

        led1.set_high();
        led2.set_high();
        led3.set_low();
        led4.set_high();

        delay(&mut timer, 250_000);

        led1.set_high();
        led2.set_high();
        led3.set_high();
        led4.set_low();

        delay(&mut timer, 250_000);
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: TimerExt,
{
    timer.start(cycles);
    block!(timer.wait());
}
