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
    echo!("🌍 Environment Variables and Directory Management");
    echo!("===============================================\n");

    // 1. Environment variable operations
    echo!("1. Environment variable operations:");
    environment_variables()?;

    // 2. Working directory management
    echo!("\n2. Working directory management:");
    working_directory_management()?;

    echo!("\n🎉 Environment tutorial completed!");
    Ok(())
}

fn environment_variables() -> Result<()> {
    echo!("🔧 Environment variable operations:");

    // Multiple environment variables
    echo!("\n📝 Multiple environment variables:");
    cmd!("sh", "-c", "echo \"Name: $NAME, Version: $VERSION\"")
        .env("NAME", "scriptify")
        .env("VERSION", "0.1.0")
        .run()?;

    // Reading current environment
    echo!("\n🔍 Reading current environment:");
    if let Ok(current_user) = env::var("USER") {
        echo!("Current USER:", current_user);
    }

    if let Ok(current_path) = env::var("PATH") {
        let path_count = current_path.split(':').count();
        println!("PATH contains {} directories", path_count);
    }

    Ok(())
}

fn working_directory_management() -> Result<()> {
    echo!("📁 Working directory management:");

    // Get current directory
    let original_dir = env::current_dir()?;
    echo!("Original directory:", original_dir.display());

    // Create temporary working directory
    fs::create_dir_all("temp_work")?;
    echo!("Created temporary directory: temp_work");

    // Execute command in different directory
    echo!("\n🔄 Executing commands in different directories:");
    cmd!("pwd").current_dir("temp_work").run()?;

    // Chain operations in specific directory
    echo!("\n⛓️ Chained operations in specific directory:");
    cmd!("echo", "Hello from temp directory")
        .current_dir("temp_work")
        .run()?;

    // Create files in specific directory
    echo!("\n📄 Creating files in specific directory:");
    let file_content = cmd!("echo", "File content")
        .current_dir("temp_work")
        .output()?;
    fs::write("temp_work/test.txt", file_content)?;

    // Verify file creation
    cmd!("ls", "-la").current_dir("temp_work").run()?;

    // Cleanup
    fs::remove_dir_all("temp_work")?;
    echo!("✅ Cleaned up temporary directory");

    Ok(())
}
