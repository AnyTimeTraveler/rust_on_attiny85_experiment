#![feature(asm, lang_items, panic_handler)]

#![no_std]
#![no_main]

extern crate arduino;

use arduino::{DDRB, PORTB};
use core::ptr::write_volatile;

#[no_mangle]
pub extern fn main() {
    // Set all PORTB pins up as outputs
    unsafe { write_volatile(DDRB, 0xFF) }

    loop {
        // Set all pins on PORTB to high.
        unsafe { write_volatile(PORTB, 0xFF) }

        small_delay();

        // Set all pins on PORTB to low.
        unsafe { write_volatile(PORTB, 0x00) }

        small_delay();
    }
}

/// A small busy loop.
fn small_delay() {
    for _ in 0..400000 {
        unsafe { asm!("" :::: "volatile")}
    }
}

// These do not need to be in a module, but we group them here for clarity.
pub mod std {
    #[lang = "eh_personality"]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
    }

    #[panic_handler]
    fn panic(_info: &::core::panic::PanicInfo) -> ! {
        loop {}
    }
}

