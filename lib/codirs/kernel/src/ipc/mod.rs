//! This crate handles IPC between kernel components.
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

/// This enum holds various result states passed between different processes in the microkernel
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum IPCResult {
    /// This is a `ACKNOWLEDGE` result, which is when a IPC call was received and processed by a
    /// process.
    RESULT_ACK,
    /// This is a `REJECT` result, which is when a IPC call to a process was rejected.
    RESULT_REJ,
    /// This is a `ILLEGAL` result, which is when a IPC call to a process was determined to be
    /// unsupported/illegal.
    RESULT_IGL,
    /// This is a `NOP` result, which is when a IPC call to a process does not return any result.
    RESULT_NOP,
}
