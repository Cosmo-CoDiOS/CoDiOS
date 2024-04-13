//! This is the x86_64/aarch64-native emulator for the `CoDiOS` firmware.
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

use embassy_executor::Spawner;
use codios::kmain;

#[cfg(all(any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"))]
use log::*;

#[cfg(all(not(feature = "firmware"), any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"))]
fn main() {
    println!("Emulator starting...");
}
