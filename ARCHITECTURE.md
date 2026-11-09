# WORKSPACE: Architecture & Technical Design Document

This document defines the architectural principles, system components, data flow, domain models, and security boundaries of **WORKSPACE**.

---

## 1. Architectural Philosophy

1. **Rust as the Core Engine**: All business logic, scheduling algorithms, skill mastery evaluations, database access, and markdown generation live in Rust. The frontend is a presentation and interaction layer.
2. **Offline-First & Local-First**: The desktop application runs completely self-contained with embedded SQLite. No internet connection or cloud service is required for basic curriculum management, focus sessions, and task scheduling.
3. **Multi-User Cloud Ready**: The same core logic powers `workspace-server` via Axum and PostgreSQL, enforcing strict multi-tenant isolation.
4. **Model-Agnostic AI**: AI providers (Gemini, OpenAI, Claude, local Ollama) are interchangeable through a unified trait with structured output validation.
5. **No Visual Mocks**: A feature is complete only when backed by transactional persistence and verifiable unit/integration tests.

---

## 2. High-Level System Architecture

```text
                               +----------------------------------------+
                               |     Clients (Desktop / Web / Ext)      |
                               +---+--------------------+-----------+---+
                                   |                    |           |
               IPC Commands (Tauri)|                    |HTTP/SSE   |Local Loopback
                                   v                    v           v
+----------------------------------+---+      +---------+-----------+----------------+
|          apps/desktop                |      |           crates/workspace-server    |
| - Tauri v2 Native Host               |      | - Axum Web Framework                 |
| - Native Tray & Idle Detection Hook  |      | - JWT & Tenant Context Middleware    |
| - Embedded SQLite Pool               |      | - REST & SSE Event Handlers          |
+----------------------------------+---+      +---------------------+----------------+
                                   |                                |
                                   +----------------+---------------+
                                                    |
                                                    v
                               +--------------------+-------------------+
                               |         crates/workspace-core          |
                               | - Schedule Timeline Projection Engine  |
                               | - Curriculum & Dependency Graph Engine |
                               | - Multi-factor Skill Mastery Engine    |
                               | - Markdown Journal Compiler            |
                               | - Deep Work & Focus Scorer             |
                               +--------------------+-------------------+
                                                    |
                     +------------------------------+-------------------------------+
                     |                                                              |
                     v                                                              v
+--------------------+-------------------+                    +---------------------+-----------------+
|         crates/workspace-db            |                    |          crates/workspace-ai          |
| - Abstract Storage Traits              |                    | - Trait AiProvider                    |
| - SQLite Repository (Local Desktop)    |                    | - Gemini, OpenAI, Claude, Ollama      |
| - PostgreSQL Repository (Cloud Server) |                    | - JSON Schema Structured Extraction   |
| - Versioned SQL Migrations             |                    | - Streaming Tutoring Pipelines        |
+--------------------+-------------------+                    +---------------------+-----------------+
                     |                                                              |
                     v                                                              v
       [ SQLite / PostgreSQL DB ]                                         [ External AI Endpoints ]
```

---

## 3. Storage Layer & Dual-Engine Strategy

To support both **desktop local-first** and **cloud multi-tenant** environments without duplicating database logic, `workspace-db` adopts the **Repository Pattern**:

### Storage Engine Abstraction
```rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DbError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn create(&self, user: &NewUser) -> Result<User, DbError>;
    async fn update_profile(&self, user_id: &UserId, profile: &UpdateProfile) -> Result<UserProfile, DbError>;
    async fn delete_account(&self, user_id: &UserId) -> Result<(), DbError>;
}

#[async_trait]
pub trait CurriculumRepository: Send + Sync {
    async fn list_for_user(&self, user_id: &UserId) -> Result<Vec<Curriculum>, DbError>;
    async fn get_with_topics(&self, user_id: &UserId, id: &CurriculumId) -> Result<Option<CurriculumDetails>, DbError>;
    async fn save_curriculum(&self, user_id: &UserId, curriculum: &CurriculumDetails) -> Result<(), DbError>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn list_daily_tasks(&self, user_id: &UserId, date: NaiveDate) -> Result<Vec<DailyTask>, DbError>;
    async fn update_task_status(&self, user_id: &UserId, task_id: &TaskId, status: TaskStatus) -> Result<(), DbError>;
    async fn log_focus_session(&self, user_id: &UserId, session: &FocusSession) -> Result<SessionId, DbError>;
}
```

