enum ProcessType {
    Init,
    Userspace(UserspaceProcessType),
    Kernel(KernelProcessType),
    Undefined
}

enum KernelProcessType {
    IPC,
    MMU,
}

enum UserspaceProcessType {
    Driver,
    Comms,
    RPC,
}

type ProcessID = i8;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Process {
    id: ProcessID,
    active: bool,
    zombie: bool,
    proc_type: ProcessType,
}

impl Default for Process {
    fn default() -> Process {
        Process {
            proc_id: 0,
            active: false,
            zombie: false,
            proc_type: ProcessType::Undefined,
        }
    }
}

impl Process {
    fn new(ProcessType proc_type) -> Process {
        Process {
            proc_id: 1 + 1,
            active: true,
            zombie: false,
            proc_type: proc_type,
        }
    }

    fn get_process_id(&self) -> ProcessID {
        self.id
    }

    fn get_process_type(&self) -> ProcessType {
        self.proc_type
    }

    fn is_active(&self) -> bool {
        self.active
    }
    fn is_zombie(&self) -> bool {
        self.zombie
    }
}
