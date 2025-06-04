//! # Pipe Modes - Advanced Pipeline Mode Control
//!
//! This example demonstrates advanced pipe mode control for stderr and combined output:
//! - Default stdout piping
//! - Stderr-only piping
//! - Combined stdout+stderr piping
//! - Explicit pipe mode configuration
//!
//! Estimated time: ~4 minutes
//! Prerequisites: Complete error_handling.rs and environment.rs

use scriptify::*;

fn main() -> Result<()> {
    println!("🔀 Advanced Pipe Mode Control");
    println!("============================\n");

    // Example 1: Default stdout piping
    println!("1. Default stdout piping:");
    let output = cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .output()?;
    println!("   Output: {}", output.trim());
    println!();

    // Example 2: Stderr piping
    println!("2. Stderr piping:");
    println!("   Command: Generate error message and count its characters");
    let error_char_count = cmd!("sh", "-c", "echo 'Error: Something went wrong!' >&2")
        .pipe_stderr(cmd!("wc", "-c"))
        .output()?;
    println!(
        "   Error message character count: {}",
        error_char_count.trim()
    );
    println!();

    // Example 3: Both stdout and stderr piping
    println!("3. Combined stdout+stderr piping:");
    println!("   Command: Generate both outputs and sort them together");
    let combined_output = cmd!("sh", "-c", "echo 'stdout line'; echo 'stderr line' >&2")
        .pipe_both(cmd!("sort"))
        .output()?;
    println!("   Combined and sorted output:");
    for line in combined_output.lines() {
        println!("     {}", line);
    }
    println!();

    // Example 4: Mixed pipe modes in a single pipeline
    println!("4. Mixed pipe modes in a single pipeline:");
    let mixed_output = cmd!("sh", "-c", "echo 'stdout'; echo 'stderr' >&2")
        .pipe_stderr(cmd!("sed", "s/stderr/processed_stderr/"))
        .pipe(cmd!("cat"))
        .output()?;
    println!("   Mixed pipeline output: {}", mixed_output.trim());
    println!();

    // Example 5: Error processing pipeline
    println!("5. Error processing pipeline:");
    println!("   Command: Generate multiple error lines and count them");
    let error_lines = cmd!(
        "sh",
        "-c",
        "echo 'ERROR 1' >&2; echo 'ERROR 2' >&2; echo 'ERROR 3' >&2"
    )
    .pipe_stderr(cmd!("wc", "-l"))
    .output()?;
    println!("   Number of error lines: {}", error_lines.trim());
    println!();

    // Example 6: Complex stderr processing
    println!("6. Complex stderr processing:");
    println!("   Command: Filter specific errors from stderr");
    let filtered_errors = cmd!(
        "sh",
        "-c",
        "echo 'INFO: starting' >&2; echo 'ERROR: failed' >&2; echo 'INFO: done' >&2"
    )
    .pipe_stderr(cmd!("grep", "ERROR"))
    .output()?;
    println!("   Filtered errors: {}", filtered_errors.trim());

    println!("\n🎉 Pipe modes tutorial completed!");
    println!("Next, try complex_pipes.rs for advanced pipeline techniques");

    Ok(())
}
