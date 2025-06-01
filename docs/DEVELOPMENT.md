# Development Workflow

This document outlines the development workflow and important patterns for maintaining this project.

## Documentation Management

### README.md Generation

**IMPORTANT**: README.md is automatically generated and should NOT be edited directly.

#### How README.md is Generated

1. **Source**: The main content comes from the docstring in `src/lib.rs` (lines starting with `//!`)
2. **Generator**: The `xtask/src/readme.rs` script handles the generation process
3. **Command**: Run `cargo xtask readme` to regenerate README.md

#### To Update README.md

1. **Edit the source**: Modify the docstring in `src/lib.rs`
   ```rust
   //! # Your new content here
   //! 
   //! More documentation...
   ```

2. **Regenerate**: Run the generation command
   ```bash
   cargo xtask readme
   ```

3. **Verify**: Check that README.md was updated correctly

#### README Structure

The generated README.md includes:
- Header with badges (from `xtask/src/readme.rs`)
- Main content (from `src/lib.rs` docstring)
- Examples section (auto-extracted from `examples/` directory)
- Development section (from `xtask/src/readme.rs`)

## Development Tasks

Use `cargo xtask` for all development tasks:

```bash
# Generate README.md from lib.rs documentation
cargo xtask readme

# Generate and open documentation
cargo xtask docs

# Run all tests
cargo xtask test

# Run cargo check
cargo xtask check

# Format code (required before commits)
cargo xtask fmt

# Run clippy lints
cargo xtask clippy

# Clean build artifacts
cargo xtask clean

# Run benchmarks
cargo xtask bench

# Generate test coverage report
cargo xtask coverage

# Run full CI pipeline locally
cargo xtask ci
```

## Code Quality

### Before Committing

Always run the full CI pipeline locally:
```bash
cargo xtask ci
```

This will:
1. Format code (`cargo fmt`)
2. Run clippy lints
3. Check compilation
4. Run tests
5. Regenerate README.md

### Formatting

- Code MUST be formatted with `cargo fmt` before committing
- CI will fail if code is not properly formatted
- Use `cargo xtask fmt` which includes additional checks

## Platform Support

### Current Support
- **Linux**: ✅ Full support
- **macOS**: ✅ Full support  
- **Windows**: ⚠️ Limited support

### Windows Limitations
- Examples and tests use Unix-specific commands (`ls`, `cat`, `tr`, `sort`, etc.)
- Windows is excluded from CI testing
- Core functionality works but examples may fail

### Adding New Examples

When adding examples to `examples/` directory:
1. Examples are automatically included in README.md generation
2. Use commands that work on Unix systems (Linux/macOS)
3. Consider Windows compatibility if possible

## Git Workflow

### Commit Message Format
Use descriptive commit messages that explain what and why:
```
Fix rustfmt formatting issues

- Apply cargo fmt to all source files
- Fix line spacing and indentation
- Ensure consistent code style across the project
```

### Before Pushing
1. Run `cargo xtask ci` to ensure everything passes
2. Check that README.md is up to date (run `cargo xtask readme` if needed)
3. Verify CI badges and links are working

## Common Mistakes to Avoid

1. **❌ Editing README.md directly** - Always edit `src/lib.rs` and regenerate
2. **❌ Skipping formatting** - CI will fail without proper formatting
3. **❌ Adding Windows-specific tests** - Focus on Unix compatibility
4. **❌ Forgetting to run xtask ci** - Always test locally before pushing

## Project Structure

```
scriptify/
├── src/lib.rs              # Main documentation (README source)
├── src/cmd.rs              # Command execution functionality
├── src/fs.rs               # File system operations
├── src/echo.rs             # Output formatting
├── src/color.rs            # Color definitions
├── src/style.rs            # Style utilities
├── examples/               # Examples (auto-included in README)
├── xtask/                  # Development task runner
│   ├── src/main.rs         # Task definitions
│   └── src/readme.rs       # README generation logic
├── docs/                   # Development documentation
├── .github/workflows/      # CI configuration
└── README.md               # Generated - DO NOT EDIT
```

## Troubleshooting

### CI Failures
- **Rustfmt**: Run `cargo xtask fmt`
- **Tests**: Run `cargo xtask test` locally
- **Clippy**: Run `cargo xtask clippy` and fix warnings

### README Issues
- Check `src/lib.rs` docstring syntax
- Verify `cargo readme` tool is installed
- Run `cargo xtask readme` manually

Remember: When in doubt, run `cargo xtask ci` to catch issues before they reach CI!