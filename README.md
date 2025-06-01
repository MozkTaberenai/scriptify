# scriptify

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
- **Windows** ⚠️ Limited support with automatic fallback

**Note on Windows**: While the core functionality works on Windows, many examples and tests use Unix-specific commands (`ls`, `cat`, `tr`, `sort`, etc.) that are not available in standard Windows environments. The new native pipeline implementation automatically falls back to shell-based pipes on Windows for compatibility. Windows support could be improved in future versions with command mapping or by requiring tools like Git Bash or WSL.

### Requirements

- **Rust 1.87.0 or later** for optimal pipeline performance with `std::io::pipe`
- **Rust 1.70.0 or later** minimum (will use fallback shell-based pipes on older versions)

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
- **Automatic fallback**: Falls back to shell-based pipes if needed for compatibility

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
