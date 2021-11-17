enum ProcessType {
    Init,
    Userspace(UserspaceProcessType),
    Kernel(KernelProcessType)
}

enum KernelProcessType {
    IPCHandler,
    MMU
}

enum UserspaceProcessType {
    Driver,
    Comms,
    RPC,
}

type ProcessID = i8;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Process {
    p_id: ProcessID,
    active: bool,
    zombie: bool,
}

impl Default for Process {
    fn default() -> Process {
        Process {
            p_id: 0,
            active: false,
            zombie: false,
        }
    }
}

impl Process {
    fn is_active(&self) -> bool {
        self.active
    }
    fn is_zombie(&self) -> bool {
        self.zombie
    }
}
