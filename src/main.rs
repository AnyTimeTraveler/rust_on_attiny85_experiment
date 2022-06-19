#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![feature(llvm_asm)]

extern crate avr_device;

use core::ops::BitAnd;
use core::panic::PanicInfo;
use avr_device::attiny85::Peripherals;

static mut COUNTER: u16 = 0;

#[allow(non_snake_case)]
#[avr_device::interrupt(attiny85)]
unsafe fn TIMER0_OVF() {
    COUNTER += 1;
    llvm_asm!("reti");
}

#[allow(non_snake_case)]
#[avr_device::interrupt(attiny85)]
fn TIMER0_COMPA() {}

#[allow(non_snake_case)]
#[avr_device::interrupt(attiny85)]
fn TIMER0_COMPB() {}

#[avr_device::entry]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };

    // Set Synchronization Mode (stops timer)
    peripherals.TC0.gtccr.write(|w| w.tsm().set_bit());
    // Disable COMPA Value compare
    peripherals.TC0.tccr0a.write(|w| w.com0a().disconnected());
    // Disable COMPB Value compare
    peripherals.TC0.tccr0a.write(|w| w.com0b().disconnected());
    // Start counting from 0x00 to 0xFF and overflow back to 0x00 without stopping
    peripherals.TC0.tccr0a.write(|w| w.wgm0().normal_top());

    //Disable COMPA Interrupt
    peripherals.TC0.timsk.write(|w| w.ocie0a().clear_bit());
    // Disable COMPB Interrupt
    peripherals.TC0.timsk.write(|w| w.ocie0b().clear_bit());
    // Enable overflow interrupt
    peripherals.TC0.timsk.write(|w| w.toie0().set_bit());
    // Set Synchronization Mode (stops timer)
    peripherals.TC0.gtccr.write(|w| w.tsm().clear_bit());

    // Enable interrupts
    unsafe {
        llvm_asm!("sei");
    }

    // shorthand for IO
    let portb = peripherals.PORTB;

    // set port 1 (LED) as output
    portb.ddrb.write(|w| w.pb1().set_bit());
    // set port 3 (LED) as output
    portb.ddrb.write(|w| w.pb3().set_bit());

    // set port 1 (LED) on
    portb.portb.write(|w| w.pb1().set_bit());

    // do nothing forever
    loop {
        let status_register = peripherals.CPU.mcusr.read().bits();
        // Check if Global Interrupt Enable is set
        let global_interrupt_enable_bit = status_register.bitand(0x80) != 0;

        portb.portb.write(|w| w.pb3().bit(global_interrupt_enable_bit));

        let counter = unsafe { COUNTER };
        let led_on = counter < u16::MAX / 2;

        portb.portb.write(|w| w.pb1().bit(led_on));
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
