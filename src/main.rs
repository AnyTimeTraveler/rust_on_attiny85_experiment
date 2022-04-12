#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate avr_device;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::panic::PanicInfo;

static mut A: bool = false;

pub extern fn isr() {
    unsafe { A = !A };
}

#[no_mangle]
pub extern fn main() {
    let peripherals = unsafe { avr_device::attiny85::Peripherals::steal() };
    peripherals.WDT.wdtcr.write(|w|
        w.wdif().set_bit()
            .wdie().set_bit()
            .wde().set_bit()
            .wdpl().cycles_64k());

    // peripherals.EXINT.gifr.write(|w| w.)

    unsafe  {
        let prt =
    }

    // shorthand for IO
    let portb = peripherals.PORTB;

    // set port 1 (LED) as output
    portb.ddrb.write(|w| w.pb1().set_bit());

    // set port 1 (LED) on
    portb.portb.write(|w| w.pb1().set_bit());

    // do nothing forever
    loop {
        unsafe {
            portb.portb.write(|w| w.pb1().bit(A));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
