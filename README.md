# Rust Thread Pool Web Server

This project is a simple multi-threaded web server implemented in Rust. It uses a thread pool to handle incoming HTTP requests concurrently. The server responds to two routes: `/` and `/sleep`. The `/sleep` route simulates a long-running task by delaying the response for 5 seconds.

---
## Code Overview

### Thread Pool
The thread pool (`threadpool.rs`) manages a fixed number of worker threads (`worker.rs`). Each worker thread listens for jobs (closures) and executes them.

### HTTP Server
The server listens for incoming TCP connections and dispatches them to the thread pool. The `handle_connection` function processes each request and sends an appropriate HTTP response.

### Routes
- **`GET /`**: Serves the `hello.html` file.
- **`GET /sleep`**: Simulates a long-running task by sleeping for 5 seconds before serving the `hello.html` file.
- **Other Routes**: Serves the `404.html` file.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/rust-threadpool-server.git
   cd rust-threadpool-server
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run
   ```

### Testing the Server

1. Open your web browser and navigate to:
   - `http://127.0.0.1:7878/` for the main page.
   - `http://127.0.0.1:7878/sleep` for the delayed response.

2. Use a tool like `curl` to test the server:
   ```bash
   curl http://127.0.0.1:7878/
   curl http://127.0.0.1:7878/sleep
   curl http://127.0.0.1:7878/unknown
   ```

