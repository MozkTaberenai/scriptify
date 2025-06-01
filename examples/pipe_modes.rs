//! Example demonstrating different pipe modes for stderr and combined output
//!
//! This example shows how to use the different pipe modes:
//! - Default stdout piping
//! - Stderr-only piping
//! - Combined stdout+stderr piping

use scriptify::{PipeMode, cmd};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Pipe Mode Examples ===\n");

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
        .pipe(cmd!("wc", "-c"))
        .pipe_stderr()
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
        .pipe(cmd!("sort"))
        .pipe_both()
        .output()?;
    println!("   Combined and sorted output:");
    for line in combined_output.lines() {
        println!("     {}", line);
    }
    println!();

    // Example 4: Using PipeMode explicitly
    println!("4. Explicit pipe mode setting:");
    let explicit_output = cmd!("echo", "test data")
        .pipe(cmd!("cat"))
        .pipe_mode(PipeMode::Stdout)
        .output()?;
    println!("   Explicit stdout mode: {}", explicit_output.trim());
    println!();

    // Example 5: Error processing pipeline
    println!("5. Error processing pipeline:");
    println!("   Command: Generate multiple error lines and count them");
    let error_lines = cmd!(
        "sh",
        "-c",
        "echo 'ERROR 1' >&2; echo 'ERROR 2' >&2; echo 'ERROR 3' >&2"
    )
    .pipe(cmd!("wc", "-l"))
    .pipe_stderr()
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
    .pipe(cmd!("grep", "ERROR"))
    .pipe_stderr()
    .output()?;
    println!("   Filtered errors: {}", filtered_errors.trim());

    Ok(())
}
