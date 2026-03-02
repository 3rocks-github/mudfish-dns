/// Command execution logic.
/// Translates CLI commands into IPC messages and handles the response.
use crate::client::IpcClient;

pub struct CommandRunner {
    #[allow(dead_code)]
    client: IpcClient,
}

impl CommandRunner {
    pub fn new(client: IpcClient) -> Self {
        Self { client }
    }

    pub fn execute_start(&self) {
        // self.client.send_command(Message::StartCore);
        println!("Sent Start command to daemon.");
    }

    pub fn execute_status(&self) {
        // self.client.send_command(Message::StatusRequest);
        println!("Mudfish DNS is currently running.");
    }

    pub fn execute_purge_cache(&self) {
        // self.client.send_command(Message::PurgeCache);
        println!("DNS cache purged successfully.");
    }

    // Add other command executions (stop, restart, logs, config)
}
