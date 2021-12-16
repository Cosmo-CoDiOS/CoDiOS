//! This is the main firmware image for the CoDiOS firmware.
#![cfg_attr(any(target_arch = "arm"), no_std)]
#![cfg_attr(any(target_arch = "arm"), no_main)]
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

extern crate codios_firmware as firmware;
#[cfg(target_arch = "arm")]
extern crate panic_semihosting as _;

#[cfg(target_arch = "arm")]
use cortex_m_rt::entry;
#[cfg(target_arch = "arm")]
use firmware::kernel::kernel_main;

#[cfg(target_arch = "arm")]
#[entry]
#[cfg(target_arch = "arm")]
fn main() -> ! {
    kernel_main();
}
