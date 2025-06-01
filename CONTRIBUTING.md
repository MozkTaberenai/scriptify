# Contributing to scriptify

Thank you for your interest in contributing to scriptify! This document provides comprehensive guidelines for both human and AI contributors.

## ⚠️ CRITICAL: Mandatory Pre-Commit Process

**BEFORE EVERY COMMIT, YOU MUST RUN:**

```bash
cargo xtask precommit
```

This command runs comprehensive checks (tests + clippy + formatting). **ALL checks must pass** before committing.

## Quick Start for Contributors

1. **Fork and clone** the repository
2. **Install Rust** (latest stable version)
3. **Run the development setup**:
   ```bash
   cargo test            # Verify everything works
   cargo xtask precommit # MANDATORY: Run pre-commit checks
   ```

## Required Reading

### 1. Project Overview
- `README.md` - Project description, features, and usage examples
- `CHANGELOG.md` - Recent changes and version history
- `Cargo.toml` - Dependencies and project metadata

### 2. Codebase Structure
```
scriptify/
├── src/
│   ├── lib.rs              # Main documentation (README source)
│   ├── cmd/                # Command execution functionality
│   │   ├── mod.rs          # Core implementation
│   │   └── tests.rs        # Test suite
│   ├── echo.rs             # Output formatting
│   ├── fs.rs               # File system operations
│   ├── color.rs            # Color definitions (internal)
│   └── style.rs            # Style definitions (internal)
├── examples/               # Usage examples
└── xtask/                  # Development task runner
```

## MANDATORY: Branch-Based Development Workflow

### Absolute Rules (ZERO TOLERANCE)
- **NEVER WORK DIRECTLY ON MAIN** - All changes must go through feature branches
- **NEVER COMMIT** with ANY clippy warnings or test failures
- **NEVER PUSH TO MAIN** - Always use feature branches and Pull Requests
- **ALWAYS** commit formatting changes separately before work commits

### Starting New Work
1. **Update main branch first:**
   ```bash
   git checkout main
   git pull origin main
   ```

2. **Create feature branch:**
   ```bash
   # Use descriptive branch names with prefixes
   git checkout -b feature/your-feature-name
   git checkout -b fix/bug-description
   git checkout -b docs/documentation-update
   git checkout -b refactor/code-improvement
   git checkout -b test/add-test-coverage
   git checkout -b chore/dependency-update
   ```

### Branch Naming Convention
- `feature/` - New functionality (e.g., `feature/stderr-piping`)
- `fix/` - Bug fixes (e.g., `fix/clippy-warnings`)
- `docs/` - Documentation updates (e.g., `docs/update-readme`)
- `refactor/` - Code refactoring (e.g., `refactor/pipeline-structure`)
- `test/` - Test additions/fixes (e.g., `test/pipe-mode-coverage`)
- `chore/` - Build process, dependencies (e.g., `chore/update-deps`)

## Documentation Management

### Important: README.md is Generated

**⚠️ DO NOT edit README.md directly!**

- README.md is automatically generated from `src/lib.rs` docstrings
- To update README.md: edit `src/lib.rs` and run `cargo xtask readme`

## Development Tasks

Development tasks can be run with `cargo` directly or via `cargo xtask`:

```bash
# Direct cargo commands
cargo test            # Run tests
cargo clippy --all-targets --all-features -- -D warnings  # Run comprehensive lints
cargo fmt             # Format code
cargo check           # Check compilation
cargo doc --open      # Generate and open documentation

# xtask commands (project-specific)
cargo xtask readme    # Generate README.md (after editing src/lib.rs)
cargo xtask precommit # Run pre-commit checks (test + clippy + fmt)
cargo xtask ci        # Run full CI pipeline locally
```

## Code Quality (MANDATORY)

### Pre-Commit Process (CRITICAL)

**MANDATORY before EVERY commit:**

```bash
# Step 1: Ensure tests pass
cargo test

# Step 2: Run all pre-commit checks (RECOMMENDED)
cargo xtask precommit  # Runs test + clippy + fmt automatically

# Step 3: CRITICAL - Handle formatting changes
git status  # Check for changes made by rustfmt
# If any files are modified by formatting, MUST commit them:
git add .
git commit -m "fix: apply rustfmt formatting changes"

# Step 4: Verify formatting is clean
cargo fmt -- --check  # MUST show no errors before proceeding

# Step 5: Update README if needed
cargo xtask readme    # If src/lib.rs docs were changed

# Step 6: Commit your actual changes
git add .
git commit -m "feat: descriptive commit message"
git push origin feature/branch-name
```

