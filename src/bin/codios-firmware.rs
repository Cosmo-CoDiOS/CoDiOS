//! This is the main firmware image for the `CoDiOS` firmware.
#![cfg_attr(target_arch = "arm", no_main, no_std)]
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

use {defmt_rtt as _, panic_probe as _};

#[cfg_attr(all(target_arch = "arm", feature = "firmware"), cortex_m_rt::entry)]
#[cfg(all(target_arch = "arm", feature = "firmware"))]
fn main() -> ! {
    loop {}
}
