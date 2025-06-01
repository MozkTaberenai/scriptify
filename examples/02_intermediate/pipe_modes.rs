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
    echo!("🔀 Advanced Pipe Mode Control");
    echo!("============================\n");

    // Example 1: Default stdout piping
    echo!("1. Default stdout piping:");
    let output = cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .output()?;
    echo!("   Output:", output.trim());
    echo!();

    // Example 2: Stderr piping
    echo!("2. Stderr piping:");
    echo!("   Command: Generate error message and count its characters");
    let error_char_count = cmd!("sh", "-c", "echo 'Error: Something went wrong!' >&2")
        .pipe_stderr(cmd!("wc", "-c"))
        .output()?;
    echo!("   Error message character count:", error_char_count.trim());
    echo!();

    // Example 3: Both stdout and stderr piping
    echo!("3. Combined stdout+stderr piping:");
    echo!("   Command: Generate both outputs and sort them together");
    let combined_output = cmd!("sh", "-c", "echo 'stdout line'; echo 'stderr line' >&2")
        .pipe_both(cmd!("sort"))
        .output()?;
    echo!("   Combined and sorted output:");
    for line in combined_output.lines() {
        println!("     {}", line);
    }
    echo!();

    // Example 4: Mixed pipe modes in a single pipeline
    echo!("4. Mixed pipe modes in a single pipeline:");
    let mixed_output = cmd!("sh", "-c", "echo 'stdout'; echo 'stderr' >&2")
        .pipe_stderr(cmd!("sed", "s/stderr/processed_stderr/"))
        .pipe(cmd!("cat"))
        .output()?;
    echo!("   Mixed pipeline output:", mixed_output.trim());
    echo!();

    // Example 5: Error processing pipeline
    echo!("5. Error processing pipeline:");
    echo!("   Command: Generate multiple error lines and count them");
    let error_lines = cmd!(
        "sh",
        "-c",
        "echo 'ERROR 1' >&2; echo 'ERROR 2' >&2; echo 'ERROR 3' >&2"
    )
    .pipe_stderr(cmd!("wc", "-l"))
    .output()?;
    echo!("   Number of error lines:", error_lines.trim());
    echo!();

    // Example 6: Complex stderr processing
    echo!("6. Complex stderr processing:");
    echo!("   Command: Filter specific errors from stderr");
    let filtered_errors = cmd!(
        "sh",
        "-c",
        "echo 'INFO: starting' >&2; echo 'ERROR: failed' >&2; echo 'INFO: done' >&2"
    )
    .pipe_stderr(cmd!("grep", "ERROR"))
    .output()?;
    echo!("   Filtered errors:", filtered_errors.trim());

    echo!("\n🎉 Pipe modes tutorial completed!");
    echo!("Next, try complex_pipes.rs for advanced pipeline techniques");

    Ok(())
}
