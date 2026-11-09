# WORKSPACE

> **An AI-Powered Personal Learning, Focus and Developer Productivity Environment**  
> *"Know what to learn. Know what to do. Understand what you've achieved. Always know what's next."*

---

[![CI](https://github.com/victorejike/workspace/actions/workflows/ci.yml/badge.svg)](https://github.com/victorejike/workspace/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.98+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-v2-24C8D8.svg)](https://v2.tauri.app)
[![React](https://img.shields.io/badge/React-19-61DAFB.svg)](https://react.dev)

---

## 1. Vision & Purpose

**WORKSPACE** is an intelligent personal engineering environment designed to help developers, computer science students, aspiring AI engineers, and self-taught learners build lasting technical competence.

Most learners struggle not from a lack of educational materials, but from a lack of execution structure:
- **Curriculum Drift**: Jumping randomly between technologies without completing foundations.
- **Illusion of Competence**: Confusing time spent watching videos or reading tutorials with actual understanding.
- **Unreliable Records**: Having no unified, verifiable journal of practical projects, code written, and conceptual assessments.
- **Accountability Failure**: Losing momentum when work or personal commitments fluctuate.

WORKSPACE transforms unstructured learning into a measurable, structured, and focused engineering routine. It runs as both a native **desktop application** (Tauri v2 + SQLite) and a **multi-user cloud platform** (Axum + PostgreSQL).

---

## 2. Core Modules

| Module | Purpose |
| :--- | :--- |
| **A. Accounts & Profiles** | Multi-tenant isolation, career goals, timezone scheduling, and data export/deletion controls. |
| **B. Curriculum Intelligence** | Ingests PDF, DOCX, Markdown, or topics into structured roadmaps with dependencies and prerequisites. |
| **C. Personalized Planner** | Calculates realistic timelines accounting for revision buffers, rest days, and daily time commitments. |
| **D. Daily Workspace** | High-density developer dashboard featuring deep work tracking, attention quality, and chronological daily timeline. |
| **E. AI Learning Mentor** | Context-aware tutoring with strict checkpoint assessments (`MASTERED`, `NEEDS_REVISION`, `NOT_MASTERED`). |
| **F. Focus & Activity Tracker**| Transparent deep work session recording with native idle detection. |
| **G. IDE Integrations** | VS Code companion extension for linking coding sessions directly to learning tasks. |
| **H. GitHub Integration** | Auto-publishes user-approved Markdown learning journals (`workspace-learning/`) to GitHub repositories. |
| **I. Chrome Extension** | Manifest V3 extension for tracking technical documentation and tutorial reading sessions. |
| **J. Verified Resource Discovery**| Discovers up-to-date official docs, RFCs, and release notes with explicit verification timestamps. |
| **K. Progress Journal** | Searchable daily summaries generated automatically with knowledge gap audits. |
| **L. Reminders & Alerts** | Cross-platform notifications, quiet hours management, and daily review prompts. |

---

## 3. Technology Stack

- **Core & Backend Engine**: Rust (Cargo workspace, Tokio, Axum, SQLx).
- **Desktop Application**: Tauri v2 (native windowing, system tray, native idle detection).
- **Frontend Architecture**: React 19, TypeScript, Tailwind CSS, Lucide Icons, Vite.
- **Storage Strategy**:
  - **Local Desktop**: Embedded SQLite (zero daemon setup, fully functional offline).
  - **Cloud Multi-Tenant**: PostgreSQL with `pgvector` for semantic topic retrieval.
- **AI Integration**: Model-agnostic abstraction supporting Google Gemini, OpenAI, Anthropic Claude, and local Ollama.

---

## 4. Repository Structure

```text
workspace/
├── crates/
│   ├── workspace-types/       # Domain structs, enums, DTOs, Serde schemas
│   ├── workspace-core/        # Scheduling, mastery, markdown generation engines
│   ├── workspace-db/          # Dual SQLite/PostgreSQL migrations & repositories
│   ├── workspace-ai/          # AI provider traits, prompt definitions, schema guards
│   └── workspace-server/      # Multi-user Axum HTTP & WebSocket service
├── apps/
│   ├── desktop/               # Native Tauri v2 desktop application
│   └── web/                   # Modern React + Tailwind CSS dashboard UI
├── integrations/
│   ├── vscode-extension/      # VS Code workspace companion
│   └── chrome-extension/      # Chrome Manifest V3 tracker
├── docs/                      # Architectural specs and documentation
├── .env.example               # Reference environment variables
└── Cargo.toml                 # Root Cargo workspace manifest
```

---

## 5. Quickstart & Local Setup

### Prerequisites
- **Rust Toolchain**: 1.85+ (`cargo`, `rustc`).
- **Node.js**: 20+ with `npm` or `pnpm`.
- **Linux Packages** (for desktop builds): `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`.

### Running the Desktop Application
```bash
# Clone the repository
git clone https://github.com/victorejike/workspace.git
cd workspace

# Copy environment template
cp .env.example .env

# Install web dependencies
npm install

# Run the Tauri desktop app in development mode
npm run tauri dev
```

### Running Backend Server & Tests
```bash
# Run all Rust workspace tests
cargo test --workspace

# Run the Axum API server
cargo run -p workspace-server
```

---

## 6. Architecture & Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) — Comprehensive technical design, domain models, and storage engine abstractions.
- [DEVELOPMENT.md](DEVELOPMENT.md) — Contributor onboarding, local setup, test execution, and debugging.
- [ROADMAP.md](ROADMAP.md) — Phased milestones from Foundation to Production.
- [SECURITY.md](SECURITY.md) — Security principles, tenant isolation, and vulnerability reporting.
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines, code style, and PR workflow.

---

## 7. License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
