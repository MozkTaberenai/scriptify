//! # Simple Commands - Basic Command Execution
//!
//! This example demonstrates various basic command execution patterns:
//! - Different types of commands
//! - Success/failure handling
//! - Output capture and processing
//!
//! Estimated time: ~2 minutes
//! Prerequisites: Complete hello_world.rs

use scriptify::*;

fn main() -> Result<()> {
    println!("📚 Learning Basic Command Execution");
    println!("=================================\n");

    // 1. Information gathering commands
    println!("1. System information gathering:");
    demonstrate_info_commands()?;

    // 2. Text processing commands
    println!("\n2. Text processing:");
    demonstrate_text_commands()?;

    // 3. Error handling
    println!("\n3. Error handling:");
    demonstrate_error_handling()?;

    println!("\n🎉 Basic commands tutorial completed!");
    println!("Next, try simple_pipes.rs to learn about pipelines");

    Ok(())
}

fn demonstrate_info_commands() -> Result<()> {
    // Date and time
    let date_output = cmd!("date").output()?;
    println!("📅 Current date/time: {}", date_output.trim());

    // Current directory
    let pwd_output = cmd!("pwd").output()?;
    println!("📁 Current directory: {}", pwd_output.trim());

    // User information
    if let Ok(user) = cmd!("whoami").output() {
        println!("👤 Current user: {}", user.trim());
    }

    Ok(())
}

fn demonstrate_text_commands() -> Result<()> {
    // Uppercase conversion
    let uppercase = cmd!("echo", "hello scriptify")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .output()?;
    println!("🔤 Uppercase conversion: {}", uppercase.trim());

    // Character count
    let char_count = cmd!("echo", "scriptify").pipe(cmd!("wc", "-c")).output()?;
    println!("🔢 Character count: {} characters", char_count.trim());

    Ok(())
}

fn demonstrate_error_handling() -> Result<()> {
    // Successful command
    println!("✅ Successful command:");
    cmd!("echo", "This command will succeed").run()?;

    // Safely handle potentially failing commands
    println!("⚠️ Error handling example:");
    match cmd!("nonexistent_command").run() {
        Ok(_) => println!("Command succeeded"),
        Err(err) => println!("Command failed: {}", err),
    }

    println!("✅ Error handling completed");

    Ok(())
}