### Essential Clippy Commands
```bash
# Comprehensive clippy check (RECOMMENDED)
cargo clippy --all-targets --all-features -- -D warnings

# Individual checks
cargo clippy --tests -- -D warnings
cargo clippy --examples -- -D warnings
```

### Common Clippy Issues
- `clippy::write_with_newline` - Use `writeln!()` instead of `write!(_, "\n")`
- `clippy::unwrap_used` - Consider using `?` or proper error handling
- `clippy::expect_used` - Prefer explicit error handling

## Pull Request Workflow

### Before Submitting a PR

1. **During development** (run frequently):
   ```bash
   cargo xtask precommit  # Runs test + clippy + fmt
   ```

2. **Before final commit** (includes README generation):
   ```bash
   cargo xtask ci
   ```

3. **Important**: If `cargo fmt` makes changes, commit them separately:
   ```bash
   git status              # Check for formatting changes
   git add . && git commit -m "fix: apply rustfmt formatting"
   ```

4. **Commit both source and generated files**

### Creating Pull Requests

**Option 1: GitHub CLI (Preferred):**
```bash
# Create PR with title and description
gh pr create --title "Brief description of changes" \
  --body "Detailed description:
- What was changed
- Why it was changed
- How to test it
- Any breaking changes"

# Create draft PR for work in progress
gh pr create --draft --title "WIP: Feature name"
```

**Option 2: GitHub Web Interface:**
- Push branch to origin
- Visit GitHub repository
- Click "Compare & pull request"
- Fill in title and description

### PR Guidelines
- **Title**: Use conventional commit format (feat:, fix:, docs:, etc.)
- **Description**: Include what changed, why, testing instructions, breaking changes
- **Size**: Keep PRs focused and reasonably sized
- **Tests**: Ensure all tests pass and new functionality is tested

### Merge Strategy
- **Prefer "Squash and merge"** for clean main branch history
- **Delete feature branch** after successful merge
- **Never force push** to shared branches

## Platform Support

- **Primary platforms**: Linux and macOS
- **CI**: Only runs on Linux and macOS

When adding examples, use commands available on Unix systems.

## Code Style

- Use `cargo fmt` for formatting (enforced by CI)
- Follow Rust naming conventions
- Add documentation for public APIs
- Write tests for new functionality
- **ZERO TOLERANCE**: Fix ALL clippy warnings before committing

## Testing Strategy

- All examples should work on Unix systems
- Use `.quiet()` in tests to avoid cluttering output
- Test both native and fallback pipeline implementations
- Performance examples demonstrate streaming efficiency

## Commit Guidelines

- **BRANCH FIRST**: Never commit directly to main - always use feature branches
- Use conventional commit format (feat:, fix:, docs:, refactor:, test:, chore:)
- Test locally with full workflow before committing
- Update documentation in `src/lib.rs` when adding features
- Separate logical changes into different commits
- **MANDATORY**: Run clippy checks before EVERY commit
- **PULL REQUEST REQUIRED**: All changes must go through PR review process

## Common Development Tasks

### Add New Feature
```bash
git checkout -b feature/new-command-feature
# Modify src/cmd/mod.rs, add tests to src/cmd/tests.rs, update src/lib.rs docs
# Run pre-commit checks, commit, push, create PR
```

### Add Example
```bash
git checkout -b docs/add-example
# Create in examples/, ensure Unix compatibility
# Run pre-commit checks, commit, push, create PR
```

### Fix Performance
```bash
git checkout -b refactor/improve-performance
# Focus on pipeline implementation in src/cmd/mod.rs
# Run pre-commit checks, commit, push, create PR
```

### Update Documentation
```bash
git checkout -b docs/update-documentation
# Edit src/lib.rs docstring, run cargo xtask readme
# Run pre-commit checks, commit, push, create PR
```

### Fix Clippy Warnings
```bash
git checkout -b fix/clippy-warnings
# Run cargo clippy --all-targets --all-features -- -D warnings
# Fix all warnings, run pre-commit checks, commit, push, create PR
```

## Recent Important Changes

- Implemented efficient native pipelines using `std::io::pipe` (Rust 1.87.0+)
- Restructured cmd module into `src/cmd/` directory with separate test file
- Added automatic fallback for shell-based pipes on older Rust versions
- Simplified xtask to focus on project-specific tasks only

## AI Agent Specific Instructions

**For AI Agents**: See `.ai-instructions.md` for additional AI-specific guidelines including tool usage, communication style, and code block formatting requirements.

## Getting Help

- Look at existing code for patterns
- Ask questions in issues or discussions
- Review `examples/` directory for usage patterns
- Check CHANGELOG.md for recent changes

## Code of Conduct

Be respectful and constructive in all interactions. This project follows the Rust community's standards of inclusive and welcoming behavior.