#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate avr_device;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn main() {
    let peripherals = unsafe { avr_device::attiny85::Peripherals::steal() };

    // shorthand for IO
    let portb = peripherals.PORTB;

    // set port 1 (LED) as output
    portb.ddrb.write(|w| w.pb1().set_bit());

    // set port 1 (LED) on
    portb.portb.write(|w| w.pb1().set_bit());

    // do nothing forever
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
