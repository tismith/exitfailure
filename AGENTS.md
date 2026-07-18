# Repository Guidelines

## Project overview

- This is a small Rust library providing `ExitFailure` and `ExitDisplay` wrappers for errors returned from `main()`.
- Keep the public API focused on those wrappers and preserve their user-facing error formatting unless a change explicitly requires otherwise.
- The crate supports Rust 1.26 and later. Preserve Rust 1.26-compatible syntax and APIs; do not remove compatibility code merely because it is unnecessary on newer compilers.
- Public items must be documented. `src/lib.rs` denies missing documentation, unsafe code, unstable features, and several other lints.

## Development and validation

- Format Rust changes with `cargo fmt --all` and verify them with `cargo fmt --all -- --check`.
- Run `cargo clippy --all-targets` for linting.
- Build examples before integration tests with `cargo build --examples`, then run `cargo test --verbose`.
- Build documentation with `cargo doc --no-deps --verbose` when public APIs or documentation change.
- CI also exercises stable, beta, nightly, macOS, Linux, and Windows toolchains. Avoid platform-specific paths or output assumptions; integration tests invoke binaries from `target/debug/examples`.
- CI enforces at least 90% line coverage with `cargo llvm-cov`. Add or update tests for behavior changes.
- `Cargo.lock` is intentionally ignored because this is a library crate; do not add it unless the repository policy changes.

## Commit messages

Follow the commit-message requirements in `CONTRIBUTING.md` whenever creating or suggesting a commit.

- Format the subject as `TYPE(COMPONENT): MESSAGE`, or as `TYPE: MESSAGE` when no component is useful.
- Use one of these types: `feat`, `perf`, `fix`, `docs`, `test`, `refactor`, `style`, `wip`, `build`, or `chore`.
- The optional component may name a single file, directory, or logical component.
- Use `wip` only for temporary work-in-progress commits; such commits should normally be removed or rewritten with `git rebase` before the work is finalized.

Examples:

- `feat(parser): support nested expressions`
- `docs: clarify installation steps`
