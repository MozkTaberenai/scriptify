//! # Mixed Pipe Modes - Simple Pipeline Examples
//!
//! This example demonstrates mixed pipe mode scenarios with simple, reliable commands:
//! - Basic stderr to stdout piping
//! - Stdout to stderr to both combinations
//! - Multi-stage mixed pipelines
//!
//! Estimated time: ~3 minutes
//! Prerequisites: Complete pipe_modes.rs

use scriptify::*;

fn main() -> Result<()> {
    echo!("🔀 Mixed Pipe Modes - Simple Examples");
    echo!("====================================\n");

    // Example 1: Basic stderr → stdout pipeline
    echo!("1. Basic Error Processing:");
    echo!("   Stderr from first command → Count characters → Process result");

    let char_count = cmd!("sh", "-c", "echo 'Error message' >&2")
        .pipe_stderr(cmd!("wc", "-c")) // stderr → stdout (count chars)
        .pipe(cmd!("tr", "-d", " ")) // stdout → stdout (remove spaces)
        .output()?;

    echo!("   Character count:", char_count.trim());
    echo!();

    // Example 2: Mixed stdout/stderr processing
    echo!("2. Mixed Output Processing:");
    echo!("   Generate both outputs → Process separately → Combine");

    let mixed_result = cmd!("sh", "-c", "echo 'normal output'; echo 'error output' >&2")
        .pipe_stderr(cmd!("sed", "s/^/ERR: /")) // stderr → stdout (prefix errors)
        .pipe(cmd!("sed", "s/^/OK: /")) // stdout → stdout (prefix normal)
        .pipe_both(cmd!("sort")) // both → stdout (sort all)
        .output()?;

    echo!("   Mixed processing result:");
    for line in mixed_result.lines() {
        echo!("     {}", line);
    }
    echo!();

    // Example 3: Alternating pipe modes
    echo!("3. Alternating Pipe Modes:");
    echo!("   stdout → stderr → stdout → both sequence");

    let alternating_result = cmd!("echo", "start")
        .pipe(cmd!(
            "sh",
            "-c",
            "read input; echo \"$input processed\"; echo \"warning\" >&2"
        ))
        .pipe_stderr(cmd!("wc", "-c")) // stderr → stdout (count warning chars)
        .pipe(cmd!(
            "sh",
            "-c",
            "read count; echo \"chars: $count\"; echo \"info\" >&2"
        ))
        .pipe_both(cmd!("wc", "-l")) // both → stdout (count all lines)
        .output()?;

    echo!(
        "   Line count from alternating pipeline:",
        alternating_result.trim()
    );
    echo!();

    // Example 4: Data filtering example
    echo!("4. Simple Data Filtering:");
    echo!("   Generate data → Filter valid/invalid → Mark and combine");

    let data = "item1\nbad@item\nitem2\nitem3";
    let filtered_result = cmd!("grep", "-E", "^[a-z0-9]+$")
        .pipe(cmd!(
            "sh",
            "-c",
            "while read line; do echo \"valid: $line\"; done"
        ))
        .input(data)
        .output()?;

    echo!("   Filtered valid items:");
    for line in filtered_result.lines() {
        echo!("     {}", line);
    }
    echo!();

    // Example 5: Error counting pipeline
    echo!("5. Error Counting Pipeline:");
    echo!("   Generate mixed output → Count errors via stderr → Format result");

    let error_count = cmd!(
        "sh",
        "-c",
        "echo 'line1'; echo 'err1' >&2; echo 'line2'; echo 'err2' >&2"
    )
    .pipe_stderr(cmd!("wc", "-l")) // stderr → stdout (count errors)
    .pipe(cmd!("sh", "-c", "read count; echo \"Found $count errors\""))
    .output()?;

    echo!("   Error counting result:", error_count.trim());

    echo!("\n🎉 Mixed pipe modes examples completed!");
    echo!("Key concepts demonstrated:");
    echo!("  • pipe_stderr() - Routes stderr to next command's stdin");
    echo!("  • pipe_both() - Routes both stdout+stderr to next command's stdin");
    echo!("  • pipe() - Routes stdout to next command's stdin (default)");
    echo!("  • Mixed sequences allow complex data processing flows");

    Ok(())
}
