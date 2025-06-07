//! Demonstrates Reader/Writer functionality in scriptify
//!
//! This example shows how to use std::io::Read and Write traits
//! with scriptify commands for flexible input/output handling.

use scriptify::*;
use std::fs::File;
use std::io::Cursor;

fn main() -> Result<()> {
    println!("=== Reader/Writer Demo ===\n");

    // Example 1: Using a string as input via Cursor (implements Read)
    println!("1. Using Cursor (in-memory reader):");
    let input_data = "apple\nbanana\ncherry\napricot";
    let cursor = Cursor::new(input_data);

    let result = cmd!("grep", "ap").input_reader(cursor).output()?;
    println!("Lines containing 'ap': {}", result.trim());

    // Example 2: Using buffered file input
    println!("\n2. Creating and reading from a file:");

    // First create a test file
    std::fs::write(
        "test_input.txt",
        "line 1\nline 2\nline 3\nspecial line\nline 5",
    )?;

    let file = File::open("test_input.txt")?;
    let result = cmd!("grep", "special").input_buffered(file).output()?;
    println!("Found: {}", result.trim());

    // Example 3: Streaming output to a file
    println!("\n3. Streaming output directly to a file:");
    let output_file = File::create("output.txt")?;
    cmd!("echo", "Hello, World!\nThis goes to file").stream_to(output_file)?;

    let content = std::fs::read_to_string("output.txt")?;
    println!("File content: {}", content.trim());

    // Example 4: Using Vec<u8> as a writer (implements Write)
    println!("\n4. Capturing output in memory buffer:");
    let mut buffer = Vec::new();
    cmd!("echo", "-n", "buffered output").stream_to(&mut buffer)?;
    println!("Buffer contains: '{}'", String::from_utf8_lossy(&buffer));

    // Example 5: Chain reader and writer together
    println!("\n5. Full Reader -> Command -> Writer pipeline:");
    let input_file = File::open("test_input.txt")?;
    let output_file = File::create("filtered_output.txt")?;

    cmd!("sort").run_with_io(input_file, output_file)?;

    let sorted_content = std::fs::read_to_string("filtered_output.txt")?;
    println!("Sorted content:\n{}", sorted_content);

    // Example 6: Using pipeline with readers/writers
    println!("\n6. Pipeline with Reader/Writer:");
    let input_data = "zebra\napple\nbanana\ncherry";
    let cursor = Cursor::new(input_data);
    let mut output_buffer = Vec::new();

    cmd!("sort")
        .pipe(cmd!("head", "-2"))
        .run_with_io(cursor, &mut output_buffer)?;

    println!(
        "First 2 sorted items: {}",
        String::from_utf8_lossy(&output_buffer).trim()
    );

    // Cleanup
    std::fs::remove_file("test_input.txt").ok();
    std::fs::remove_file("output.txt").ok();
    std::fs::remove_file("filtered_output.txt").ok();

    println!("\n=== Demo completed ===");
    Ok(())
}
