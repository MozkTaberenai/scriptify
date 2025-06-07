//! Example demonstrating command argument quoting for readability.
//!
//! This example shows how scriptify automatically quotes command arguments
//! that affect readability when echoing commands (spaces, control chars, etc).

use scriptify::*;

fn main() {
    println!("=== Command Argument Quoting Examples ===\n");

    // Simple arguments (no quoting needed)
    println!("1. Simple arguments:");
    cmd!("echo", "hello", "world").run().unwrap();
    println!();

    // Arguments with spaces
    println!("2. Arguments with spaces:");
    cmd!("echo", "hello world", "from rust").run().unwrap();
    println!();

    // Arguments with various characters (no quoting needed for readability)
    println!("3. Arguments with various characters:");
    cmd!("echo", "file*.txt", "$HOME/test", "command|grep")
        .run()
        .unwrap();
    println!();

    // Arguments with quotes
    println!("4. Arguments with quotes:");
    cmd!("echo", "say \"hello\"", "it's working").run().unwrap();
    println!();

    // Arguments with mixed quotes
    println!("5. Arguments with mixed quotes:");
    cmd!("echo", "it's a \"test\"", "can't use $HOME")
        .run()
        .unwrap();
    println!();

    // Arguments with control characters
    println!("6. Arguments with control characters:");
    cmd!("echo", "line1\nline2", "tab\there").run().unwrap();
    println!();

    // Empty arguments
    println!("7. Empty arguments:");
    cmd!("echo", "", "not empty", "").run().unwrap();
    println!();

    // Environment variables with spaces
    println!("8. Environment variables with spaces:");
    cmd!("echo", "testing")
        .env("SPECIAL_VAR", "value with spaces")
        .env("COMPLEX_VAR", "$HOME and `pwd`")
        .run()
        .unwrap();
    println!();

    // Working directory with spaces
    println!("9. Working directory with spaces (simulated):");
    // Note: This would fail if the directory doesn't exist, so we just show the echo
    let _cmd = cmd!("echo", "in special dir")
        .current_dir("/tmp/test directory")
        .no_echo();
    // Just demonstrate the echo formatting without actually running
    println!("Command would be echoed as shown above if directory existed");
    println!();

    // Complex pipeline with special characters
    println!("10. Pipeline with special arguments:");
    cmd!("echo", "data with spaces")
        .pipe(cmd!("grep", "with"))
        .pipe(cmd!("sed", "s/with/containing/g"))
        .run()
        .unwrap();
    println!();

    // Demonstrate that actual command execution works correctly
    println!("11. Verifying that execution works correctly:");
    let output = cmd!("echo", "hello world", "from", "scriptify")
        .no_echo()
        .output()
        .unwrap();
    println!("Captured output: '{}'", output.trim());
    println!();

    println!("=== All examples completed successfully! ===");
    println!();
    println!("Notice how arguments that affect readability (spaces, control chars)");
    println!("are automatically quoted in the command echo output for clarity.");
}
