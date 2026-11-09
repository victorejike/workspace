# Contributing to WORKSPACE

Thank you for your interest in contributing to **WORKSPACE**! We welcome contributions from developers, computer science educators, and open-source contributors of all experience levels.

---

## 1. Development Principles

- **Focus on Real Utility**: Every feature must solve a real learner problem. Avoid adding complexity without clear user value.
- **Rust First**: Core business logic, scheduling algorithms, and data access belong in Rust crates, not frontend components.
- **Test-Driven Rigor**: Pull requests introducing business logic or database migrations must include unit and integration tests.
- **No Hardcoded Data**: Never hardcode personal learning paths, names, credentials, or proprietary configurations.

---

## 2. Git Workflow & Branching

1. Fork the repository and create your branch from `main`:
   ```bash
   git checkout -b feat/curriculum-prerequisites
   ```
2. Keep commits atomic and use Conventional Commit messages:
   - `feat:` A new user-facing feature or domain capability.
   - `fix:` A bug fix in core logic, UI, or database queries.
   - `docs:` Documentation improvements or architecture updates.
   - `refactor:` Code reorganization without behavioral changes.
   - `test:` Adding or updating test suites.
   - `chore:` Dependency bumps, CI configuration, or build scripts.

---

## 3. Rust Code Standards

- **Formatting**: Run `cargo fmt --all` prior to committing.
- **Linting**: Your code must pass `cargo clippy --workspace --all-targets -- -D warnings` without warnings.
- **Error Handling**:
  - Use `thiserror` for library crates (`workspace-types`, `workspace-core`, `workspace-db`).
  - Never use `.unwrap()` or `.expect()` in non-test code paths. Use proper `Result<T, E>` pattern matching or the `?` operator.
  - Document potential error variants in custom error enums.
- **Documentation**: All public traits, structs, functions, and enum variants must include descriptive docstrings (`///`).

---

## 4. Frontend Code Standards

- **Strict TypeScript**: No `any` types. Define clear interfaces in `src/types/`.
- **Styling**: Use utility classes from Tailwind CSS. Respect the dark theme palette.
- **Accessibility**: Ensure keyboard navigation and ARIA attributes for all interactive elements (buttons, timeline controls, modals).
- **Testing**: Add component or interaction tests for complex UI widgets.

---

## 5. Pull Request Checklist

Before submitting your pull request, verify:
- [ ] `cargo test --workspace` passes cleanly.
- [ ] `cargo fmt --all -- --check` reports no discrepancies.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` produces zero warnings.
- [ ] Frontend builds without TypeScript or bundling errors (`npm run build`).
- [ ] Documentation has been updated for any new public APIs or schema changes.
- [ ] Sensitive secrets or `.env` files are not included.
