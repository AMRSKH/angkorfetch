# Contributing to AngkorFetch

Thank you for your interest in contributing to AngkorFetch! This document explains how to set up your environment, follow our coding standards, and submit pull requests.

## About the Project

AngkorFetch is a fast, cross-platform system information ("fetch") utility written in Rust. It delivers clean, immediate hardware and system data across Windows, macOS, and Linux with zero external runtime dependencies and terminal-adaptive graphics.

## Development Requirements

To contribute code, ensure you have:

* **Rust stable** (edition 2021) and **Cargo** ([rustup.rs](https://rustup.rs/))
* **Git**
* **Python 3.x** (for package manifest verification)

## Getting Started

1. Fork the repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/AMRSKH/angkorfetch.git
   cd angkorfetch
   ```
3. Create a descriptive topic branch:
   ```bash
   git switch -c feat/my-improvement
   ```

## Build

Compile the project in debug mode:
```bash
cargo build
```

Or build an optimized release binary:
```bash
cargo build --release
```

## Testing

Run the standard test suite:
```bash
cargo test --locked
```

Run manifest automation tests:
```bash
python -m unittest discover -s scripts -p "test_*.py"
```

## Formatting

Verify code formatting conforms to Rust standards:
```bash
cargo fmt --check
```

Format code automatically:
```bash
cargo fmt
```

## Linting

Ensure there are no Clippy warnings:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Coding Guidelines

* **Idiomatic Rust**: Write clean, concise, readable code.
* **Minimal dependencies**: AngkorFetch intentionally restricts its dependency footprint to three core crates (`sysinfo`, `colored`, `terminal_size`). Do not add third-party crates without maintainer consensus.
* **Graceful degradation**: Never call `.unwrap()` or panic on missing system attributes or permission errors. When data is unavailable, fall back cleanly to `"Unknown"` or `"N/A"`.
* **Platform parity**: Preserve existing behavior across Windows, Linux, and macOS.
* **No unrelated refactoring**: Keep pull requests focused on one objective.

## Pull Requests

When opening a Pull Request:
1. Provide a clear description of **what** changed and **why**.
2. Specify which operating systems and architectures are affected.
3. Include tests or instructions demonstrating how you verified your changes.
4. Keep PRs focused; split unrelated changes into separate PRs.
5. Base feature PRs on the `dev` branch; submit urgent bug fixes or documentation updates to `main`.

## Adding a New OS

If you want to add support for a new operating system:
- Add appropriate `#[cfg(target_os = "...")]` blocks inside `src/main.rs`.
- Follow the query conventions of existing OS implementations (use native OS APIs or system files directly rather than spawning heavy subprocesses where possible).
- Fall back gracefully when root/administrator permissions are absent.

## Adding a Package Manager

When introducing packaging for a new distribution channel or package manager:
1. **Package Metadata**: Define clean, standard packaging specifications (e.g. `linux/`, `flatpak/`, `winget-pkgs/`).
2. **Release Integration**: Coordinate with GitHub Actions release workflows to ensure version and artifact naming match.
3. **Validation**: Test the build, install, upgrade, and uninstall flows locally.
4. **Publishing**: Document the submission or repository hosting process in `docs/PACKAGING.md`.
5. **Documentation**: **Never** add an installation command claiming a package is "Available" in the README until the package is published in an accessible registry. Use "Pending" or "Planned" status until verified.

## Detailed Documentation Links

- Technical development details: [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)
- Packaging specifications and distribution statuses: [docs/PACKAGING.md](docs/PACKAGING.md)
- Release engineering and CI workflows: [docs/RELEASING.md](docs/RELEASING.md)
- Repository maintainer operational guide: [docs/MAINTAINING.md](docs/MAINTAINING.md)
