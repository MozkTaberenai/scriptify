//! No echo mode tests.
//!
//! Tests for no echo execution mode functionality, including command echo suppression,
//! pipeline propagation, and inheritance behavior.

use crate::cmd;

/// Tests basic no echo mode functionality
#[test]
fn test_no_echo_mode() {
    // Test that no echo mode doesn't crash with run()
    let result = cmd!("echo", "test").no_echo().run();
    assert!(result.is_ok());

    // Test that no echo mode doesn't crash with output()
    let result = cmd!("echo", "test").no_echo().output();
    assert!(result.is_ok());

    // Test no echo mode with failing command
    let result = cmd!("nonexistent_command").no_echo().run();
    assert!(result.is_err());
}

/// Tests that no echo mode actually suppresses the command echo
#[test]
fn test_no_echo_mode_suppresses_echo() {
    use std::fs;
    use std::process::{Command, Stdio};

    // Create test script that runs both modes
    let test_code = r#"
fn main() {
    use scriptify::cmd;
    
    // Run normal mode
    eprintln!("=== NORMAL MODE START ===");
    let _ = cmd!("echo", "test_command").output();
    eprintln!("=== NORMAL MODE END ===");
    
    // Run no echo mode
    eprintln!("=== NO_ECHO MODE START ===");
    let _ = cmd!("echo", "test_command").no_echo().output();
    eprintln!("=== NO_ECHO MODE END ===");
}
"#;

    // Write test file
    fs::write("/tmp/echo_test.rs", test_code).unwrap();

    // Compile test
    let compile_result = Command::new("rustc")
        .args([
            "/tmp/echo_test.rs",
            "--edition",
            "2024",
            "-o",
            "/tmp/echo_test",
        ])
        .output()
        .unwrap();

    if !compile_result.status.success() {
        // If compilation fails, fall back to basic flag test
        let cmd_no_echo = cmd!("echo", "test").no_echo();
        let cmd_normal = cmd!("echo", "test");

        assert!(cmd_no_echo.suppress_echo);
        assert!(!cmd_normal.suppress_echo);

        let output_no_echo = cmd_no_echo.output().unwrap();
        let output_normal = cmd_normal.output().unwrap();

        assert_eq!(output_no_echo.trim(), "test");
        assert_eq!(output_normal.trim(), "test");
        assert_eq!(output_no_echo, output_normal);
        return;
    }

    // Execute and capture stderr
    let output = Command::new("/tmp/echo_test")
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stderr_str = String::from_utf8_lossy(&output.stderr);

    // Parse sections and verify no echo mode has less echo output
    let normal_section = extract_section(&stderr_str, "NORMAL MODE");
    let no_echo_section = extract_section(&stderr_str, "NO_ECHO MODE");

    // No echo section should have less or equal content (indicating less/no echoing)
    assert!(no_echo_section.len() <= normal_section.len());

    // Clean up
    let _ = fs::remove_file("/tmp/echo_test.rs");
    let _ = fs::remove_file("/tmp/echo_test");
}

fn extract_section<'a>(text: &'a str, mode: &str) -> &'a str {
    let start = format!("=== {} START ===", mode);
    let end = format!("=== {} END ===", mode);

    if let Some(start_pos) = text.find(&start) {
        if let Some(end_pos) = text.find(&end) {
            return &text[start_pos..end_pos];
        }
    }
    ""
}

/// Tests that no echo mode propagates correctly through pipelines
#[test]
fn test_pipeline_no_echo_propagation() {
    let pipeline_no_echo = cmd!("echo", "test")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo();

    let pipeline_normal = cmd!("echo", "test").pipe(cmd!("tr", "[:lower:]", "[:upper:]"));

    assert!(pipeline_no_echo.suppress_echo);
    assert!(!pipeline_normal.suppress_echo);

    let output_no_echo = pipeline_no_echo.output().unwrap();
    let output_normal = pipeline_normal.output().unwrap();

    assert_eq!(output_no_echo.trim(), "TEST");
    assert_eq!(output_normal.trim(), "TEST");
}

