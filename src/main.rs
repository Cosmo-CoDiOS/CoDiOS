#![no_std]
#![no_main]

extern crate core;

use core::panic::PanicInfo;

// Change this to display an error on external display, and create Android notification.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
