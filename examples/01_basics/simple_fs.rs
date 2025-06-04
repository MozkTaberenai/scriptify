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
    println!("📁 File System Operations Basics");
    println!("===============================\n");

    // 1. Creating directories and files
    println!("1. Creating directories and files:");
    create_files_and_dirs()?;

    // 2. File operations
    println!("\n2. File operations:");
    file_operations()?;

    // 3. Checking file information
    println!("\n3. Checking file information:");
    check_file_info()?;

    // 4. Cleanup
    println!("\n4. Cleanup:");
    cleanup()?;

    println!("\n🎉 Basic file system tutorial completed!");
    println!("Next, explore 02_intermediate/ examples for advanced features");

    Ok(())
}

fn create_files_and_dirs() -> Result<()> {
    // Create working directory
    println!("📁 Creating working directory 'demo':");
    fs::create_dir_all("demo")?;

    // Create subdirectories
    println!("📁 Creating subdirectories:");
    fs::create_dir_all("demo/subdir")?;

    // Create files
    println!("📄 Creating text files:");
    fs::write("demo/hello.txt", "Hello, scriptify!\nThis is a demo file.")?;
    fs::write("demo/numbers.txt", "1\n2\n3\n4\n5")?;

    println!("✅ Files and directories created successfully");
    Ok(())
}

fn file_operations() -> Result<()> {
    // Read file contents
    println!("📖 Reading file contents:");
    let content = fs::read_to_string("demo/hello.txt")?;
    println!("hello.txt contents:\n{}", content);

    // Copy file
    println!("📋 Copying file:");
    fs::copy("demo/hello.txt", "demo/hello_copy.txt")?;

    // Move file (rename)
    println!("📦 Moving file:");
    fs::rename("demo/hello_copy.txt", "demo/subdir/moved_file.txt")?;

    println!("✅ File operations completed");
    Ok(())
}

fn check_file_info() -> Result<()> {
    // List directory contents
    println!("📋 Checking directory contents:");

    println!("Contents of demo/:");
    for entry in fs::read_dir("demo")? {
        let entry = entry?;
        let path = entry.path();
        let file_type = if path.is_dir() { "📁" } else { "📄" };
        println!("  {} {}", file_type, path.display());
    }

    println!("Contents of demo/subdir/:");
    for entry in fs::read_dir("demo/subdir")? {
        let entry = entry?;
        let path = entry.path();
        let file_type = if path.is_dir() { "📁" } else { "📄" };
        println!("  {} {}", file_type, path.display());
    }

    println!("✅ Directory listing completed");
    Ok(())
}

fn cleanup() -> Result<()> {
    println!("🧹 Cleaning up working files:");

    // Remove created files and directories
    if Path::new("demo").exists() {
        fs::remove_dir_all("demo")?;
        println!("✅ Removed demo directory");
    }

    println!("🎉 Cleanup completed");
    Ok(())
}
