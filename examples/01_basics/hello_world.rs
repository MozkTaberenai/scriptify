//! # Hello World - Scriptify Beginner Example
//!
//! This example demonstrates the most basic scriptify features:
//! - Simple command execution
//! - Using standard print macros
//! - Basic error handling
//!
//! Estimated time: ~1 minute
//! Target audience: Rust beginners, scriptify newcomers

use scriptify::*;

fn main() -> Result<()> {
    // 🎯 Most basic command execution
    println!("🚀 Welcome to scriptify!");
    println!("This tutorial teaches basic command execution\n");

    // Simple echo command
    println!("1. Basic command execution:");
    cmd!("echo", "Hello, scriptify!").run()?;

    // Command with multiple arguments
    println!("\n2. Multiple arguments:");
    cmd!("echo", "arg1", "arg2", "arg3").run()?;

    // Capturing output
    println!("\n3. Capturing command output:");
    let output = cmd!("echo", "This output will be stored in a variable").output()?;
    println!("Captured output: {}", output.trim());

    // Using standard print macros
    println!("\n4. Using standard print macros:");
    println!("✅ Command execution completed!");
    println!("📝 You can use println! with formatted arguments like this: arg1 arg2 arg3");

    println!("\n🎉 Hello World tutorial completed!");
    println!("Next, try simple_commands.rs to learn more");

    Ok(())
}
