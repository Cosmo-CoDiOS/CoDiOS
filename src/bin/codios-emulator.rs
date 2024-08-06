//! This is the x86_64/aarch64-native emulator for the `CoDiOS` firmware.
#![feature(impl_trait_in_assoc_type)]
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
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
#[cfg_attr(all(not(feature = "firmware"), any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"), embassy_executor::main)]
async fn main(spawner: Spawner) {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .format_timestamp_millis()
        .init();

    info!("Starting CoDiOS emulator...");
    warn!("In emulation mode, expect STM32-specific functionality to be missing!");

    spawner.spawn(kmain()).unwrap();
}
