# Building GOAT From Source

This guide covers compiling and running GOAT directly from its source code.

## Prerequisites

1. Install [Rust](https://rustup.rs/).
2. Install [Node.js](https://nodejs.org/) (Version 18+).
3. Ensure you have C++ build tools installed on your platform (e.g., `build-essential` on Ubuntu, Xcode Command Line Tools on macOS).

## 1. Clone the Repository

```bash
git clone https://github.com/ziuus/GOAT.git
cd GOAT
```

## 2. Compile the Core Backend

The core of GOAT (TUI, Headless CLI, Daemon API) is written in Rust.

To build in release mode (recommended for performance):
```bash
cargo build --release
```

The resulting binary will be at `target/release/goat`. You may want to add this to your system `PATH`.

## 3. Build the Web Dashboard

The web dashboard is a Next.js application that interfaces with the GOAT daemon.

```bash
cd apps/dashboard
npm install
npm run build
```

## 4. Build the Desktop App (Tauri)

If you prefer a desktop application rather than running the daemon and dashboard separately:

```bash
cd apps/desktop
npm install
npm run tauri build
```

## Running the Components

**Run the CLI/TUI:**
```bash
cargo run --release -- tui
```

**Run the Daemon:**
```bash
cargo run --release -- daemon start
```

**Run the Dashboard (Dev Mode):**
Make sure the daemon is running in another terminal window first.
```bash
cd apps/dashboard
npm run dev
```

## Running Tests

To verify your build:
```bash
cargo test
```
