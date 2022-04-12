#![no_std]
#![no_main]
#![feature(lang_items)]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn main() {
    loop {

    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
