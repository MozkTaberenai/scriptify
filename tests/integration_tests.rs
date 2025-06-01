//! Integration tests for scriptify
//!
//! These tests verify that scriptify works correctly in real-world scenarios
//! by testing actual command execution and pipeline functionality.

use scriptify::*;
use std::fs;
use std::path::Path;

#[test]
fn test_basic_command_execution() {
    let result = cmd!("echo", "hello world").output().unwrap();
    assert_eq!(result.trim(), "hello world");
}

#[test]
fn test_command_with_multiple_args() {
    let result = cmd!("echo", "one", "two", "three").output().unwrap();
    assert_eq!(result.trim(), "one two three");
}

#[test]
fn test_command_piping() {
    let result = cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .output()
        .unwrap();
    assert_eq!(result.trim(), "HELLO WORLD");
}

#[test]
fn test_multiple_stage_pipeline() {
    let result = cmd!("echo", "apple\nbanana\napple")
        .pipe(cmd!("sort"))
        .pipe(cmd!("uniq"))
        .output()
        .unwrap();

    let lines: Vec<&str> = result.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"apple"));
    assert!(lines.contains(&"banana"));
}

#[test]
fn test_command_with_input() {
    let input_data = "line1\nline2\nline3\n";
    let result = cmd!("wc", "-l").input(input_data).output().unwrap();
    assert_eq!(result.trim(), "3");
}

#[test]
fn test_pipeline_with_input() {
    let input_data = "banana\napple\ncherry\nbanana\n";
    let result = cmd!("sort")
        .pipe(cmd!("uniq"))
        .input(input_data)
        .output()
        .unwrap();

    let lines: Vec<&str> = result.trim().split('\n').collect();
    assert_eq!(lines.len(), 3);
    assert!(lines.contains(&"apple"));
    assert!(lines.contains(&"banana"));
    assert!(lines.contains(&"cherry"));
}

#[test]
fn test_quiet_mode() {
    // This test mainly ensures quiet mode doesn't break functionality
    let result = cmd!("echo", "test").quiet().output().unwrap();
    assert_eq!(result.trim(), "test");
}

#[test]
fn test_environment_variables() {
    let result = cmd!("sh", "-c", "echo $TEST_VAR")
        .env("TEST_VAR", "test_value")
        .output()
        .unwrap();
    assert_eq!(result.trim(), "test_value");
}

#[test]
fn test_working_directory() {
    let current_dir = std::env::current_dir().unwrap();
    let parent = current_dir.parent().unwrap_or(&current_dir);

    let result = cmd!("pwd").cwd(parent).output().unwrap();

    assert_eq!(result.trim(), parent.to_string_lossy());
}

#[test]
fn test_error_handling() {
    let result = cmd!("nonexistent_command_12345").quiet().run();
    assert!(result.is_err());
}

#[test]
fn test_nonexistent_command() {
    let result = cmd!("this_command_does_not_exist_12345").quiet().run();
    assert!(result.is_err());
}

#[test]
fn test_pipe_stderr() {
    let result = cmd!("sh", "-c", "echo 'error message' >&2")
        .pipe_stderr(cmd!("wc", "-c"))
        .output()
        .unwrap();

    // Should count characters in stderr
    assert!(result.trim().parse::<i32>().unwrap() > 0);
}

#[test]
fn test_pipe_both() {
    let result = cmd!("sh", "-c", "echo 'stdout'; echo 'stderr' >&2")
        .pipe_both(cmd!("wc", "-l"))
        .output()
        .unwrap();

    // Should count lines from both stdout and stderr
    assert_eq!(result.trim(), "2");
}

#[test]
fn test_complex_text_processing() {
    let data = "apple\nbanana\ncherry\napple\ndate\nbanana\n";
    let result = cmd!("sort")
        .pipe(cmd!("uniq", "-c"))
        .pipe(cmd!("sort", "-nr"))
        .input(data)
        .output()
        .unwrap();

    let lines: Vec<&str> = result.trim().split('\n').collect();
    // Should have 4 unique items with counts
    assert_eq!(lines.len(), 4);

    // First line should have count 2 (apple or banana)
    let first_count: i32 = lines[0].split_whitespace().next().unwrap().parse().unwrap();
    assert_eq!(first_count, 2);
}

#[test]
fn test_file_system_integration() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let test_dir = "test_fs_integration";
    let test_file = format!("{}/test.txt", test_dir);

    // Clean up any existing test directory
    if Path::new(test_dir).exists() {
        fs::remove_dir_all(test_dir)?;
    }

    // Create directory and file using scriptify
    scriptify::fs::create_dir_all(test_dir)?;
    scriptify::fs::write(&test_file, "test content")?;

    // Verify using commands
    let ls_result = cmd!("ls", test_dir).output()?;
    assert!(ls_result.contains("test.txt"));

    let cat_result = cmd!("cat", &test_file).output()?;
    assert_eq!(cat_result.trim(), "test content");

    // Clean up
    scriptify::fs::remove_dir_all(test_dir)?;

    Ok(())
}

