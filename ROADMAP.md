# WORKSPACE: Development Roadmap & Milestones

This document tracks progress, deliverables, and acceptance criteria across each development phase.

---

## Progress Overview

- [x] **Phase 0: Repository Audit, Architecture & Scaffolding** *(Completed)*
- [ ] **Phase 1: Functional Foundation (Core Engine & Desktop Shell)** *(Current Milestone)*
- [ ] **Phase 2: Focus & Progress Engine**
- [ ] **Phase 3: AI Learning Intelligence**
- [ ] **Phase 4: Current Resource Discovery**
- [ ] **Phase 5: GitHub Integration**
- [ ] **Phase 6: External Tracking (VS Code & Chrome Extensions)**
- [ ] **Phase 7: Communication & Reminders**
- [ ] **Phase 8: Production Readiness & Hardening**

---

## Detailed Milestone Breakdown

### Phase 0: Repository Audit & Architecture
- [x] Repository assessment and environment audit (`rustc 1.98`, `node 22`, GTK/WebKit2GTK).
- [x] Architecture design (`ARCHITECTURE.md`).
- [x] Developer onboarding documentation (`DEVELOPMENT.md`).
- [x] Security policies and threat model (`SECURITY.md`).
- [x] Contribution guidelines (`CONTRIBUTING.md`).
- [x] Environment configuration template (`.env.example`).
- [x] Multi-crate Cargo workspace manifest.

### Phase 1: Functional Foundation
- [ ] Implement `workspace-types` (Entities: User, Curriculum, Module, Topic, Task, Schedule).
- [ ] Implement `workspace-db` with SQLite connection pool and initial migrations:
  - `0001_initial_schema.sql` (users, profiles, preferences, curricula, modules, topics, tasks).
- [ ] Implement User account service with Argon2id password hashing.
- [ ] Implement `workspace-core` Planner & Schedule Projection engine.
- [ ] Build desktop shell (`apps/desktop` via Tauri v2) with React 19 frontend:
  - Top Navigation (Overview, Planner, Projects, Goals, Profile).
  - "Stay on Track Today" dashboard card.
  - Interactive Daily Timeline (8 AM - 8 PM).
  - Curriculum editor & topic checklist.
- **Acceptance Criteria**: A user can create a profile, set learning commitments, define a curriculum, and view generated daily tasks on the timeline.

### Phase 2: Focus & Progress Engine
- [ ] Implement Deep Work timer engine in `workspace-core` (Start, Pause, Resume, Complete).
- [ ] Native OS idle detection integration via Tauri hooks.
- [ ] Attention Quality and Focus Stability scoring algorithms.
- [ ] Daily Progress Journal with automated Markdown generation.
- **Acceptance Criteria**: A user can track a focus session, close the application, reopen it, and recover their session state and focus metrics without data loss.

### Phase 3: AI Learning Intelligence
- [ ] Model-agnostic `AiProvider` trait supporting Gemini, OpenAI, Claude, and local Ollama.
- [ ] Curriculum Analyzer for parsing unstructured syllabi (PDF, Markdown, TXT) into roadmaps.
- [ ] Context-aware AI Tutoring with streaming responses.
- [ ] Interactive Checkpoints: 5-question conceptual and code assessments.
- [ ] Evaluation engine with classification: `MASTERED`, `NEEDS_REVISION`, `NOT_MASTERED`.
- **Acceptance Criteria**: A user can complete an assessment checkpoint, receive actionable explanations, and see their topic mastery state update accordingly.

### Phase 4: Current Resource Discovery
- [ ] Resource registry for official documentation (Rust, Tauri, MDN, PyTorch, etc.).
- [ ] Changelog & release tracker for detecting deprecated APIs and framework updates.
- [ ] Resource ranking by reliability, freshness, and topic relevance.
- **Acceptance Criteria**: A user querying resources for a curriculum topic receives verified official links accompanied by explicit verification dates.

### Phase 5: GitHub Integration
- [ ] GitHub OAuth / Personal Access Token management with AES-256 encrypted storage.
- [ ] Automated provisioning of `workspace-learning/` repository structure.
- [ ] Progress journal Markdown generator and diff review.
- [ ] User-authorized commit & push mechanism.
- **Acceptance Criteria**: A user can review generated daily Markdown reports and commit them to their GitHub learning repository.

### Phase 6: External Tracking
- [ ] Local loopback API for authenticated editor/browser communication.
- [ ] VS Code companion extension for tracking active coding sessions.
- [ ] Chrome Manifest V3 extension for tracking technical reading sessions.
- [ ] Explicit user consent management for all external tracking.
- **Acceptance Criteria**: Coding and documentation activities are transparently logged and associated with active daily tasks.

### Phase 7: Communication & Reminders
- [ ] Cross-platform desktop notifications.
- [ ] Daily schedule reminders and quiet hours enforcement.
- [ ] Optional email digests.
- [ ] Meta WhatsApp Business API integration hooks.
- **Acceptance Criteria**: Users receive configurable reminders for scheduled tasks and upcoming checkpoints.

### Phase 8: Production Readiness & Hardening
- [ ] Comprehensive unit and integration test coverage across all crates.
- [ ] Docker compose configuration for multi-user cloud deployment.
- [ ] GitHub Actions CI pipeline for continuous linting, testing, and multi-platform binary packaging.
- [ ] Security audit: Tenant boundary verification and rate-limiting validation.
