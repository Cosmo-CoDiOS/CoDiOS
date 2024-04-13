//! This is the main firmware image for the `CoDiOS` firmware.
#![cfg_attr(all(target_arch = "arm", feature = "firmware", not(feature = "emulator")), no_main, no_std)]
#![feature(impl_trait_in_assoc_type)]
#![deny(
    warnings,
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

#[cfg(all(feature = "firmware", target_arch = "arm"))]
use defmt::*;
#[cfg(all(feature = "firmware", target_arch = "arm"))]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use codios::kmain;

#[cfg(all(target_arch = "arm", feature = "firmware", not(feature = "emulator")))]
fn main() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
