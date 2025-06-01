# scriptify

[![Crates.io](https://img.shields.io/crates/v/scriptify.svg)](https://crates.io/crates/scriptify)
[![Documentation](https://docs.rs/scriptify/badge.svg)](https://docs.rs/scriptify)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/MozkTaberenai/scriptify/workflows/CI/badge.svg)](https://github.com/MozkTaberenai/scriptify/actions)

## scriptify

**Scriptify your Rust** - A simple and intuitive library that makes running shell commands and file operations easy and visible.

### Why scriptify?

When you need to write system administration scripts, build tools, or automation in Rust,
you often find yourself wrestling with `std::process::Command` and `std::fs`. scriptify
provides a clean, shell-script-like interface while keeping all the benefits of Rust's
type safety and error handling.

#### Key Features

- **🎨 Colorful output**: See exactly what commands are being executed
- **🔗 Easy piping**: Chain commands together naturally
- **📁 File operations**: Wrapper around `std::fs` with automatic logging
- **🔧 Builder pattern**: Fluent API for command construction
- **⚡ Zero dependencies**: Only uses `anstyle` for colors
- **🛡️ Type safe**: All the safety of Rust with the convenience of shell scripts

### Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
scriptify = "0.1.0"
```

### Basic Usage

#### Command Execution

```rust
use scriptify::*;

// Simple command execution
cmd!("echo", "Hello, World!").run()?;

// Get command output
let output = cmd!("date").output()?;
println!("Current date: {}", output.trim());

// Command with multiple arguments
cmd!("ls", "-la", "/tmp").run()?;

// Using the builder pattern
cmd!("grep", "error")
    .arg("logfile.txt")
    .cwd("/var/log")
    .env("LANG", "C")
    .run()?;
```

#### Command Piping

Chain commands together just like in shell scripts:

```rust
use scriptify::*;

// Simple pipe
cmd!("echo", "hello world")
    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
    .run()?;

// Multiple pipes
cmd!("cat", "/etc/passwd")
    .pipe(cmd!("grep", "bash"))
    .pipe(cmd!("wc", "-l"))
    .run()?;

// Get piped output
let result = cmd!("ps", "aux")
    .pipe(cmd!("grep", "rust"))
    .pipe(cmd!("wc", "-l"))
    .output()?;
echo!("Rust processes:", result.trim());
```

#### Input and Output

```rust
use scriptify::*;

// Provide input to a command
let result = cmd!("sort")
    .input("banana\napple\ncherry\n")
    .output()?;
echo!("Sorted fruits:", result.trim());

// Pipe with input
let result = cmd!("tr", "[:lower:]", "[:upper:]")
    .input("hello world")
    .output()?;
echo!("Uppercase:", result.trim());
```

#### Environment and Working Directory

```rust
use scriptify::*;

// Set environment variables
cmd!("printenv", "MY_VAR")
    .env("MY_VAR", "Hello from Rust!")
    .run()?;

// Change working directory
cmd!("pwd").cwd("/tmp").run()?;

// Combine multiple settings
cmd!("make", "install")
    .env("PREFIX", "/usr/local")
    .env("DESTDIR", "/tmp/staging")
    .cwd("./my-project")
    .run()?;
```

#### File Operations

All file operations are logged automatically:

```rust
use scriptify::*;

// Basic file operations
fs::write("config.txt", "debug=true\nport=8080")?;
let content = fs::read_to_string("config.txt")?;
echo!("Config content:", content);

// Directory operations
fs::create_dir_all("project/src")?;
fs::copy("config.txt", "project/config.txt")?;

// Cleanup
fs::remove_file("config.txt")?;
fs::remove_dir_all("project")?;
```

#### Error Handling

scriptify uses Rust's standard error handling patterns:

```rust
use scriptify::*;

// Handle command failures gracefully
match cmd!("nonexistent-command").run() {
    Ok(_) => echo!("Command succeeded"),
    Err(e) => echo!("Command failed:", e),
}

// Use the ? operator for early returns
fn deploy_app() -> Result<()> {
    cmd!("cargo", "build", "--release").run()?;
    cmd!("docker", "build", "-t", "myapp", ".").run()?;
    cmd!("docker", "push", "myapp").run()?;
    echo!("Deployment complete!");
    Ok(())
}
```

#### Quiet Mode

Sometimes you don't want to see the command output:

```rust
use scriptify::*;

// Run silently
cmd!("git", "status").quiet().run()?;

// Get output without showing the command
let output = cmd!("whoami").quiet().output()?;
echo!("Current user:", output.trim());
```

### Real-World Examples

#### Build Script

```rust
use scriptify::*;

fn main() -> Result<()> {
    echo!("Building project...");

    // Clean previous build
    if fs::metadata("target").is_ok() {
        fs::remove_dir_all("target")?;
    }

    // Build in release mode
    cmd!("cargo", "build", "--release").run()?;

    // Run tests
    cmd!("cargo", "test").run()?;

    // Package the binary
    fs::create_dir_all("dist")?;
    fs::copy("target/release/myapp", "dist/myapp")?;

    echo!("Build complete! Binary available in dist/");
    Ok(())
}
```

#### Log Analysis

```rust
use scriptify::*;

fn analyze_logs() -> Result<()> {
    echo!("Analyzing web server logs...");

    // Count total requests
    let total = cmd!("wc", "-l")
        .input(&fs::read_to_string("/var/log/nginx/access.log")?)
        .output()?;
    echo!("Total requests:", total.trim());

    // Find top IPs
    let top_ips = cmd!("cut", "-d", " ", "-f", "1")
        .pipe(cmd!("sort"))
        .pipe(cmd!("uniq", "-c"))
        .pipe(cmd!("sort", "-nr"))
        .pipe(cmd!("head", "-10"))
        .input(&fs::read_to_string("/var/log/nginx/access.log")?)
        .output()?;

    echo!("Top 10 IPs:");
    echo!(top_ips);

    Ok(())
}
```

#### System Administration

```rust
use scriptify::*;

fn system_info() -> Result<()> {
    echo!("=== System Information ===");

    // OS information
    let os = cmd!("uname", "-a").output()?;
    echo!("OS:", os.trim());

    // Memory usage
    cmd!("free", "-h").run()?;

    // Disk usage
    cmd!("df", "-h").run()?;

    // Running processes
    let process_count = cmd!("ps", "aux")
        .pipe(cmd!("wc", "-l"))
        .output()?;
    echo!("Running processes:", process_count.trim());

    Ok(())
}
```

### Environment Variables

You can control scriptify's behavior with environment variables:

- `NO_ECHO`: Set to any value to suppress command echoing globally

```bash
NO_ECHO=1 cargo run  # Run without command echoing
```

### Comparison with Alternatives

| Feature | scriptify | std::process::Command | shell scripts |
|---------|-----------|----------------------|---------------|
| Type safety | ✅ | ✅ | ❌ |
| Error handling | ✅ | ✅ | ⚠️ |
| Piping | ✅ | ⚠️ Manual | ✅ |
| Visibility | ✅ | ❌ | ✅ |
| Cross-platform | ✅ | ✅ | ⚠️ |
| IDE support | ✅ | ✅ | ⚠️ |
| Debugging | ✅ | ✅ | ❌ |

### Contributing

We welcome contributions! Please see our [GitHub repository](https://github.com/MozkTaberenai/scriptify) for more information.

### License

This project is licensed under the MIT License.

License: MIT


## Examples

The following examples are available in the `examples/` directory:

### cmd

```rust
// Basic command execution
cmd!("echo", "Hello, World!").run()?;
// Command with multiple arguments
cmd!("echo").args(["a", "b", "c"]).run()?;
// Command with environment variable
cmd!("echo", "hello").env("USER", "alice").run()?;
// Command with working directory
cmd!("ls", "-la").cwd("src").run()?;
// Get command output
let date = cmd!("date").output()?;
echo!("Current date:", date.trim());
// Handle command that might fail
if let Err(err) = cmd!("unknown_command").run() {
    echo!("Command failed:", err);
}
// Command piping
cmd!("echo", "hello world")
    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
    .run()?;
// Multiple pipes
cmd!("date")
    .pipe(cmd!("rev"))
    .pipe(cmd!("tr", "[:upper:]", "[:lower:]"))
    .run()?;
// Pipe with input
let result = cmd!("tr", "[:lower:]", "[:upper:]")
    .input("hello world")
    .output()?;
echo!("Uppercase:", result.trim());
// Pipeline with input
let result = cmd!("sort")
    .pipe(cmd!("uniq"))
    .input("apple\nbanana\napple\ncherry\nbanana")
    .output()?;
echo!("Unique fruits:", result.trim());
// Quiet execution (no echo)
cmd!("echo", "This won't be echoed").quiet().run()?;
Ok(())
```

Run with: `cargo run --example cmd`

### fs

```rust
fs::create_dir("tmp")?;
fs::write("tmp/a.txt", "abc")?;
show_metadata("tmp/a.txt")?;
fs::copy("tmp/a.txt", "tmp/b.txt")?;
show_metadata("tmp/b.txt")?;
fs::hard_link("tmp/a.txt", "tmp/h.txt")?;
show_metadata("tmp/h.txt")?;
fs::rename("tmp/a.txt", "tmp/c.txt")?;
show_metadata("tmp/c.txt")?;
fs::create_dir_all("tmp/d/e")?;
for entry in fs::read_dir("tmp")? {
    show_metadata(entry?.path())?;
}
fs::remove_file("tmp/b.txt")?;
fs::remove_dir("tmp/d/e")?;
fs::remove_dir_all("tmp")?;
Ok(())
```

Run with: `cargo run --example fs`


## Development

This project uses `cargo xtask` for development tasks:

```bash
# Generate README.md
cargo xtask readme

# Run all tests
cargo xtask test

# Run code formatting
cargo xtask fmt

# Run clippy lints
cargo xtask clippy

# Run full CI pipeline
cargo xtask ci
```

