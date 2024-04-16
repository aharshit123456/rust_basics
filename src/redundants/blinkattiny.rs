use core::ptr::write_volatile;
use ore::ptr::read_volatile;

const DDRB: *mut u8 = 0x17 as *mut u8;
const PORTB: *mut u8 = 0x18 as *mut u8;

fn delay(mut time: i32) {
    while time > 0 {
        time -= 1;
    }
}

fn main() {
    unsafe {
        write_volatile(DDRB, read_volatile(DDRB) | (1 << 1));
        loop {
            write_volatile(PORTB, read_volatile(PORTB) ^ (1 << 1));
            delay(10000);
        }
    }
}

/*
for c this could would have used
use avr-gcc -mmcu attiny85 for compilation to .out, avr-objectcopy for hex intel format and avrdude for uploading
TODO: how to compile .rs for avr-rust compilation
*/
