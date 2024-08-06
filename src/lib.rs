//! Base crate.
#![cfg_attr(
    any(target_arch = "arm", feature = "firmware"),
    feature(alloc_error_handler), allow(unstable_features), no_std
)]
#![feature(impl_trait_in_assoc_type)]
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[cfg(not(any(feature = "firmware", feature = "emulator")))]
compile_error!("No flag specified which tells us to build the emulator or firmware! Unable to continue.");

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "x86_64",
    target_arch = "aarch64"
)))]
compile_error!("Unsupported target specified, refusing to build.");

#[cfg(all(feature = "firmware", feature = "emulator", any(target_arch = "arm", target_arch = "aarch64", target_arch = "x86_64")))]
compile_error!("Cannot build emulator and firmware at the same time.");

#[cfg(all(not(any(target_arch = "x86_64", target_arch = "aarch64")), feature = "emulator"))]
compile_error!("Emulator is only supported on x86_64 and aarch64.");

#[cfg(all(any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"))]
pub mod emulator;

#[cfg(all(any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"))]
use log::*;
#[cfg(all(feature = "firmware", target_arch = "arm"))]
use defmt::*;
#[cfg(all(feature = "firmware", target_arch = "arm"))]
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
pub async fn kmain() {
    info!("Looping NOW. (kmain)");
    loop {
        emulator::emulator_main().await.unwrap();
    }
}
