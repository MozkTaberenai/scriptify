# Contributing to scriptify

Thank you for your interest in contributing to scriptify! This document provides guidelines for contributing to the project.

## Quick Start for Contributors

1. **Fork and clone** the repository
2. **Install Rust** (latest stable version)
3. **Run the development setup**:
   ```bash
   cargo xtask ci
   ```

## Development Workflow

### Important: README.md is Generated

**⚠️ DO NOT edit README.md directly!**

- README.md is automatically generated from `src/lib.rs` docstrings
- To update README.md: edit `src/lib.rs` and run `cargo xtask readme`
- See `docs/DEVELOPMENT.md` for detailed workflow

### Development Tasks

All development tasks use `cargo xtask`:

```bash
cargo xtask readme    # Generate README.md (after editing src/lib.rs)
cargo xtask fmt       # Format code (required before commits)
cargo xtask test      # Run tests
cargo xtask clippy    # Run lints
cargo xtask ci        # Run full CI pipeline locally
```

### Before Submitting a PR

1. **Run full CI locally**:
   ```bash
   cargo xtask ci
   ```

2. **Ensure README.md is up to date**:
   ```bash
   cargo xtask readme
   ```

3. **Commit both source and generated files**

## Platform Support

- **Primary platforms**: Linux and macOS
- **Windows**: Limited support (examples use Unix commands)
- **CI**: Only runs on Linux and macOS

When adding examples, use commands available on Unix systems.

## Code Style

- Use `cargo fmt` for formatting (enforced by CI)
- Follow Rust naming conventions
- Add documentation for public APIs
- Write tests for new functionality

## Submitting Changes

1. **Create a feature branch** from `main`
2. **Make your changes** following the guidelines above
3. **Test thoroughly** with `cargo xtask ci`
4. **Write descriptive commit messages**
5. **Submit a pull request**

## AI Agent Instructions

**For AI Agents**: Before starting work on this repository, please read `.ai-instructions.md` for specific onboarding instructions and development guidelines.

## Getting Help

- Check `docs/DEVELOPMENT.md` for detailed workflows
- Look at existing code for patterns
- Ask questions in issues or discussions

## Code of Conduct

Be respectful and constructive in all interactions. This project follows the Rust community's standards of inclusive and welcoming behavior.