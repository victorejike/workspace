# WORKSPACE: Developer Setup & Contribution Guide

This guide describes how to configure your local development environment, build the project, run test suites, and debug **WORKSPACE**.

---

## 1. System Prerequisites

### Core Toolchains
- **Rust Toolchain**: 1.85+ (Stable). Recommended: `rustup default stable`.
- **Node.js**: v20 or v22 (LTS). Recommended: `nvm use 22`.
- **Package Manager**: `npm` (v10+) or `pnpm`.
- **Docker** (optional): For running PostgreSQL + pgvector locally.

### Linux System Dependencies (for Tauri v2 Desktop)
If developing on Ubuntu / Debian:
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

---

## 2. Getting Started

### 2.1 Clone and Environment Setup
```bash
# Clone the repository
git clone https://github.com/victorejike/workspace.git
cd workspace

# Setup local environment
cp .env.example .env
```

### 2.2 Install Frontend Dependencies
```bash
cd apps/web
npm install
cd ../..
```

---

## 3. Running WORKSPACE Locally

### 3.1 Running the Native Desktop Application (Tauri v2)
The desktop application automatically initializes an embedded SQLite database in `~/.workspace/workspace.sqlite`.
```bash
# Run with live-reloading UI and native Rust IPC
cargo run -p desktop
# Or via npm script:
npm run tauri dev
```

### 3.2 Running the Standalone Web Server (Axum)
If developing the multi-user cloud backend:
```bash
# Optional: Start local PostgreSQL if running server in cloud mode
docker compose up -d postgres

# Run database migrations
cargo run -p workspace-db --bin migrate

# Launch the Axum HTTP & WebSocket server
RUST_LOG=info,workspace=debug cargo run -p workspace-server
```

### 3.3 Running the Web Frontend in Browser
```bash
cd apps/web
npm run dev
# Accessible at http://localhost:5173
```

---

## 4. Running Test Suites

Automated tests are mandatory for all core modules, domain models, and storage repositories.

### 4.1 Rust Tests
```bash
# Run all workspace unit and integration tests
cargo test --workspace

# Run tests with output logging enabled
cargo test --workspace -- --nocapture

# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --workspace --all-targets -- -D warnings
```

### 4.2 Frontend Tests & Type Checking
```bash
cd apps/web
npm run typecheck
npm run lint
npm run test
```

---

## 5. Database Migrations

Database migrations are located in `crates/workspace-db/migrations/`.

### Writing Migrations
- Standard ANSI SQL should be used to guarantee compatibility with both SQLite and PostgreSQL.
- Migrations are versioned sequentially:
  `0001_initial_schema.sql`, `0002_curriculum_and_tasks.sql`, etc.

---

## 6. Project Conventions

- **Module Independence**: Do not add dependencies from `workspace-core` to `workspace-server`. Domain logic must remain independent of web frameworks.
- **Error Handling**: Use `thiserror` for library crates (`workspace-core`, `workspace-db`, `workspace-ai`) and `anyhow` for applications (`workspace-server`, `desktop`).
- **No Panics**: Never use `unwrap()` or `expect()` in production code paths. Propagate errors via `Result<T, E>`.
- **Logging**: Use the `tracing` crate (`tracing::info!`, `tracing::warn!`, `tracing::error!`).
