//! This crate handles IPC between kernel components.

/// This enum holds various result states passed between different processes in the RTOS.
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum IPCResult {
    /// This is a `ACKNOWLEDGE` result, which is when a IPC call was received and processed by a
    /// process.
    RESULT_ACK = 0,
    /// This is a `REJECT` result, which is when a IPC call to a process was rejected.
    RESULT_REJ = 1,
    /// This is a `ILLEGAL` result, which is when a IPC call to a process was determined to be
    /// unsupported/illegal.
    RESULT_IGL = 2,
    /// This is a `NOP` result, which is when a IPC call to a process does not return any result.
    RESULT_NOP = 3,
}
