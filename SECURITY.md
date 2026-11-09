# WORKSPACE: Security Policy & Threat Model

Security and data privacy are core architectural priorities of **WORKSPACE**. Because this environment handles personal learning records, external tokens, and developer activity, strict security controls are enforced across all layers.

---

## 1. Reporting a Vulnerability

If you discover a security vulnerability in WORKSPACE, please do not disclose it publicly.

- **Email**: `security@workspace.dev` (or create a private GitHub Security Advisory).
- **Response SLA**: We acknowledge receipt of vulnerability reports within 48 hours and provide a remediation timeline within 7 days.

---

## 2. Authentication & Credential Management

- **Password Storage**: Passwords are never stored in plaintext. They are hashed using **Argon2id** with high-memory parameters and unique per-user salts.
- **Session Tokens**: API authentication uses short-lived JSON Web Tokens (15-minute expiration) paired with cryptographically secure, rotatable refresh tokens.
- **Cookie Security**: In web deployments, tokens are stored strictly in `HttpOnly`, `Secure`, and `SameSite=Strict` cookies to mitigate Cross-Site Scripting (XSS) and Cross-Site Request Forgery (CSRF).
- **Desktop Storage**: In the Tauri desktop application, sensitive tokens are stored using the OS-level credential manager (Secret Service / libsecret on Linux, Keychain on macOS, Credential Manager on Windows).

---

## 3. Multi-Tenant Isolation

For multi-user cloud deployments:
- **Tenant Scoping**: All database queries must enforce explicit tenant scoping (`WHERE user_id = $1`). Direct queries without tenant parameters are forbidden in repository implementations.
- **Object-Level Authorization**: All requests to access curricula, tasks, notes, or AI conversations verify ownership prior to returning or mutating data.
- **Separate Secret Storage**: Third-party integration credentials (e.g., GitHub tokens) are stored in an encrypted table using **AES-256-GCM** encryption keys managed via environment secrets.

---

## 4. Privacy & Activity Tracking Boundaries

1. **No Silent Tracking**: External tracking (via VS Code or Chrome extensions) is strictly opt-in and disabled by default.
2. **Data Minimization**:
   - The browser extension only evaluates URL domains against a user-configurable whitelist of educational sites (documentation, tutorials). It **never** captures webpage content, DOM elements, passwords, or personal messages.
   - The editor extension only associates active repository names and project open/close timestamps with selected learning tasks. It **never** logs keystrokes or transmits source code.
3. **Immediate Pause Control**: Users can pause all activity tracking with a single click at any time.

---

## 5. Data Ownership, Export & Deletion

- **Export**: Users can download a complete export of their curricula, focus sessions, journal entries, and assessment records in standard JSON and Markdown formats.
- **Account Erasure**: In accordance with privacy regulations (including GDPR), requesting account deletion executes an immediate cascading purge of all user records, sessions, and encrypted integration secrets.
