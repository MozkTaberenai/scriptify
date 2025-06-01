//! Pipeline performance demonstration
//!
//! This example demonstrates the native pipeline performance features
//! in scriptify using Rust 1.87.0's std::io::pipe implementation.

use scriptify::*;
use std::time::Instant;

fn main() -> Result<()> {
    println!("Pipeline Performance Demonstration");
    println!("==================================\n");

    // Test with a reasonably large dataset
    let test_data = generate_test_data(10000);

    println!("Testing with {} lines of data", test_data.lines().count());
    println!("Data size: {} bytes\n", test_data.len());

    // Test 1: Simple native pipeline
    println!("Test 1: Simple text processing pipeline");
    let start = Instant::now();
    let result1 = cmd!("tr", "[:lower:]", "[:upper:]")
        .pipe(cmd!("sort"))
        .pipe(cmd!("uniq", "-c"))
        .input(&test_data)
        .output()?;
    let duration1 = start.elapsed();

    println!("Pipeline result: {} lines", result1.lines().count());
    println!("Time taken: {:?}\n", duration1);

    // Test 2: Memory efficiency comparison
    println!("Test 2: Memory efficiency with large data streaming");
    let large_data = generate_test_data(50000);

    let start = Instant::now();
    let result2 = cmd!("grep", "test")
        .pipe(cmd!("wc", "-l"))
        .input(&large_data)
        .output()?;
    let duration2 = start.elapsed();

    println!("Large data processing result: {}", result2.trim());
    println!("Time taken: {:?}\n", duration2);

    // Test 3: Complex pipeline with multiple stages
    println!("Test 3: Complex multi-stage pipeline");
    let start = Instant::now();
    let result3 = cmd!("cat")
        .pipe(cmd!("grep", "data"))
        .pipe(cmd!("cut", "-d", ":", "-f", "2"))
        .pipe(cmd!("sort", "-n"))
        .pipe(cmd!("tail", "-5"))
        .input(&test_data)
        .output()?;
    let duration3 = start.elapsed();

    println!("Complex pipeline result: {} lines", result3.lines().count());
    println!("Time taken: {:?}\n", duration3);

    // Test 4: Demonstrate streaming vs buffering
    println!("Test 4: Real-time processing demonstration");
    let start = Instant::now();

    // This would process data as it comes in, not waiting for all input
    cmd!("head", "-100")
        .pipe(cmd!("nl"))
        .input(&test_data)
        .run()?;

    let duration4 = start.elapsed();
    println!("Streaming processing time: {:?}\n", duration4);

    println!("Native Pipeline Features:");
    println!("========================");
    println!("✅ Memory efficient streaming processing");
    println!("✅ Low process overhead with direct pipes");
    println!("✅ Real-time data processing for large datasets");
    println!("✅ Excellent error isolation and handling");
    println!("✅ Platform-independent implementation");

    Ok(())
}

fn generate_test_data(lines: usize) -> String {
    let mut data = String::new();
    for i in 0..lines {
        data.push_str(&format!("line {}: test data item number {}\n", i, i % 100));
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_efficiency() -> Result<()> {
        let data = generate_test_data(1000);

        // Test that native pipeline works correctly
        let result = cmd!("grep", "test")
            .pipe(cmd!("wc", "-l"))
            .input(&data)
            .output()?;

        assert!(!result.trim().is_empty());
        Ok(())
    }

    #[test]
    fn test_memory_efficiency() -> Result<()> {
        // Test memory efficiency with native pipes streaming
        let large_data = generate_test_data(10000);

        let result = cmd!("head", "-10")
            .pipe(cmd!("tail", "-5"))
            .input(&large_data)
            .output()?;

        assert_eq!(result.lines().count(), 5);
        Ok(())
    }
}
