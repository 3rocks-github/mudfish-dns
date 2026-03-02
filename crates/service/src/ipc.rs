/// IPC module for Service Daemon.
/// Safely communicates with UI Process.

pub struct IpcServer;

impl IpcServer {
    pub fn new() -> Self {
        Self
    }

    pub fn listen(&self) {
        // Listen for IPC messages from UI
    }

    pub fn handle_message(&self, _msg: mudfish_dns_common::ipc::Message) {
        // Execute Start/Stop, Status
    }
}
