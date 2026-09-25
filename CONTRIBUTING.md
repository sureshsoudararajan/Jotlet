# Contributing to Jotlet

Thank you for your interest in contributing to Jotlet! We welcome bug reports, feature requests, documentation improvements, and code contributions.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## Development Setup

### Prerequisites

- Arch Linux (or any Linux distribution with GNOME 44+)
- Rust stable (1.80+)
- GTK4 development headers (`gtk4`)
- Libadwaita development headers (`libadwaita`)
- SQLite development headers (`sqlite`)
- `pkgconf` / `pkg-config`

On Arch Linux:
```bash
sudo pacman -S --needed rustup gtk4 libadwaita sqlite pkgconf
rustup default stable
```

### Building from Source

```bash
git clone https://github.com/example/jotlet.git
cd jotlet

# Check types and syntax
cargo check

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Build debug binary
cargo build

# Run Jotlet
cargo run

# Build optimized release binary
cargo build --release
```

## Guidelines

1. **Idiomatic Rust**: Write clean, safe Rust. Avoid `unwrap()` in production paths; use `Result` and `Option` properly.
2. **GNOME HIG**: Respect GNOME Human Interface Guidelines. Keep UI minimal, clean, and distraction-free.
3. **Privacy First**: Jotlet is strictly an offline personal notes tool. Never add network dependencies, analytics, or telemetry.
4. **Testing**: Add unit tests in `tests/` for any new logic, database operations, or format conversions.
5. **No Broken Builds**: Ensure `cargo check`, `cargo test`, and `cargo clippy -- -D warnings` pass before submitting PRs.

## Submitting Pull Requests

1. Fork the repository and create your branch from `main`:
   ```bash
   git checkout -b feature/my-new-feature
   ```
2. Commit your changes with clear, descriptive commit messages.
3. Push to your fork and submit a Pull Request.
4. Ensure CI checks pass.
