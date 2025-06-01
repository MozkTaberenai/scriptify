//! # Simple File System - Basic File Operations
//!
//! This example demonstrates basic file operations using scriptify:
//! - Creating and writing files
//! - Creating directories
//! - Copying and moving files
//! - Safe cleanup
//!
//! Estimated time: ~3 minutes
//! Prerequisites: Complete simple_pipes.rs

use scriptify::*;
use std::path::Path;

fn main() -> Result<()> {
    echo!("📁 File System Operations Basics");
    echo!("===============================\n");

    // 1. Creating directories and files
    echo!("1. Creating directories and files:");
    create_files_and_dirs()?;

    // 2. File operations
    echo!("\n2. File operations:");
    file_operations()?;

    // 3. Checking file information
    echo!("\n3. Checking file information:");
    check_file_info()?;

    // 4. Cleanup
    echo!("\n4. Cleanup:");
    cleanup()?;

    echo!("\n🎉 Basic file system tutorial completed!");
    echo!("Next, explore 02_intermediate/ examples for advanced features");

    Ok(())
}

fn create_files_and_dirs() -> Result<()> {
    // Create working directory
    echo!("📁 Creating working directory 'demo':");
    fs::create_dir_all("demo")?;

    // Create subdirectories
    echo!("📁 Creating subdirectories:");
    fs::create_dir_all("demo/subdir")?;

    // Create files
    echo!("📄 Creating text files:");
    fs::write("demo/hello.txt", "Hello, scriptify!\nThis is a test file.")?;
    fs::write("demo/numbers.txt", "1\n2\n3\n4\n5")?;

    echo!("✅ Files and directories created successfully");
    Ok(())
}

fn file_operations() -> Result<()> {
    // Read file contents
    echo!("📖 Reading file contents:");
    let content = fs::read_to_string("demo/hello.txt")?;
    println!("hello.txt contents:\n{}", content);

    // Copy file
    echo!("📋 Copying file:");
    fs::copy("demo/hello.txt", "demo/hello_copy.txt")?;

    // Move file (rename)
    echo!("📦 Moving file:");
    fs::rename("demo/hello_copy.txt", "demo/subdir/moved_file.txt")?;

    Ok(())
}

fn check_file_info() -> Result<()> {
    // List directory contents
    echo!("📋 Checking directory contents:");

    echo!("Contents of demo/:");
    for entry in fs::read_dir("demo")? {
        let entry = entry?;
        let path = entry.path();
        let file_type = if path.is_dir() { "📁" } else { "📄" };
        println!("  {} {}", file_type, path.display());
    }

    echo!("Contents of demo/subdir/:");
    for entry in fs::read_dir("demo/subdir")? {
        let entry = entry?;
        let path = entry.path();
        let file_type = if path.is_dir() { "📁" } else { "📄" };
        println!("  {} {}", file_type, path.display());
    }

    // Check file size
    if let Ok(metadata) = std::fs::metadata("demo/hello.txt") {
        println!("📊 hello.txt size: {} bytes", metadata.len());
    }

    Ok(())
}

fn cleanup() -> Result<()> {
    echo!("🧹 Cleaning up working files:");

    // Remove created files and directories
    if Path::new("demo").exists() {
        fs::remove_dir_all("demo")?;
        echo!("✅ Removed demo directory");
    }

    echo!("🎉 Cleanup completed");
    Ok(())
}
