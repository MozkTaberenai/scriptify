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

### Platform Support

Currently supported platforms:
- **Linux** ✅ Full support with native pipe optimization
- **macOS** ✅ Full support with native pipe optimization
- **Windows** ⚠️ Limited support

**Note on Windows**: While the core functionality works on Windows, many examples and tests use Unix-specific commands (`ls`, `cat`, `tr`, `sort`, etc.) that are not available in standard Windows environments. Windows support could be improved in future versions with command mapping or by requiring tools like Git Bash or WSL.

### Requirements

- **Rust 1.87.0 or later** - Required for native pipeline performance with `std::io::pipe`

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

Chain commands together just like in shell scripts. **New in Rust 1.87.0**: scriptify now uses native `std::io::pipe` for enhanced performance and memory efficiency!

```rust
use scriptify::*;

// Simple pipe
cmd!("echo", "hello world")
    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
    .run()?;

// Multiple pipes - now using efficient native pipes!
cmd!("cat", "/etc/passwd")
    .pipe(cmd!("grep", "bash"))
    .pipe(cmd!("wc", "-l"))
    .run()?;

// Get piped output with streaming processing
let result = cmd!("ps", "aux")
    .pipe(cmd!("grep", "rust"))
    .pipe(cmd!("wc", "-l"))
    .output()?;
echo!("Rust processes:", result.trim());
```

##### Pipeline Performance Improvements (Rust 1.87.0+)

- **Memory efficient**: Uses streaming instead of buffering all data
- **Better performance**: Native pipes reduce process overhead
- **Platform independent**: No shell dependency for multi-command pipes
- **Native implementation**: Uses `std::io::pipe` for optimal performance

```rust
use scriptify::*;

// Large data processing with efficient streaming
let large_data = "..."; // Megabytes of data
let result = cmd!("grep", "pattern")
    .pipe(cmd!("sort"))
    .pipe(cmd!("uniq", "-c"))
    .input(large_data)
    .output()?; // Processes without loading all data into memory
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
| Piping | ✅ Native pipes (1.87+) | ⚠️ Manual | ✅ |
| Memory efficiency | ✅ Streaming | ❌ | ⚠️ |
| Visibility | ✅ | ❌ | ✅ |
| Cross-platform | ✅ | ✅ | ⚠️ |
| IDE support | ✅ | ✅ | ⚠️ |
| Debugging | ✅ | ✅ | ❌ |
| Performance | ✅ Optimized | ⚠️ | ⚠️ |

### Contributing

We welcome contributions! Please see our [GitHub repository](https://github.com/MozkTaberenai/scriptify) for more information.

### License

This project is licensed under the MIT License.

License: MIT


## Examples

The following examples are available in the `examples/` directory:

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

### pipe_modes

```rust
println!("=== Pipe Mode Examples ===\n");
// Example 1: Default stdout piping
println!("1. Default stdout piping:");
let output = cmd!("echo", "hello world")
    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
    .output()?;
println!("   Output: {}", output.trim());
println!();
// Example 2: Stderr piping
println!("2. Stderr piping:");
println!("   Command: Generate error message and count its characters");
let error_char_count = cmd!("sh", "-c", "echo 'Error: Something went wrong!' >&2")
    .pipe(cmd!("wc", "-c"))
    .pipe_stderr()
    .output()?;
println!(
    "   Error message character count: {}",
    error_char_count.trim()
);
println!();
// Example 3: Both stdout and stderr piping
println!("3. Combined stdout+stderr piping:");
println!("   Command: Generate both outputs and sort them together");
let combined_output = cmd!("sh", "-c", "echo 'stdout line'; echo 'stderr line' >&2")
    .pipe(cmd!("sort"))
    .pipe_both()
    .output()?;
