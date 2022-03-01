//! Base crate.
#![cfg_attr(any(target_arch = "arm", feature = "firmware"), no_std)]
#![cfg_attr(
    any(target_arch = "arm", feature = "firmware"),
    allow(unstable_features)
)]
#![cfg_attr(
    any(target_arch = "arm", feature = "firmware"),
    feature(alloc_error_handler)
)]
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    //missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[cfg(feature = "emulator")]
#[cfg(target_arch = "arm")]
compile_error!(
    "Unable to build the emulator for the CoDi chip, this isn't supported!"
);

#[cfg(not(any(feature = "firmware", feature = "emulator")))]
compile_error!("No flag specified which tells us to build the emulator or firmware! Unable to continue.");

#[cfg(not(any(
    target_arch = "arm",
    target_arch = "x86_64",
    target_arch = "aarch64"
)))]
compile_error!("Unsupported target specified, refusing to build.");

#[cfg(all(target_arch = "arm", feature = "firmware"))]
pub mod kernel;

#[cfg(all(target_arch = "arm", feature = "firmware"))]
pub(crate) mod userspace;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[cfg(feature = "emulator")]
pub mod emulator;
