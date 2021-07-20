//! This is the core of `CoDirs`; the microkernel for the RTOS.
//! It is split out into different crates.

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

/// Main kernel entrypoint for `CoDirs`.
pub fn kernel_main() -> ! {
    loop {}
}

/// Out of Memory (OOM) handler.
/// Currently this just hangs.
/// We should probably cleanup after this, display a recovery message, and then reboot.
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

mod drivers;
mod hal;
mod ipc;
mod protocol;
mod rpc;

use protocol::ProtocolCommands;
