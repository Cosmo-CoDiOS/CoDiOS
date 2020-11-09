//! This is the core of `CoDirs`; the microkernel for the RTOS.
//! It is split out into different crates.
#![no_std]
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
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![allow(unstable_features)]
#![feature(alloc_error_handler)]

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

/// Main kernel entrypoint for `CoDirs`.
pub fn kernel_main() -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

pub mod drivers;
pub mod ipc;
pub mod rpc;
