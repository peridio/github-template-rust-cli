# Rust CLI Template

## Features

- **Configuration files** - JSON/YAML config with environment variable overrides and profile system (local, CI, release)
- **CI/CD** - Automated checks, multi-platform releases, and code coverage
- **Test patterns** - Example integration tests in `tests/`
- **Self-upgrade** - Upgrade in-place with built-in upgrade command
- **Structured logging** - Syslog levels and progressive verbosity
- **Error handling** - Type-safe errors with automatic propagation and context

## Commands included

- `run` - Example file processing with structured output
- `upgrade` - Self-upgrade from GitHub releases

## Getting started

1. Clone this repository to your desired location:
   ```bash
   git clone https://github.com/peridio/template-cli-rust.git my-cli
   cd my-cli
   ```
2. Remove the existing git history and start fresh:
   ```bash
   rm -rf .git
   git init
   ```
3. Run the template replacement script, it will describe usage:
   ```bash
   ./scripts/replace_templates.sh
   ```
4. Run `cargo build` to verify everything compiles
5. Create your own repository and push:
   ```bash
   git add .
   git commit -m "Initial commit from Rust CLI template"
   git remote add origin <your-repository-url>
   git push -u origin main
   ```
6. Delete this section and update the README for your project

---

# __TEMPLATE_PACKAGE_NAME__

- [Installation](docs/installation.md)
- [Release](docs/release.md)
- [Testing](docs/testing.md)
