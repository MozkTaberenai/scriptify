//! # Environment - Working with Environment Variables and Directories
//!
//! This example demonstrates advanced environment manipulation with scriptify:
//! - Setting and using environment variables
//! - Working directory management
//!
//! Estimated time: ~2 minutes
//! Prerequisites: Complete error_handling.rs

use scriptify::*;
use std::env;

fn main() -> Result<()> {
    println!("🌍 Environment Variables and Directory Management");
    println!("===============================================\n");

    // 1. Environment variable operations
    println!("1. Environment variable operations:");
    environment_variables()?;

    // 2. Working directory management
    println!("\n2. Working directory management:");
    working_directory_management()?;

    println!("\n🎉 Environment tutorial completed!");
    Ok(())
}

fn environment_variables() -> Result<()> {
    println!("🔧 Environment variable operations:");

    // Multiple environment variables
    println!("\n📝 Multiple environment variables:");
    cmd!("sh", "-c", "echo \"Name: $NAME, Version: $VERSION\"")
        .env("NAME", "scriptify")
        .env("VERSION", "0.1.0")
        .run()?;

    // Reading current environment
    println!("\n🔍 Reading current environment:");
    if let Ok(current_user) = env::var("USER") {
        println!("Current USER: {}", current_user);
    }

    if let Ok(current_path) = env::var("PATH") {
        let path_count = current_path.split(':').count();
        println!("PATH contains {} directories", path_count);
    }

    Ok(())
}

fn working_directory_management() -> Result<()> {
    println!("📁 Working directory management:");

    // Get current directory
    let original_dir = env::current_dir()?;
    println!("Original directory: {}", original_dir.display());

    // Create temporary working directory
    fs::create_dir_all("temp_work")?;
    println!("Created temporary directory: temp_work");

    // Execute command in different directory
    println!("\n🔄 Executing commands in different directories:");
    cmd!("pwd").current_dir("temp_work").run()?;

    // Chain operations in specific directory
    println!("\n⛓️ Chained operations in specific directory:");
    cmd!("echo", "Hello from temp directory")
        .current_dir("temp_work")
        .run()?;

    // Create files in specific directory
    println!("\n📄 Creating files in specific directory:");
    let file_content = cmd!("echo", "File content")
        .current_dir("temp_work")
        .output()?;
    fs::write("temp_work/test.txt", file_content)?;

    // Verify file creation
    cmd!("ls", "-la").current_dir("temp_work").run()?;

    // Cleanup
    fs::remove_dir_all("temp_work")?;
    println!("✅ Cleaned up temporary directory");

    Ok(())
}