println!("   Combined and sorted output:");
for line in combined_output.lines() {
    println!("     {}", line);
}
println!();
// Example 4: Using PipeMode explicitly
println!("4. Explicit pipe mode setting:");
let explicit_output = cmd!("echo", "test data")
    .pipe(cmd!("cat"))
    .pipe_mode(PipeMode::Stdout)
    .output()?;
println!("   Explicit stdout mode: {}", explicit_output.trim());
println!();
// Example 5: Error processing pipeline
println!("5. Error processing pipeline:");
println!("   Command: Generate multiple error lines and count them");
let error_lines = cmd!(
    "sh",
    "-c",
    "echo 'ERROR 1' >&2; echo 'ERROR 2' >&2; echo 'ERROR 3' >&2"
)
.pipe(cmd!("wc", "-l"))
.pipe_stderr()
.output()?;
println!("   Number of error lines: {}", error_lines.trim());
println!();
// Example 6: Complex stderr processing
println!("6. Complex stderr processing:");
println!("   Command: Filter specific errors from stderr");
let filtered_errors = cmd!(
    "sh",
    "-c",
    "echo 'INFO: starting' >&2; echo 'ERROR: failed' >&2; echo 'INFO: done' >&2"
)
.pipe(cmd!("grep", "ERROR"))
.pipe_stderr()
.output()?;
println!("   Filtered errors: {}", filtered_errors.trim());
Ok(())
```

Run with: `cargo run --example pipe_modes`

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

### pipeline_performance

```rust
println!("Pipeline Performance Demonstration");
println!("==================================\n");
// Test with a reasonably large dataset
let test_data = generate_test_data(10000);
println!("Testing with {} lines of data", test_data.lines().count());
println!("Data size: {} bytes\n", test_data.len());
// Test 1: Simple native pipeline
println!("Test 1: Simple text processing pipeline");
let start = Instant::now();
let result1 = cmd!("tr", "[:lower:]", "[:upper:]")
    .pipe(cmd!("sort"))
    .pipe(cmd!("uniq", "-c"))
    .input(&test_data)
    .output()?;
let duration1 = start.elapsed();
println!("Pipeline result: {} lines", result1.lines().count());
println!("Time taken: {:?}\n", duration1);
// Test 2: Memory efficiency comparison
println!("Test 2: Memory efficiency with large data streaming");
let large_data = generate_test_data(50000);
let start = Instant::now();
let result2 = cmd!("grep", "test")
    .pipe(cmd!("wc", "-l"))
    .input(&large_data)
    .output()?;
let duration2 = start.elapsed();
println!("Large data processing result: {}", result2.trim());
println!("Time taken: {:?}\n", duration2);
// Test 3: Complex pipeline with multiple stages
println!("Test 3: Complex multi-stage pipeline");
let start = Instant::now();
let result3 = cmd!("cat")
    .pipe(cmd!("grep", "data"))
    .pipe(cmd!("cut", "-d", ":", "-f", "2"))
    .pipe(cmd!("sort", "-n"))
    .pipe(cmd!("tail", "-5"))
    .input(&test_data)
    .output()?;
let duration3 = start.elapsed();
println!("Complex pipeline result: {} lines", result3.lines().count());
println!("Time taken: {:?}\n", duration3);
// Test 4: Demonstrate streaming vs buffering
println!("Test 4: Real-time processing demonstration");
let start = Instant::now();
// This would process data as it comes in, not waiting for all input
cmd!("head", "-100")
    .pipe(cmd!("nl"))
    .input(&test_data)
    .run()?;
let duration4 = start.elapsed();
println!("Streaming processing time: {:?}\n", duration4);
println!("Native Pipeline Features:");
println!("========================");
println!("✅ Memory efficient streaming processing");
println!("✅ Low process overhead with direct pipes");
println!("✅ Real-time data processing for large datasets");
println!("✅ Excellent error isolation and handling");
println!("✅ Platform-independent implementation");
Ok(())
```

Run with: `cargo run --example pipeline_performance`


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