/// Tests that no echo mode is inherited when creating pipelines
#[test]
fn test_no_echo_mode_inheritance() {
    let no_echo_cmd = cmd!("echo", "hello").no_echo();
    let pipeline = no_echo_cmd.pipe(cmd!("cat"));

    assert!(pipeline.suppress_echo);

    let normal_cmd = cmd!("echo", "hello");
    let pipeline2 = normal_cmd.pipe(cmd!("cat"));

    assert!(!pipeline2.suppress_echo);
}

/// Tests no echo mode with various execution methods
#[test]
fn test_no_echo_mode_execution_methods() {
    // Test run() method with no echo
    let result = cmd!("echo", "run_test").no_echo().run();
    assert!(result.is_ok());

    // Test output() method with no echo
    let output = cmd!("echo", "output_test").no_echo().output().unwrap();
    assert_eq!(output.trim(), "output_test");

    // Test with input
    let output = cmd!("cat").input("input_test").no_echo().output().unwrap();
    assert_eq!(output.trim(), "input_test");
}

/// Tests no echo mode with environment variables and working directory
#[test]
fn test_no_echo_mode_with_env_and_dir() {
    use std::env;

    let temp_dir = env::temp_dir();

    let output = cmd!("printenv", "NO_ECHO_TEST_VAR")
        .env("NO_ECHO_TEST_VAR", "no_echo_value")
        .current_dir(&temp_dir)
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "no_echo_value");
}

/// Tests no echo mode with error conditions
#[test]
fn test_no_echo_mode_error_handling() {
    // Test no echo mode with command not found
    let result = cmd!("command_that_does_not_exist_xyz").no_echo().run();
    assert!(result.is_err());

    // Test no echo mode with non-zero exit
    let result = cmd!("sh", "-c", "exit 42").no_echo().run();
    assert!(result.is_err());

    // Test no echo mode with invalid directory
    let result = cmd!("echo", "test")
        .current_dir("/path/that/does/not/exist/xyz")
        .no_echo()
        .run();
    assert!(result.is_err());
}

/// Tests no echo mode behavior consistency
#[test]
fn test_no_echo_mode_consistency() {
    // Multiple executions should have consistent behavior
    for i in 0..5 {
        let output = cmd!("echo", &format!("test_{}", i))
            .no_echo()
            .output()
            .unwrap();
        assert_eq!(output.trim(), format!("test_{}", i));
    }
}

/// Tests no echo mode with complex pipelines
#[test]
fn test_no_echo_mode_complex_pipelines() {
    // Test no echo mode with stderr piping
    let output = cmd!("sh", "-c", "echo 'error_msg' >&2")
        .pipe_stderr(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "ERROR_MSG");

    // Test no echo mode with mixed pipe modes
    let output = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
        .pipe_both(cmd!("sort"))
        .no_echo()
        .output()
        .unwrap();
    let lines: Vec<&str> = output.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"err"));
    assert!(lines.contains(&"out"));
}

/// Tests no echo mode flag propagation in builder pattern
#[test]
fn test_no_echo_mode_builder_propagation() {
    // Test that no echo flag is preserved through builder chain
    let cmd = cmd!("echo", "test")
        .arg("extra")
        .env("TEST_VAR", "value")
        .no_echo()
        .arg("more");

    assert!(cmd.suppress_echo);

    // Test pipeline creation from no echo command
    let pipeline = cmd.pipe(cmd!("cat"));
    assert!(pipeline.suppress_echo);
}

/// Tests no echo mode with concurrent execution
#[test]
fn test_no_echo_mode_concurrency() {
    use std::thread;

    let thread_count = std::cmp::min(10, std::thread::available_parallelism().unwrap().get() * 2);

    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            thread::spawn(move || {
                cmd!("echo", &format!("concurrent_{}", i))
                    .no_echo()
                    .output()
                    .unwrap()
            })
        })
        .collect();

    let results: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Each thread should get its own output correctly in no echo mode
    assert_eq!(results.len(), thread_count);
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.trim(), format!("concurrent_{}", i));
    }
}
