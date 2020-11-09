//! The main firmware for the CoDi-rs RTOS
#![no_std]
#![no_main]
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
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

extern crate codirs_kernel as kernel;
extern crate panic_semihosting as _;

use cortex_m_rt::entry;
use kernel::kernel_main;
#[entry]
fn main() -> ! {
    kernel_main()
}
