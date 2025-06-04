//! # Simple Pipes - Basic Pipeline Operations
//!
//! This example demonstrates command pipeline connections:
//! - Connecting two commands
//! - Chaining multiple commands
//! - Pipeline output capture
//!
//! Estimated time: ~3 minutes
//! Prerequisites: Complete simple_commands.rs

use scriptify::*;

fn main() -> Result<()> {
    println!("🔗 Pipeline Basics");
    println!("==================\n");

    // 1. Basic two-command pipes
    println!("1. Basic pipelines:");
    basic_pipes()?;

    // 2. Multiple command chains
    println!("\n2. Multiple command chains:");
    multiple_pipes()?;

    // 3. Input data processing
    println!("\n3. Input data processing:");
    input_processing()?;

    println!("\n🎉 Basic pipeline tutorial completed!");
    println!("Next, try simple_fs.rs to learn file operations");

    Ok(())
}

fn basic_pipes() -> Result<()> {
    // Convert text to uppercase
    println!("📝 Convert text to uppercase:");
    cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .run()?;

    // Reverse string
    println!("\n🔄 Reverse string:");
    cmd!("echo", "scriptify").pipe(cmd!("rev")).run()?;

    // Count words
    println!("\n🔢 Count words:");
    let word_count = cmd!("echo", "Hello beautiful scriptify world")
        .pipe(cmd!("wc", "-w"))
        .output()?;
    println!("Word count: {} words", word_count.trim());

    Ok(())
}

fn multiple_pipes() -> Result<()> {
    // Chain three commands
    println!("🔗 Chain three commands:");
    cmd!("echo", "Hello World")
        .pipe(cmd!("tr", "[:upper:]", "[:lower:]")) // to lowercase
        .pipe(cmd!("rev")) // reverse
        .run()?;

    // Data transformation chain
    println!("\n🔄 Data transformation chain:");
    let result = cmd!("echo", "apple,banana,cherry")
        .pipe(cmd!("tr", ",", "\n")) // comma to newline
        .pipe(cmd!("sort")) // sort lines
        .pipe(cmd!("tr", "\n", " ")) // newline to space
        .output()?;
    println!("Sorted result: {}", result.trim());

    Ok(())
}

fn input_processing() -> Result<()> {
    // Process input data through pipeline
    println!("📄 Input data processing:");

    let data = "orange\napple\nbanana\napple\ncherry\nbanana";
    println!("Original data:\n{}", data);

    // Remove duplicates and sort
    let unique_sorted = cmd!("sort").pipe(cmd!("uniq")).input(data).output()?;
    println!("\nUnique + sorted result:");
    println!("{}", unique_sorted.trim());

    // Count lines
    let line_count = cmd!("wc", "-l").input(data).output()?;
    println!("Total lines: {} lines", line_count.trim());

    Ok(())
}
