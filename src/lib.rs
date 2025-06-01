//! # scriptify
//!
//! **Scriptify your Rust** - A simple and intuitive library that makes running shell commands and file operations easy and visible.
//!
//! ## Why scriptify?
//!
//! When you need to write system administration scripts, build tools, or automation in Rust,
//! you often find yourself wrestling with `std::process::Command` and `std::fs`. scriptify
//! provides a clean, shell-script-like interface while keeping all the benefits of Rust's
//! type safety and error handling.
//!
//! ### Key Features
//!
//! - **🎨 Colorful output**: See exactly what commands are being executed
//! - **🔗 Easy piping**: Chain commands together naturally
//! - **📁 File operations**: Wrapper around `std::fs` with automatic logging
//! - **🔧 Builder pattern**: Fluent API for command construction
//! - **⚡ Zero dependencies**: Only uses `anstyle` for colors
//! - **🛡️ Type safe**: All the safety of Rust with the convenience of shell scripts
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! scriptify = "0.1.0"
//! ```
//!
//! ## Platform Support
//!
//! Currently supported platforms:
//! - **Linux** ✅ Full support with native pipe optimization
//! - **macOS** ✅ Full support with native pipe optimization
//!
//! Scriptify is designed for Unix-like systems and uses Unix shell commands and utilities.
//!
//! ## Requirements
//!
//! - **Rust 1.87.0 or later** - Required for native pipeline performance with `std::io::pipe`
//!
//! ## Basic Usage
//!
//! ### Command Execution
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Simple command execution
//! cmd!("echo", "Hello, World!").run()?;
//!
//! // Get command output
//! let output = cmd!("date").output()?;
//! println!("Current date: {}", output.trim());
//!
//! // Command with multiple arguments
//! cmd!("ls", "-la", "/tmp").run()?;
//!
//! // Using the builder pattern
//! cmd!("grep", "error")
//!     .arg("logfile.txt")
//!     .current_dir("/var/log")
//!     .env("LANG", "C")
//!     .run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Command Piping
//!
//! Chain commands together just like in shell scripts. **New in Rust 1.87.0**: scriptify now uses native `std::io::pipe` for enhanced performance and memory efficiency!
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Simple pipe
//! cmd!("echo", "hello world")
//!     .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
//!     .run()?;
//!
//! // Multiple pipes - now using efficient native pipes!
//! cmd!("cat", "/etc/passwd")
//!     .pipe(cmd!("grep", "bash"))
//!     .pipe(cmd!("wc", "-l"))
//!     .run()?;
//!
//! // Get piped output with streaming processing
//! let result = cmd!("ps", "aux")
//!     .pipe(cmd!("grep", "rust"))
//!     .pipe(cmd!("wc", "-l"))
//!     .output()?;
//! echo!("Rust processes:", result.trim());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! #### Pipeline Performance Improvements (Rust 1.87.0+)
//!
//! - **Memory efficient**: Uses streaming instead of buffering all data
//! - **Better performance**: Native pipes reduce process overhead
//! - **Platform independent**: No shell dependency for multi-command pipes
//! - **Native implementation**: Uses `std::io::pipe` for optimal performance
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Large data processing with efficient streaming
//! let large_data = "..."; // Megabytes of data
//! let result = cmd!("grep", "pattern")
//!     .pipe(cmd!("sort"))
//!     .pipe(cmd!("uniq", "-c"))
//!     .input(large_data)
//!     .output()?; // Processes without loading all data into memory
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Tutorial: Learning Scriptify
//!
//! ### Getting Started with Your First Program
//!
//! Let's create a simple "Hello, World!" program that demonstrates scriptify's visibility:
//!
//! ```no_run
//! use scriptify::*;
//!
//! fn main() -> Result<()> {
//!     echo!("Hello, scriptify!");
//!     cmd!("echo", "Hello from the shell!").run()?;
//!     Ok(())
//! }
//! ```
//!
//! When you run this with `cargo run`, you'll see:
//! ```text
//! Hello, scriptify!
//! cmd echo "Hello from the shell!"
//! Hello from the shell!
//! ```
//!
//! Notice how scriptify shows you what commands it's executing - this is one of its key features for visibility and debugging.
//!
//! ### The cmd! Macro
//!
//! The heart of scriptify is the `cmd!` macro:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Basic command
//! cmd!("ls").run()?;
//!
//! // Command with arguments  
//! cmd!("ls", "-la").run()?;
//!
//! // Multiple arguments
//! cmd!("echo", "Hello", "World").run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### The echo! Macro
//!
//! Use `echo!` for formatted output with automatic spacing:
//!
//! ```no_run
//! use scriptify::*;
//!
//! echo!("Simple message");
//! echo!("Value:", 42);
//! echo!("Multiple", "arguments", "work", "too");
//!
//! let name = "Alice";
//! let age = 30;
//! echo!("User:", name, "Age:", age);
//! ```
//!
//! ### Builder Pattern
//!
//! Commands support a fluent builder pattern for complex configurations:
//!
//! ```no_run
//! use scriptify::*;
//!
//! cmd!("grep", "error")
//!     .arg("logfile.txt")
//!     .current_dir("/var/log")
//!     .env("LANG", "C")
//!     .quiet()
//!     .run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Input and Output
//!
//! Provide input to commands and capture their output:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Provide input to a command
//! let result = cmd!("sort")
//!     .input("banana\napple\ncherry\n")
//!     .output()?;
//! echo!("Sorted fruits:", result.trim());
//!
//! // Pipe with input
//! let result = cmd!("tr", "[:lower:]", "[:upper:]")
//!     .input("hello world")
//!     .output()?;
//! echo!("Uppercase:", result.trim());
//!
//! // Reading from files and processing
//! let file_content = fs::read_to_string("Cargo.toml")?;
//! let line_count = cmd!("wc", "-l")
//!     .input(&file_content)
//!     .output()?;
//! echo!("Lines in Cargo.toml:", line_count.trim());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Pipe Modes
//!
//! Control what streams are piped between commands:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Pipe stdout (default)
//! cmd!("echo", "data")
//!     .pipe(cmd!("sort"))
//!     .run()?;
//!
//! // Pipe stderr
//! cmd!("sh", "-c", "echo 'error message' >&2")
//!     .pipe_stderr(cmd!("grep", "ERROR"))
//!     .run()?;
//!
//! // Pipe both stdout and stderr
//! cmd!("sh", "-c", "echo 'stdout'; echo 'stderr' >&2")
//!     .pipe_both(cmd!("sort"))
//!     .run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Environment and Working Directory
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Set environment variables
//! cmd!("printenv", "MY_VAR")
//!     .env("MY_VAR", "Hello from Rust!")
//!     .run()?;
//!
//! // Change working directory
//! cmd!("pwd").current_dir("/tmp").run()?;
//!
//! // Combine multiple settings
//! cmd!("make", "install")
//!     .env("PREFIX", "/usr/local")
//!     .env("DESTDIR", "/tmp/staging")
//!     .current_dir("./my-project")
//!     .run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### File Operations
//!
//! All file operations are logged automatically:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Basic file operations
//! fs::write("config.txt", "debug=true\nport=8080")?;
//! let content = fs::read_to_string("config.txt")?;
//! echo!("Config content:", content);
//!
//! // Directory operations
//! fs::create_dir_all("project/src")?;
//! fs::copy("config.txt", "project/config.txt")?;
//!
//! // Directory traversal
//! for entry in fs::read_dir("project")? {
//!     let entry = entry?;
//!     let path = entry.path();
//!     if path.is_dir() {
//!         echo!("Directory:", path.display());
//!     } else {
//!         echo!("File:", path.display());
//!     }
//! }
//!
//! // Cleanup
//! fs::remove_file("config.txt")?;
//! fs::remove_dir_all("project")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Error Handling
//!
//! scriptify uses Rust's standard error handling patterns:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Handle command failures gracefully
//! match cmd!("nonexistent-command").run() {
//!     Ok(_) => echo!("Command succeeded"),
//!     Err(e) => echo!("Command failed:", e),
//! }
//!
//! // Check command availability
//! if cmd!("which", "git").run().is_ok() {
//!     echo!("Git is available");
//!     cmd!("git", "--version").run()?;
//! } else {
//!     echo!("Git not found - please install it");
//! }
//!
//! // Use the ? operator for early returns
//! fn deploy_app() -> Result<()> {
//!     cmd!("cargo", "build", "--release").run()?;
//!     cmd!("docker", "build", "-t", "myapp", ".").run()?;
//!     cmd!("docker", "push", "myapp").run()?;
//!     echo!("Deployment complete!");
//!     Ok(())
//! }
//! # deploy_app().ok();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Advanced Usage Patterns
//!
//! ### Environment Variables and Working Directory
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Set environment variables
//! cmd!("printenv", "MY_VAR")
//!     .env("MY_VAR", "Hello from Rust!")
//!     .run()?;
//!
//! // Change working directory
//! cmd!("pwd").current_dir("/tmp").run()?;
//!
//! // Combine multiple settings
//! cmd!("make", "install")
//!     .env("PREFIX", "/usr/local")
//!     .env("DESTDIR", "/tmp/staging")
//!     .current_dir("./my-project")
//!     .run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Quiet Mode
//!
//! Sometimes you don't want to see the command output:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Run silently
//! cmd!("git", "status").quiet().run()?;
//!
//! // Get output without showing the command
//! let output = cmd!("whoami").quiet().output()?;
//! echo!("Current user:", output.trim());
//!
//! // Global quiet mode using environment
//! unsafe {
//!     std::env::set_var("NO_ECHO", "1");
//! }
//! cmd!("echo", "This won't show the command").run()?;
//! unsafe {
//!     std::env::remove_var("NO_ECHO");
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Dynamic Command Building
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Build commands programmatically
//! let mut find_cmd = cmd!("find", "/var/log");
//! find_cmd = find_cmd.arg("-name").arg("*.log");
//! find_cmd = find_cmd.arg("-mtime").arg("+7");
//! find_cmd.run()?;
//!
//! // Conditional arguments
//! let mut cmd = cmd!("ls");
//! cmd = cmd.arg("-l");
//! if std::env::var("SHOW_ALL").is_ok() {
//!     cmd = cmd.arg("-a");
//! }
//! cmd.run()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Quiet Mode
//!
//! Sometimes you don't want to see the command output:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Run silently
//! cmd!("git", "status").quiet().run()?;
//!
//! // Get output without showing the command
//! let output = cmd!("whoami").quiet().output()?;
//! echo!("Current user:", output.trim());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Real-World Examples
//!
//! ### Build Script
//!
//! ```no_run
//! use scriptify::*;
//!
//! fn main() -> Result<()> {
//!     echo!("Building project...");
//!     
//!     // Clean previous build
//!     if fs::metadata("target").is_ok() {
//!         fs::remove_dir_all("target")?;
//!     }
//!     
//!     // Build in release mode
//!     cmd!("cargo", "build", "--release").run()?;
//!     
//!     // Run tests
//!     cmd!("cargo", "test").run()?;
//!     
//!     // Package the binary
//!     fs::create_dir_all("dist")?;
//!     fs::copy("target/release/myapp", "dist/myapp")?;
//!     
//!     echo!("Build complete! Binary available in dist/");
//!     Ok(())
//! }
//! ```
//!
//! ### Log Analysis
//!
//! ```no_run
//! use scriptify::*;
//!
//! fn analyze_logs() -> Result<()> {
//!     echo!("Analyzing web server logs...");
//!     
//!     // Count total requests
//!     let total = cmd!("wc", "-l")
//!         .input(&fs::read_to_string("/var/log/nginx/access.log")?)
//!         .output()?;
//!     echo!("Total requests:", total.trim());
//!     
//!     // Find top IPs
//!     let top_ips = cmd!("cut", "-d", " ", "-f", "1")
//!         .pipe(cmd!("sort"))
//!         .pipe(cmd!("uniq", "-c"))
//!         .pipe(cmd!("sort", "-nr"))
//!         .pipe(cmd!("head", "-10"))
//!         .input(&fs::read_to_string("/var/log/nginx/access.log")?)
//!         .output()?;
//!     
//!     echo!("Top 10 IPs:");
//!     echo!(top_ips);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### System Administration
//!
//! ```no_run
//! use scriptify::*;
//!
//! fn system_info() -> Result<()> {
//!     echo!("=== System Information ===");
//!     
//!     // OS information
//!     let os = cmd!("uname", "-a").output()?;
//!     echo!("OS:", os.trim());
//!     
//!     // Memory usage
//!     cmd!("free", "-h").run()?;
//!     
//!     // Disk usage
//!     cmd!("df", "-h").run()?;
//!     
//!     // Running processes
//!     let process_count = cmd!("ps", "aux")
//!         .pipe(cmd!("wc", "-l"))
//!         .output()?;
//!     echo!("Running processes:", process_count.trim());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Best Practices
//!
//! ### Error Handling
//!
//! Always handle potential failures appropriately:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Good: Handle errors appropriately
//! fn safe_operation() -> Result<()> {
//!     match cmd!("risky-command").run() {
//!         Ok(_) => echo!("Operation succeeded"),
//!         Err(e) => {
//!             echo!("Operation failed:", e);
//!             // Implement fallback or recovery
//!             cmd!("fallback-command").run()?;
//!         }
//!     }
//!     Ok(())
//! }
//! # safe_operation().ok();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Performance Considerations
//!
//! Use appropriate pipeline modes and avoid unnecessary operations:
//!
//! ```no_run
//! use scriptify::*;
//!
//! // Efficient: Stream processing
//! let result = cmd!("find", ".", "-name", "*.log")
//!     .pipe(cmd!("xargs", "grep", "ERROR"))
//!     .pipe(cmd!("wc", "-l"))
//!     .output()?;
//! echo!("Error count:", result.trim());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Cross-Platform Compatibility
//!
//! Check platform capabilities when needed:
//!
//! ```no_run
//! use scriptify::*;
//!
//! fn platform_aware_command() -> Result<()> {
//!     if cfg!(unix) {
//!         cmd!("ls", "-la").run()?;
//!     } else {
//!         cmd!("dir").run()?;
//!     }
//!     Ok(())
//! }
//! # platform_aware_command().ok();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Environment Variables
//!
//! You can control scriptify's behavior with environment variables:
//!
//! - `NO_ECHO`: Set to any value to suppress command echoing globally
//!
//! ```bash
//! NO_ECHO=1 cargo run  # Run without command echoing
//! ```
//!
//! ## Comparison with Alternatives
//!
//! | Feature | scriptify | std::process::Command | shell scripts |
//! |---------|-----------|----------------------|---------------|
//! | Type safety | ✅ | ✅ | ❌ |
//! | Error handling | ✅ | ✅ | ⚠️ |
//! | Piping | ✅ Native pipes (1.87+) | ⚠️ Manual | ✅ |
//! | Memory efficiency | ✅ Streaming | ❌ | ⚠️ |
//! | Visibility | ✅ | ❌ | ✅ |
//! | Cross-platform | ✅ | ✅ | ⚠️ |
//! | IDE support | ✅ | ✅ | ⚠️ |
//! | Debugging | ✅ | ✅ | ❌ |
//! | Performance | ✅ Optimized | ⚠️ | ⚠️ |
//!
//! ## Examples
//!
//! This crate includes comprehensive examples organized by skill level:
//!
//! ### Basic Examples (`examples/01_basics/`)
//! - `hello_world.rs` - First steps with scriptify
//! - `simple_commands.rs` - Basic command execution patterns
//! - `simple_pipes.rs` - Introduction to pipelines
//! - `simple_fs.rs` - File system operations
//!
//! ### Intermediate Examples (`examples/02_intermediate/`)
//! - `environment.rs` - Environment variables and working directories
//! - `error_handling.rs` - Robust error management
//! - `pipe_modes.rs` - Stdout/stderr piping control
//! - `mixed_pipe_modes.rs` - Mixed pipeline modes and combinations
//!
//! ### Advanced Examples (`examples/03_advanced/`)
//! - `complex_pipes.rs` - Advanced pipeline operations and patterns
//! - `command_quoting.rs` - Argument quoting and escaping for readability
//! - `control_char_demo.rs` - Control character handling demonstration
//!
//! Run any example with:
//! ```bash
//! cargo run --example hello_world
//! cargo run --example simple_commands
//! cargo run --example pipe_modes
//! cargo run --example command_quoting
//! cargo run --example control_char_demo
//! # ... etc
//! ```
//!
//! Start with `hello_world.rs` and progress through the examples in order for the best learning experience.
//!
//! ## Contributing
//!
//! We welcome contributions! Please see our [GitHub repository](https://github.com/MozkTaberenai/scriptify) for more information.
//!
//! ## License
//!
//! This project is licensed under the MIT License.

#[doc(no_inline)]
pub use std::ffi::{OsStr, OsString};
#[doc(no_inline)]
pub use std::io::{BufReader, BufWriter, prelude::*};
#[doc(no_inline)]
pub use std::path::{Path, PathBuf};

mod cmd;
pub use cmd::*;

pub mod fs;

mod echo;
pub use echo::*;

pub mod color;
mod style;

/// Result type with a boxed error for convenience
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
