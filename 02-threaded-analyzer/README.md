# threaded-analyzer

This project demonstrates a fully-featured **thread pool and concurrency system in Rust**,
designed according to **Clean Architecture** principles.
It showcases advanced Rust concepts covered in Session 2 of the "Ultimate Rust" course.

## What does the app do?

The application is a **Parallel Data Processor**:
- Accepts a list of numbers from CLI input or a file.
- Splits the work into chunks and distributes it to worker threads.
- Uses a thread pool to execute heavy computations (e.g., primality check, Fibonacci).
- Synchronizes progress using `AtomicUsize` counters.
- Stores results in a shared `Mutex<Vec<T>>`.
- Reads configuration (e.g., logging threshold, thread count) from a global `once_cell`.
- Demonstrates safe concurrent reading with `RwLock` and robust error handling for poisoned locks.
- Implements thread parking/unparking to save resources when no work is available.
- Exchanges tasks and results via `mpsc` channels.
- Allows sending arbitrary functions (`Box<dyn FnOnce()>`) as jobs to the pool.

At the end of execution, the CLI outputs processed results in sorted order along with
thread usage statistics.

## Architecture

- **core/**: Domain entities and abstract traits (e.g., `Job`, `ThreadPool`).
- **service/**: Business logic orchestrating how jobs are queued and processed.
- **infrastructure/**: Concrete implementations (thread pool, synchronization, messaging).
- **apps/cli/**: Command-line interface to start jobs and view results.

## Rust Patterns Used

- **Command Pattern** – jobs encapsulated as executable commands.
- **Builder Pattern** – flexible thread configuration (`thread::Builder`).
- **Dependency Injection** – separate core interfaces from infrastructure implementations.
- **Singleton Pattern** – global configuration with `once_cell`.

## Getting Started

```bash
# Clone repository
git clone https://github.com/VChouliak/rust-learning-lab
cd rust-learning-lab/02-threaded-analyzer

# Build the CLI app
cargo build

# Run with a list of numbers
cargo run --bin CLI -- 10 25 42 101 256
