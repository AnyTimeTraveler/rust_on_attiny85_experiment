#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![feature(llvm_asm)]

extern crate avr_device;

use core::panic::PanicInfo;
use avr_device::attiny85::Peripherals;

static mut A: bool = false;

#[allow(non_snake_case)]
#[avr_device::interrupt(attiny85)]
unsafe fn WDT() {
    let peripherals = Peripherals::steal();
    peripherals.WDT.wdtcr.write(|w| w
        .wdce().set_bit()
        .wde().set_bit()
        .wdif().set_bit()
    );
    A = !A;
    peripherals.PORTB.portb.write(|w| w.pb1().bit(A));
}

#[avr_device::entry]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };

    // Clear watchdog reset
    peripherals.CPU.mcusr.write(|w|w.wdrf().clear_bit());

    // Unlock watchdog changes
    peripherals.WDT.wdtcr.write(|w|
        w
            .wdce().set_bit() // Watchdog Change Enable
            .wde().set_bit() // Watchdog Enable
    );

    // Configure watchdog
    peripherals.WDT.wdtcr.write(|w|
        w
            .wdif().set_bit() //
            .wdie().set_bit() //
            .wdpl().cycles_64k() //
            .wde().set_bit() //
    );

    // Enable interrupts
    unsafe {
        llvm_asm!("sei");
    }

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
