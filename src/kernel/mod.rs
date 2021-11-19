//! This is the core of `CoDiOS`.
//! It is split out into different modules.

use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

/// Main kernel entrypoint for `CoDiOS`.
pub fn kernel_main() -> ! {
    #[allow(clippy::empty_loop)] // temporary clippy avoidance
    loop {}
}

/// Out of Memory (OOM) handler.
/// Currently this just hangs.
/// We should probably cleanup after this, display a recovery message, and then reboot.
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    #[allow(clippy::empty_loop)] // temporary clippy avoidance
    loop {}
}

// prepare for future
// use super::userspace;
