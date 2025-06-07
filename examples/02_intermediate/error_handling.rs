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
    println!("🛡️ Error Handling Best Practices");
    println!("===============================\n");

    // 1. Basic error handling patterns
    println!("1. Basic error handling:");
    basic_error_handling()?;

    // 2. Output capture error handling
    println!("\n2. Output capture error handling:");
    output_capture_error_handling()?;

    // 3. Conditional error handling
    println!("\n3. Conditional error handling:");
    conditional_error_handling()?;

    println!("\n🎉 Error handling tutorial completed!");
    Ok(())
}

fn basic_error_handling() -> Result<()> {
    println!("📝 Basic error handling patterns:");

    // Pattern 1: Using match statement
    println!("\n🔍 Pattern 1: Match statement handling");
    match cmd!("nonexistent_command").run() {
        Ok(_) => println!("✅ Command succeeded"),
        Err(e) => println!("❌ Command failed: {}", e),
    }

    // Pattern 2: Using if let
    println!("\n🔍 Pattern 2: if let handling");
    if let Err(e) = cmd!("another_nonexistent_command").no_echo().run() {
        println!("❌ Silent failure: {}", e);
    }

    // Pattern 3: Using unwrap_or_else
    println!("\n🔍 Pattern 3: Default value handling");
    let output = cmd!("nonexistent_command")
        .no_echo()
        .output()
        .unwrap_or_else(|_| "default value".to_string());
    println!("📤 Output (using default): {}", output);

    Ok(())
}

fn output_capture_error_handling() -> Result<()> {
    println!("📤 Output capture error handling:");

    // Handle successful output capture
    match cmd!("echo", "Hello, scriptify!").output() {
        Ok(output) => println!("✅ Output: {}", output.trim()),
        Err(e) => println!("❌ Failed to capture output: {}", e),
    }

    // Handle output capture from failing command
    match cmd!("sh", "-c", "echo 'stdout'; echo 'stderr' >&2; exit 1")
        .no_echo()
        .output()
    {
        Ok(output) => println!("✅ Output: {}", output),
        Err(e) => println!("❌ Output capture failed: {}", e),
    }

    Ok(())
}

fn conditional_error_handling() -> Result<()> {
    println!("🔀 Conditional error handling:");

    // Command availability checking
    println!("🖥️ Command availability checking:");

    // Check if a command exists before using it
    match cmd!("which", "git").no_echo().run() {
        Ok(_) => {
            println!("✅ Git is available");
            cmd!("git", "--version").run()?;
        }
        Err(_) => {
            println!("⚠️ Git not found");
            println!("💡 Continuing without git functionality");
        }
    }

    // File existence checking with error handling
    println!("\n📁 File existence checking:");
    let test_files = ["Cargo.toml", "nonexistent.txt", "README.md"];

    for file in &test_files {
        match fs::metadata(file) {
            Ok(_) => println!("✅ {} exists", file),
            Err(_) => println!("❌ {} does not exist", file),
        }
    }

    // Graceful degradation example with file operations
    println!("\n🔧 Graceful degradation:");
    match cmd!("cat", "nonexistent.txt").no_echo().output() {
        Ok(content) => {
            println!("✅ File read successfully");
            println!(
                "Content preview: {}",
                &content[..std::cmp::min(50, content.len())]
            );
        }
        Err(_) => {
            println!("⚠️ File not found, using default behavior");
            println!("💡 Continuing with default configuration");
        }
    }

    Ok(())
}
