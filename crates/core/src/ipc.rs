/// IPC module for Core Process.
/// Safely communicates with UI Process and Service Process.

pub struct IpcServer;

impl IpcServer {
    pub fn new() -> Self {
        Self
    }

    pub fn listen(&self) {
        // Listen for IPC messages from UI/Service
    }

    pub fn handle_message(&self, _msg: mudfish_dns_common::ipc::Message) {
        // Execute Start/Stop, Status, Purge Cache commands
    }
}
