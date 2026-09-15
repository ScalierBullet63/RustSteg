# Contributing to RustSteg

Thank you for your interest in contributing to **RustSteg**! We appreciate your time and effort to help make this project better—whether you are fixing a bug, adding a new feature, or improving documentation.

To maintain code quality, security, and project stability, we ask all contributors to follow the guidelines outlined below.

---

## Guidelines

### 1. Pull Requests (PRs)
* **PRs Are Mandatory:** Direct pushes to the `main` branch are strictly disabled. All changes must be submitted via a Pull Request.
* **Feature Branches:** Always create a descriptive branch off `main` for your work (e.g., `feature/add-png-support`, `fix/cli-parsing`, `docs/update-readme`).
* **Clear Descriptions:** Include a summary of your changes and reference any relevant issues in your PR.

### 2. Commit Standards & Signed Commits
* **Conventional Commits:** Commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/) specification (e.g., `feat: ...`, `fix: ...`, `docs: ...`).
* **Signed Commits:** All commits should preferably be signed using a verified GPG or SSH key on GitHub.

### 3. Code Quality & CI Checks
Your PR will only be merged once all Automated CI checks pass. Before submitting, verify the following locally:

* **Formatting:** Format your code according to standard Rust style:
  cargo fmt --check

* **Linting (Clippy):** Your code **must pass Clippy with zero warnings or errors**:
  cargo clippy -- -D warnings

* **Tests:** All existing and new unit/integration tests must pass:
  cargo test

---

## Quickstart Guide

1. **Fork** the repository.
2. **Clone** your fork and create a new branch:
   git checkout -b feature/your-feature-name

3. **Make your changes** and ensure `cargo test` and `cargo clippy` run clean.
4. **Commit** your changes with a signed commit:
   git commit -S -m "feat: add support for custom payload encryption"

5. **Push** to your fork and open a **Pull Request** against the `main` branch.
