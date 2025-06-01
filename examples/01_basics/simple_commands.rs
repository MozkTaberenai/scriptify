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
    echo!("📚 Learning Basic Command Execution");
    echo!("=================================\n");

    // 1. Information gathering commands
    echo!("1. System information gathering:");
    demonstrate_info_commands()?;

    // 2. Text processing commands
    echo!("\n2. Text processing:");
    demonstrate_text_commands()?;

    // 3. Error handling
    echo!("\n3. Error handling:");
    demonstrate_error_handling()?;

    echo!("\n🎉 Basic commands tutorial completed!");
    echo!("Next, try simple_pipes.rs to learn about pipelines");

    Ok(())
}

fn demonstrate_info_commands() -> Result<()> {
    // Date and time
    let date_output = cmd!("date").output()?;
    echo!("📅 Current date/time:", date_output.trim());

    // Current directory
    let pwd_output = cmd!("pwd").output()?;
    echo!("📁 Current directory:", pwd_output.trim());

    // User information
    if let Ok(user) = cmd!("whoami").output() {
        echo!("👤 Current user:", user.trim());
    }

    Ok(())
}

fn demonstrate_text_commands() -> Result<()> {
    // Uppercase conversion
    let uppercase = cmd!("echo", "hello scriptify")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .output()?;
    echo!("🔤 Uppercase conversion:", uppercase.trim());

    // Character count
    let char_count = cmd!("echo", "scriptify").pipe(cmd!("wc", "-c")).output()?;
    echo!("🔢 Character count:", char_count.trim(), "characters");

    Ok(())
}

fn demonstrate_error_handling() -> Result<()> {
    // Successful command
    echo!("✅ Successful command:");
    cmd!("echo", "This command will succeed").run()?;

    // Safely handle potentially failing commands
    echo!("⚠️ Error handling example:");
    match cmd!("nonexistent_command").run() {
        Ok(_) => echo!("Command succeeded"),
        Err(err) => echo!("Command failed:", err.to_string()),
    }

    echo!("✅ Error handling completed");

    Ok(())
}
