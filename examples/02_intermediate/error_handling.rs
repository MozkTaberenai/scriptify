//! # Error Handling - Robust Error Management Best Practices
//!
//! This example demonstrates robust error handling techniques with scriptify:
//! - Basic error handling patterns
//! - Output capture error handling
//! - Conditional error processing
//!
//! Estimated time: ~3 minutes
//! Prerequisites: Complete all examples in 01_basics/

use scriptify::*;

fn main() -> Result<()> {
    echo!("🛡️ Error Handling Best Practices");
    echo!("===============================\n");

    // 1. Basic error handling patterns
    echo!("1. Basic error handling:");
    basic_error_handling()?;

    // 2. Output capture error handling
    echo!("\n2. Output capture error handling:");
    output_capture_error_handling()?;

    // 3. Conditional error handling
    echo!("\n3. Conditional error handling:");
    conditional_error_handling()?;

    echo!("\n🎉 Error handling tutorial completed!");
    Ok(())
}

fn basic_error_handling() -> Result<()> {
    echo!("📝 Basic error handling patterns:");

    // Pattern 1: Using match statement
    echo!("\n🔍 Pattern 1: Match statement handling");
    match cmd!("nonexistent_command").run() {
        Ok(_) => echo!("✅ Command succeeded"),
        Err(e) => echo!("❌ Command failed:", e),
    }

    // Pattern 2: Using if let
    echo!("\n🔍 Pattern 2: if let handling");
    if let Err(e) = cmd!("another_nonexistent_command").quiet().run() {
        echo!("❌ Silent failure:", e);
    }

    // Pattern 3: Using unwrap_or_else
    echo!("\n🔍 Pattern 3: Default value handling");
    let output = cmd!("nonexistent_command")
        .quiet()
        .output()
        .unwrap_or_else(|_| "default value".to_string());
    echo!("📤 Output (using default):", output);

    Ok(())
}

fn output_capture_error_handling() -> Result<()> {
    echo!("📤 Output capture error handling:");

    // Handle successful output capture
    match cmd!("echo", "Hello, scriptify!").output() {
        Ok(output) => echo!("✅ Output:", output.trim()),
        Err(e) => echo!("❌ Failed to capture output:", e),
    }

    // Handle output capture from failing command
    match cmd!("sh", "-c", "echo 'stdout'; echo 'stderr' >&2; exit 1")
        .quiet()
        .output()
    {
        Ok(output) => echo!("✅ Output:", output),
        Err(e) => echo!("❌ Output capture failed:", e),
    }

    Ok(())
}

fn conditional_error_handling() -> Result<()> {
    echo!("🔀 Conditional error handling:");

    // Command availability checking
    echo!("🖥️ Command availability checking:");

    // Check if a command exists before using it
    match cmd!("which", "git").quiet().run() {
        Ok(_) => {
            echo!("✅ Git is available");
            cmd!("git", "--version").run()?;
        }
        Err(_) => {
            echo!("⚠️ Git not found");
            echo!("💡 Continuing without git functionality");
        }
    }

    // File existence checking with error handling
    echo!("\n📁 File existence checking:");
    let test_files = ["Cargo.toml", "nonexistent.txt", "README.md"];

    for file in &test_files {
        match fs::metadata(file) {
            Ok(_) => println!("✅ {} exists", file),
            Err(_) => println!("❌ {} does not exist", file),
        }
    }

    // Graceful degradation example with file operations
    echo!("\n🔧 Graceful degradation:");
    match cmd!("cat", "nonexistent.txt").quiet().output() {
        Ok(content) => {
            echo!("✅ File read successfully");
            echo!(
                "Content preview:",
                &content[..std::cmp::min(50, content.len())]
            );
        }
        Err(_) => {
            echo!("⚠️ File not found, using default behavior");
            echo!("💡 Continuing with default configuration");
        }
    }

    Ok(())
}
