/// Common IPC Messages shared among Core, UI, and Service Processes.

pub enum Message {
    StartCore,
    StopCore,
    RestartCore,
    StatusRequest,
    StatusResponse { is_running: bool, errors: Option<String> },
    PurgeCache,
    StatsUpdate {
        queries: u64,
        latency: std::time::Duration,
    },
}
