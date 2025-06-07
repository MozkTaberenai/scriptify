//! Advanced streaming I/O functionality demonstration
//!
//! This example demonstrates advanced streaming capabilities including:
//! - Large data processing with memory efficiency
//! - Reader/Writer streaming in complex pipelines
//! - Performance considerations for streaming operations

use scriptify::cmd;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing streaming IO improvements...");

    // Test 1: Basic streaming with byte input
    println!("\n1. Testing basic streaming with bytes:");
    let mut output = Vec::new();
    cmd!("echo", "hello world").stream_to(&mut output)?;
    println!("Output: {}", String::from_utf8_lossy(&output));

    // Test 2: Streaming with Reader input
    println!("\n2. Testing streaming with Reader input:");
    let input_data = "line1\nline2\nline3\n";
    let reader = Cursor::new(input_data.as_bytes().to_vec());

    let mut output = Vec::new();
    cmd!("cat").input_reader(reader).stream_to(&mut output)?;
    println!("Output: {}", String::from_utf8_lossy(&output));

    // Test 3: Large data streaming (simulate large input)
    println!("\n3. Testing large data streaming:");
    let large_data = "x".repeat(10000) + "\n";
    let reader = Cursor::new(large_data.into_bytes());

    let mut output = Vec::new();
    cmd!("wc", "-c")
        .input_reader(reader)
        .stream_to(&mut output)?;
    println!(
        "Character count: {}",
        String::from_utf8_lossy(&output).trim()
    );

    // Test 4: Pipeline with Reader input
    println!("\n4. Testing pipeline with Reader input:");
    let input_data = "apple\nbanana\ncherry\napple\nbanana\n";
    let reader = Cursor::new(input_data.as_bytes().to_vec());

    let mut output = Vec::new();
    cmd!("cat")
        .input_reader(reader)
        .pipe(cmd!("sort"))
        .pipe(cmd!("uniq", "-c"))
        .stream_to(&mut output)?;
    println!("Sorted and counted:\n{}", String::from_utf8_lossy(&output));

    // Test 5: Buffered reader
    println!("\n5. Testing buffered reader:");
    let input_data = "This is a test\nwith multiple lines\nfor buffered reading\n";
    let reader = Cursor::new(input_data.as_bytes().to_vec());

    let mut output = Vec::new();
    cmd!("grep", "test")
        .input_buffered(reader)
        .stream_to(&mut output)?;
    println!("Grep result: {}", String::from_utf8_lossy(&output));

    // Test 6: Memory efficiency comparison
    println!("\n6. Testing memory efficiency (no actual verification, just ensuring it works):");

    // Create a moderately large input stream
    let large_input = (0..1000)
        .map(|i| format!("line {}\n", i))
        .collect::<String>();
    let reader = Cursor::new(large_input.into_bytes());

    let mut output = Vec::new();
    cmd!("head", "-5")
        .input_reader(reader)
        .stream_to(&mut output)?;
    println!(
        "First 5 lines from large input:\n{}",
        String::from_utf8_lossy(&output)
    );

    println!("\nAll streaming IO tests completed successfully!");
    Ok(())
}