#[test]
fn test_large_input_processing() {
    // Create a large input to test streaming
    let large_input: String = (0..1000).map(|i| format!("line {}\n", i)).collect();

    let result = cmd!("wc", "-l").input(&large_input).output().unwrap();

    assert_eq!(result.trim(), "1000");
}

#[test]
fn test_pipeline_error_propagation() {
    // Test that errors in the middle of a pipeline are properly handled
    let result = cmd!("echo", "test")
        .pipe(cmd!("nonexistent_command_xyz")) // This should fail
        .pipe(cmd!("cat"))
        .quiet()
        .output();

    assert!(result.is_err());
}

#[test]
fn test_csv_like_processing() {
    let csv_data = "name,age,city\nAlice,25,NYC\nBob,30,LA\nCharlie,35,Chicago\n";

    // Extract just the names (first column)
    let names = cmd!("cut", "-d", ",", "-f", "1")
        .pipe(cmd!("tail", "-n", "+2")) // Skip header
        .input(csv_data)
        .output()
        .unwrap();

    let name_list: Vec<&str> = names.trim().split('\n').collect();
    assert_eq!(name_list, vec!["Alice", "Bob", "Charlie"]);
}

#[test]
fn test_json_like_processing() {
    let json_like = r#"{"name": "test", "value": 42}
{"name": "test2", "value": 24}
{"name": "test3", "value": 123}"#;

    // Extract values using grep
    let values = cmd!("grep", "-o", "\"value\": [0-9]*")
        .pipe(cmd!("cut", "-d", " ", "-f", "2"))
        .input(json_like)
        .output()
        .unwrap();

    let value_list: Vec<&str> = values.trim().split('\n').collect();
    assert_eq!(value_list, vec!["42", "24", "123"]);
}

#[test]
fn test_log_analysis_pattern() {
    let log_data = r#"2024-01-01 10:00:00 INFO Starting application
2024-01-01 10:00:01 ERROR Failed to connect to database
2024-01-01 10:00:02 INFO Retrying connection
2024-01-01 10:00:03 ERROR Connection timeout
2024-01-01 10:00:04 INFO Application started successfully"#;

    // Count ERROR entries
    let error_count = cmd!("grep", "-c", "ERROR")
        .input(log_data)
        .output()
        .unwrap();

    assert_eq!(error_count.trim(), "2");

    // Extract error messages
    let errors = cmd!("grep", "ERROR")
        .pipe(cmd!("cut", "-d", " ", "-f", "4-"))
        .input(log_data)
        .output()
        .unwrap();

    assert!(errors.contains("Failed to connect to database"));
    assert!(errors.contains("Connection timeout"));
}

#[test]
fn test_numeric_processing() {
    let numbers = "10\n20\n30\n40\n50\n";

    // Calculate sum using awk
    let sum = cmd!("awk", "{sum += $1} END {print sum}")
        .input(numbers)
        .output()
        .unwrap();

    assert_eq!(sum.trim(), "150");

    // Find min and max
    let sorted = cmd!("sort", "-n").input(numbers).output().unwrap();

    let lines: Vec<&str> = sorted.trim().split('\n').collect();
    assert_eq!(lines.first().unwrap(), &"10");
    assert_eq!(lines.last().unwrap(), &"50");
}

#[test]
fn test_command_chaining_with_different_inputs() {
    // Test multiple separate pipelines with different inputs
    let result1 = cmd!("echo", "first")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .output()
        .unwrap();

    let result2 = cmd!("echo", "second")
        .pipe(cmd!("wc", "-c"))
        .output()
        .unwrap();

    assert_eq!(result1.trim(), "FIRST");
    assert_eq!(result2.trim(), "7"); // "second\n" = 7 chars
}

#[test]
fn test_error_recovery_in_scripts() {
    // Test that one failed command doesn't affect subsequent ones
    let _failed_result = cmd!("false").run(); // This should fail

    let success_result = cmd!("echo", "success").output().unwrap();
    assert_eq!(success_result.trim(), "success");
}

#[cfg(unix)]
#[test]
fn test_unix_specific_commands() {
    // Test some Unix-specific functionality
    let result = cmd!("uname").output().unwrap();
    assert!(!result.trim().is_empty());

    let whoami = cmd!("whoami").output().unwrap();
    assert!(!whoami.trim().is_empty());
}
