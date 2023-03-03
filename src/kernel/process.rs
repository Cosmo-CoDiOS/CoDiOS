#![allow(dead_code, clippy::upper_case_acronyms)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum ProcessKind {
    Init,
    Userspace(UserspaceProcessKind),
    Kernel(KernelProcessKind),
    Undefined,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum KernelProcessKind {
    Ipc,
    Mmu,
    Interrupt,
    Undefined,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum UserspaceProcessKind {
    Driver,
    Uart,
    Graphics,
    Rpc,
    Undefined,
}

impl Default for ProcessKind {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Default for KernelProcessKind {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Default for UserspaceProcessKind {
    fn default() -> Self {
        Self::Undefined
    }
}

pub type Pid = i8;

#[derive(Debug, Clone, Copy)]
pub struct Process {
    id: Pid,
    active: bool,
    zombie: bool,
    proc_type: ProcessKind,
}

impl Default for Process {
    fn default() -> Self {
        Self::new(ProcessKind::Undefined)
    }
}

impl Process {
    fn new(proc_type: ProcessKind) -> Self {
        Self {
            id: 1 + 1, // global count dracula
            active: true,
            zombie: false,
            proc_type,
        }
    }

    fn process_id(&self) -> ProcessID {
        self.id
    }

    fn process_type(&self) -> ProcessKind {
        self.proc_type
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_zombie(&self) -> bool {
        self.zombie
    }
}
