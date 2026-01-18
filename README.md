# AXON: The Neural System Controller

![Status](https://img.shields.io/badge/Status-Alpha-orange) ![License](https://img.shields.io/badge/License-MIT-blue)

**AXON** is a "Meta-Operating System" — a unified, intelligent control plane that sits above the kernel but below the graphical interface. It orchestrates the entire developer lifecycle (monitoring, updating, debugging, scaffolding, and searching) through a single, "Cyberpunk-styled" Terminal User Interface (TUI).

## The "Deca-Stack" Architecture

AXON leverages **11 distinct technologies**, using the best tool for each specific organ of the system:

| Component | Language | Role |
| :--- | :--- | :--- |
| **The Hub** | **Rust** | **The Central Nervous System** (UI & Orchestration) |
| **The Muscle** | **Go** | **The Executor** (Heavy I/O, Networking) |
| **The Cortex** | **Python** | **The Brain** (AI & Logic Reasoning) |
| **The Senses** | **C++** | **The Ears** (Local Voice Processing) |
| **The Memory** | **C#** | **The Archivist** (Structured Logs/DB) |
| **The Bridge** | **Node.js** | **The Eyes** (Remote Dashboard) |
| **The Config** | **Lua** | **The Reflexes** (User Scripting) |
| **The Index** | **Java** | **The Librarian** (Search Indexing) |
| **The Builder** | **Ruby** | **The Architect** (Project Scaffolding) |
| **The Logic** | **Haskell** | **The Validator** (Safety Logic) |
| **The Native** | **Swift** | **The Face** (macOS Native UI) |

## Quick Start

### Prerequisites
- **Rust**: `cargo --version` (1.75+)
- **Go**: `go version` (1.21+)
- **Protoc**: `protoc --version` (3.0+)

### 1. Build
```bash
# Build the Go Agent
cd axon-agent
go build -o ../bin/axon-agent.exe main.go

# Build the Rust Core
cd ../axon-core
cargo build --release
cp target/release/axon-core.exe ../bin/axon.exe
```

### 2. Run
```bash
cd ..
./bin/axon.exe
```

## Contributing

This project uses a "Hub-and-Spoke" architecture communicating via **gRPC**. 
- **Core Logic**: `axon-core/` (Rust)
- **Agent Logic**: `axon-agent/` (Go)
- **Shared Contracts**: `proto/` (Protobuf)

## License

MIT
