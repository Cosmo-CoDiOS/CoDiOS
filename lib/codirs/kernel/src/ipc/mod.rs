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
pub enum IPCResult {
    /// This is a 'ACKNOWLEDGE' result, which is when a IPC call was received and processed by a process.
    #[allow(non_camel_case_types)]
    RESULT_ACK,
    /// This is a `REJECT` result, which is when a IPC call to a process was rejected.
    #[allow(non_camel_case_types)]
    RESULT_REJ
}