### Concrete Engines
1. **SQLite Pool (`SqliteWorkspaceDb`)**:
   - Connection URL: `sqlite://~/.workspace/workspace.sqlite?mode=rwc`
   - PRAGMA tuning: `journal_mode = WAL`, `synchronous = NORMAL`, `foreign_keys = ON`.
2. **PostgreSQL Pool (`PostgresWorkspaceDb`)**:
   - Connection URL: `postgres://user:pass@host:5432/workspace_db`
   - Row-level tenant filtering: Every SQL statement explicitly binds `WHERE user_id = $1`.

---

## 4. Domain Logic & Core Algorithms (`workspace-core`)

### 4.1 Schedule Timeline Projections
Realistic estimation avoids burn-out through adjustable projection formulas:
$$\text{Projected Days} = \frac{\text{Total Curriculum Minutes} \times (1 + \text{Buffer Factor})}{\text{Daily Committed Minutes}} \times \frac{7}{\text{Available Days Per Week}}$$
Where:
- $\text{Buffer Factor} = 0.25$ (25% overhead for revision, difficulty spikes, and checkpoints).
- System alerts when daily hours exceed 6 hours to prevent unrealistic commitments.

### 4.2 Multi-Factor Skill Mastery
Mastery cannot be attained simply by opening a tutorial or marking a checkbox. The system evaluates three independent vectors:
1. **Conceptual Understanding (40%)**: AI Checkpoint performance (Q&A accuracy, reasoning explanation).
2. **Practical Implementation (40%)**: Code challenges, exercises, and linked Git commits.
3. **Retention Over Time (20%)**: Spaced repetition review performance after 7 and 30 days.

```text
Score >= 80% with Practical Evidence  => MASTERED
Score between 50% and 79%            => NEEDS_REVISION
Score < 50% or no Practical Evidence => NOT_MASTERED
```

---

## 5. AI Provider Architecture (`workspace-ai`)

The AI module is decoupled from specific vendors through the `AiProvider` trait:

```rust
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Return streamed tokens for interactive tutoring sessions
    async fn stream_tutoring(
        &self,
        context: &LearningContext,
        history: &[AiMessage],
        prompt: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, AiError>> + Send>>, AiError>;

    /// Extract structured curriculum or assess answers using strict JSON schema validation
    async fn structured_completion<T: DeserializeOwned + JsonSchema>(
        &self,
        system_instruction: &str,
        user_prompt: &str,
    ) -> Result<T, AiError>;
}
```

**Guardrails**:
- Fallback to offline heuristic mode if AI services are unreachable.
- Input and output validation using Serde JSON schemas to prevent hallucinated structures.
- User learning context injected dynamically (curriculum node, known knowledge gaps, previous attempt mistakes).

---

## 6. Security & Privacy Architecture

1. **Authentication**: Argon2id password hashing with unique random salts. JWT session tokens with 15-minute validity and cryptographically secure refresh tokens.
2. **Data Isolation**: Strictly scoped queries preventing cross-tenant leakage.
3. **Integration Privacy**:
   - VS Code and Chrome extensions do NOT send full DOM or files.
   - Activity tracking requires explicit user opt-in (`user_consents` table).
   - Pausing tracking stops all event recording immediately.
4. **Data Ownership**:
   - Full JSON & Markdown export capability (`GET /api/v1/user/export`).
   - Irreversible account deletion cascade (`DELETE /api/v1/user`).

---

## 7. Frontend Architecture (`apps/web`)

- Built with **React 19**, **TypeScript**, and **Tailwind CSS**.
- Design system inspired by high-density engineering environments (dark slate palette, accent amber/cyan status pills, clean sans-serif typography).
- Responsive layout adapting from compact desktop windows (Tauri webview) to full ultra-wide monitors.
- Seamless dual-mode API client: Uses `@tauri-apps/api` invoke calls when running inside desktop, and HTTP `fetch` when running as cloud web app.
