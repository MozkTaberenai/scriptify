# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of scriptify
- `cmd!` macro for easy command execution
- Command piping with `.pipe()` method
- Builder pattern for command configuration
- Environment variable setting with `.env()`
- Working directory changes with `.cwd()`
- Input/output handling with `.input()` and `.output()`
- Quiet mode with `.quiet()`
- File system operations with automatic logging (`fs` module)
- Colorful command echoing using `anstyle`
- Comprehensive error handling
- `echo!` macro for formatted output
- Support for `NO_ECHO` environment variable
- Cross-platform compatibility (Unix, Windows, macOS)

### Performance Improvements (Rust 1.87.0+)
- **Native pipeline implementation** using `std::io::pipe` for enhanced performance
- **Memory-efficient streaming** for large data processing without buffering
- **Reduced process overhead** by eliminating shell delegation for multi-command pipes
- **Platform-independent pipes** that work without shell dependency
- **Automatic fallback** to shell-based pipes for compatibility with older Rust versions
- **Parallel command execution** in pipelines for better throughput
- **Real-time data processing** with true streaming capabilities

### Features
- **Zero runtime dependencies** (only `anstyle` for colors)
- **Type-safe command building** with fluent API
- **Automatic command logging** with colored output
- **Shell-script-like piping** without shell dependency
- **Builder pattern** for complex command construction
- **Error propagation** compatible with `?` operator
- **File operations logging** for debugging

### Examples
- Command execution examples
- File system operation examples
- Real-world build script examples
- Log analysis examples
- System administration examples

### Documentation
- Comprehensive rustdoc documentation
- README with usage examples
- Comparison table with alternatives
- Development tooling (Makefile, justfile)
- GitHub Actions CI/CD pipeline

## [0.1.0] - 2024-01-XX

### Added
- Initial public release