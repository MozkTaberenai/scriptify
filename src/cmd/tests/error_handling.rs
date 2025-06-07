//! Error handling tests.
//!
//! Tests for various error conditions and edge cases in command execution,
//! including non-existent commands, exit codes, and error message quality.

use super::Cmd;
use crate::cmd;
use std::ffi::OsString;

/// Tests comprehensive command not found error handling
#[test]
fn test_command_not_found_error() {
    // Test basic command not found
    let result = cmd!("nonexistent_command_12345").no_echo().run();
    assert!(result.is_err());

    // Test that error message is informative
    let error = result.unwrap_err();
    assert!(error.message.contains("Failed to spawn command"));
    assert!(error.message.contains("nonexistent_command_12345"));

    // Test with different non-existent command
    let result = cmd!("this_command_definitely_does_not_exist")
        .no_echo()
        .run();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.message.contains("Failed to spawn command"));
    assert!(
        error
            .message
            .contains("this_command_definitely_does_not_exist")
    );

    // Test with output() method
    let result = cmd!("missing_command").no_echo().output();
    assert!(result.is_err());
}

/// Tests command that exits with non-zero status
#[test]
fn test_exit_code_handling() {
    // Test various exit codes
    for exit_code in [1, 2, 127, 255] {
        let result = cmd!("sh", "-c", &format!("exit {}", exit_code))
            .no_echo()
            .run();
        assert!(
            result.is_err(),
            "Exit code {} should result in error",
            exit_code
        );
    }

    // Test with output() method
    let result = cmd!("sh", "-c", "exit 42").no_echo().output();
    assert!(result.is_err());

    // Test successful command (should not error)
    let result = cmd!("sh", "-c", "exit 0").no_echo().run();
    assert!(result.is_ok());
}

/// Tests empty command handling
#[test]
fn test_empty_command_handling() {
    let cmd = Cmd::new("");
    assert_eq!(cmd.program, OsString::from(""));
    let result = cmd.no_echo().run();
    assert!(result.is_err());

    // Test empty command with output
    let result = Cmd::new("").no_echo().output();
    assert!(result.is_err());
}

/// Tests permission denied scenarios
#[test]
fn test_permission_denied() {
    #[cfg(unix)]
    {
        // Try to access a restricted file
        let result = cmd!("cat", "/etc/shadow").no_echo().run();
        // This should fail unless running as root
        if std::env::var("USER").unwrap_or_default() != "root" {
            assert!(result.is_err());
        }

        // Try to write to a restricted directory
        let result = cmd!("touch", "/etc/test_file_should_fail").no_echo().run();
        if std::env::var("USER").unwrap_or_default() != "root" {
            assert!(result.is_err());
        }
    }

    #[cfg(windows)]
    {
        // Try to access system files on Windows
        let result = cmd!("type", "C:\\Windows\\System32\\config\\SAM")
            .no_echo()
            .run();
        // This should typically fail for regular users
        let _ = result; // We can't reliably test this on all Windows systems
    }
}

/// Tests pipeline error propagation
#[test]
fn test_pipeline_error_propagation() {
    // First command fails
    let result = cmd!("nonexistent_command")
        .pipe(cmd!("cat"))
        .no_echo()
        .run();
    assert!(result.is_err());

    // Second command fails
    let result = cmd!("echo", "test")
        .pipe(cmd!("nonexistent_command"))
        .no_echo()
        .run();
    assert!(result.is_err());

    // First command exits with error
    let result = cmd!("sh", "-c", "exit 1").pipe(cmd!("cat")).no_echo().run();
    assert!(result.is_err());
}

/// Tests resource exhaustion scenarios (where possible)
#[test]
fn test_resource_limits() {
    // Test with extremely long argument (might hit argument length limits)
    let very_long_arg = "a".repeat(1_000_000);
    let result = cmd!("echo", &very_long_arg).no_echo().output();
    // This might succeed or fail depending on system limits
    let _ = result; // Just ensure it doesn't panic

    // Test with many arguments
    let mut cmd = cmd!("echo");
    for i in 0..1000 {
        cmd = cmd.arg(format!("arg{}", i));
    }
    let result = cmd.no_echo().output();
    // This should typically succeed, but test for robustness
    let _ = result;
}

/// Tests signal handling and interruption
#[test]
fn test_signal_handling() {
    #[cfg(unix)]
    {
        // Test with a command that should be interruptible
        // Start a sleep command in the background
        let result = cmd!("sleep", "0.1").no_echo().run();
        // Should complete normally
        assert!(result.is_ok());

        // Test that commands can be killed (indirectly)
        let result = cmd!("sh", "-c", "sleep 0.1 && echo done")
            .no_echo()
            .output();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.trim(), "done");
    }
}

/// Tests malformed input handling
#[test]
fn test_malformed_input() {
    // Test with null bytes in input
    let input_with_null = "hello\0world";
    let result = cmd!("cat").input(input_with_null).no_echo().output();
    // Should handle gracefully
    assert!(result.is_ok());

    // Test with very large input
    let huge_input = "x".repeat(10_000_000); // 10MB
    let result = cmd!("wc", "-c").input(&huge_input).no_echo().output();
    // Should handle large input
    assert!(result.is_ok());

    // Test with binary data
    let binary_data = (0..256).map(|i| i as u8).collect::<Vec<_>>();
    let binary_string = String::from_utf8_lossy(&binary_data);
    let result = cmd!("cat").input(binary_string.as_ref()).no_echo().output();
    assert!(result.is_ok());
}

/// Tests error recovery in script-like scenarios
#[test]
fn test_error_recovery_workflow() {
    // Test that one failed command doesn't affect subsequent ones
    let _failed_result = cmd!("false").no_echo().run(); // This should fail

    let success_result = cmd!("echo", "success").no_echo().output().unwrap();
    assert_eq!(success_result.trim(), "success");

    // Test graceful handling of missing files
    let missing_file_result = cmd!("cat", "nonexistent_file.txt").no_echo().output();
    assert!(missing_file_result.is_err());

    // But other operations should still work
    let working_result = cmd!("echo", "still working").no_echo().output().unwrap();
    assert_eq!(working_result.trim(), "still working");
}
