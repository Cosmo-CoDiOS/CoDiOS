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

use embassy_time::Timer;

#[cfg(all(any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"))]
pub mod emulator;

#[cfg(all(any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"))]
use log::*;
#[cfg(all(feature = "firmware", target_arch = "arm"))]
use defmt::*;
#[cfg(all(feature = "firmware", target_arch = "arm"))]
use {defmt_rtt as _, panic_probe as _};

#[cfg(not(any(feature = "firmware", feature = "emulator")))]
compile_error!("No flag specified which tells us to build the emulator or firmware! Unable to continue.");

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "x86_64",
    target_arch = "aarch64"
)))]
compile_error!("Unsupported target specified, refusing to build.");

#[cfg(all(any(target_arch = "x86_64", target_arch = "aarch64"), feature = "emulator"))]
pub mod emulator;
