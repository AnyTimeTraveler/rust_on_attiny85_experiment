#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]

extern crate avr_device;

use core::panic::PanicInfo;

static mut A: bool = false;

#[avr_device::interrupt(attiny85)]
unsafe fn WDT() {
    A = !A;
}

#[no_mangle]
pub extern fn main() {
    let peripherals = unsafe { avr_device::attiny85::Peripherals::steal() };
    peripherals.WDT.wdtcr.write(|w|
        w
            .wdif().set_bit()
            .wdie().set_bit()
            .wdpl().cycles_256k()
            .wde().set_bit()
    );

    // shorthand for IO
    let portb = peripherals.PORTB;

    // set port 1 (LED) as output
    portb.ddrb.write(|w| w.pb1().set_bit());

    // set port 1 (LED) on
    portb.portb.write(|w| w.pb1().set_bit());



    // do nothing forever
    loop {
        // let x = unsafe { A };
        // portb.portb.write(|w| w.pb1().bit(x));
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
