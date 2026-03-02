# Mudfish DNS (미꾸라지 DNS)

## Project Overview
Mudfish DNS is a DNS resolver designed to replace the default DNS resolver on various operating systems including Windows, macOS, Linux, iOS, and Android. It operates as a background service to process user DNS requests securely and efficiently.

**Key Technologies:**
- Rust (Cargo Workspace)
- Inter-Process Communication (IPC) for secure communication between components
- Modern DNS Upstream Protocols: UDP, TCP, DoH (DNS over HTTPS), DoT (DNS over TLS), DoQ (DNS over QUIC), ODoH (Oblivious DoH), and DNSSEC.

## Architecture
The application is structured as a multi-process architecture to separate concerns, improve stability, and manage permissions effectively. The workspace consists of the following crates:

1. **Core Process (`crates/core`)**: The heart of the application. It handles DNS packet redirection (via local server, WFP, Wintun, eBPF, etc.), routes queries to upstream servers, manages stability (failover, load balancing, proxies), handles caching and split DNS, and executes plugins (like ad-blocking or custom rules).
2. **UI Process (`crates/ui`)**: The user interface. It provides a dashboard for real-time statistics, start/stop controls, configuration management, and system tray integration. It communicates with the Core or Service processes to execute commands.
3. **Service Process (`crates/service`)**: A daemon running at the OS level (e.g., Windows Service, macOS launchd, Linux systemd). It manages the lifecycle of the Core Process (watchdog) and handles network configuration changes that require elevated administrative privileges.
4. **Common (`crates/common`)**: Shared libraries, configurations, and IPC definitions utilized across the different processes.

## Building and Running
As a standard Rust workspace, you can use Cargo for common development tasks:

- **Build the entire workspace:**
  ```bash
  cargo build
  ```
- **Run the components:**
  - Core: `cargo run --bin mudfish-dns-core`
  - UI: `cargo run --bin mudfish-dns-ui`
  - Service: `cargo run --bin mudfish-dns-service`
- **Run Tests:**
  ```bash
  cargo test
  ```

*(Note: Depending on the OS and the traffic interception method used, running the Core and Service processes may require elevated/administrator privileges.)*

## Development Conventions
- **Specifications:** Always checks `./specs/` directory and `*.md` files because it includes the specification of Mudfish DNS.  We need to obey when you try to implement.
- **Language:** The project is developed in **Rust**.
- **Coding Style:** As per user preference, keep the code free of unnecessary comments.
- **Modularity:** Adhere to the strict separation of concerns outlined in the architecture. Use `crates/common` for shared logic.
- **Communication:** Processes must communicate safely using the defined IPC interfaces rather than assuming shared state.
