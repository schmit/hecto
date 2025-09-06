# Repository Guidelines

## Project Structure & Module Organization
- Source: `src/` — entrypoint `src/main.rs`. Core editor logic lives in `src/editor.rs` with submodules under `src/editor/` (e.g., `terminal.rs`, `position.rs`, `view.rs`, `view/buffer.rs`, `view/line.rs`).
- Assets: `assets/` — sample files used for development (e.g., `unicode.txt`).
- Metadata: `Cargo.toml`, `Cargo.lock`. Build artifacts in `target/` (ignored).
- GitHub/CI: `.github/` (if present) contains workflows.

## Build, Test, and Development Commands
- Build: `cargo build` — compiles the crate.
- Run: `cargo run -- [file]` — launches the editor, optionally opening a file.
- Format: `cargo fmt --all` — formats code via rustfmt.
- Lint: `cargo clippy --all-targets --all-features -- -D warnings` — runs Clippy; the crate enables `clippy::all, clippy::pedantic`.
- Test: `cargo test` — runs unit/integration tests (add as needed).

## Coding Style & Naming Conventions
- Style: idiomatic Rust; formatted with rustfmt (4‑space indent, trailing commas where sensible).
- Naming: `snake_case` for functions/vars/modules; `CamelCase` for types/traits; `SCREAMING_SNAKE_CASE` for consts.
- Modules: keep terminal I/O in `src/editor/terminal.rs`; view/buffer logic in `src/editor/view/*`; avoid mixing concerns in `main.rs`.

## Testing Guidelines
- Unit tests: colocate with modules using `#[cfg(test)] mod tests { ... }`.
- Integration tests: add files under `tests/` for end‑to‑end editor behaviors.
- Conventions: name tests after behavior (e.g., `opens_empty_buffer`, `saves_file_utf8`).
- Run full suite with `cargo test`; prefer deterministic tests (no tty required) for logic pieces like buffer/view.

## Commit & Pull Request Guidelines
- Commits: imperative, concise subject (≤72 chars), include scope when useful (e.g., "view: fix line wrap"). Provide a short body explaining why when nontrivial.
- Before pushing: `cargo fmt --all && cargo clippy --all-targets --all-features -- -D warnings && cargo test`.
- PRs: clear description, linked issues, reproduction steps, and terminal screenshots/gifs when UI behavior changes. Note any follow‑ups or TODOs.
- Review: prefer small, focused PRs; add comments where behavior is subtle (terminal handling, Unicode, wrapping).
- Use Jujutsu for version control. After every change, commit using `jj commit -m <short explanation of change>`

## Architecture Overview
- Binary crate implementing a terminal text editor. Core pieces: terminal backend, buffer/view model, and high‑level editor coordination. Keep pure logic testable and isolate terminal‑specific code.

