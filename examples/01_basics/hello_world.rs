//! # Hello World - Scriptify Beginner Example
//!
//! This example demonstrates the most basic scriptify features:
//! - Simple command execution
//! - Using the echo! macro
//! - Basic error handling
//!
//! Estimated time: ~1 minute
//! Target audience: Rust beginners, scriptify newcomers

use scriptify::*;

fn main() -> Result<()> {
    // 🎯 Most basic command execution
    echo!("🚀 Welcome to scriptify!");
    echo!("This tutorial teaches basic command execution\n");

    // Simple echo command
    echo!("1. Basic command execution:");
    cmd!("echo", "Hello, scriptify!").run()?;

    // Command with multiple arguments
    echo!("\n2. Multiple arguments:");
    cmd!("echo", "arg1", "arg2", "arg3").run()?;

    // Capturing output
    echo!("\n3. Capturing command output:");
    let output = cmd!("echo", "This output will be stored in a variable").output()?;
    echo!("Captured output:", output.trim());

    // Using the echo! macro
    echo!("\n4. Using the echo! macro:");
    echo!("✅ Command execution completed!");
    echo!(
        "📝 You can use echo! with",
        "multiple",
        "arguments",
        "like this"
    );

    echo!("\n🎉 Hello World tutorial completed!");
    echo!("Next, try simple_commands.rs to learn more");

    Ok(())
}
