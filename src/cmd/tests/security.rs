//! Security tests.
//!
//! Tests for command injection prevention and safe handling of special shell characters,
//! ensuring that user input cannot be used to execute unintended commands.

use crate::cmd;

/// Tests that special shell characters are handled safely
#[test]
fn test_command_substitution_safety() {
    let output = cmd!("echo", "$(echo test)").no_echo().output().unwrap();
    // Should output literally, not execute the command substitution
    assert_eq!(output.trim(), "$(echo test)");

    let output = cmd!("echo", "; echo injected").no_echo().output().unwrap();
    assert_eq!(output.trim(), "; echo injected");
}

/// Tests various command injection patterns
#[test]
fn test_command_injection_patterns() {
    // Test multiple command separators
    let patterns = vec!["; ls", "| ls", "|| ls", "&& ls", "& ls", "\n ls", "\r\n ls"];

    for pattern in patterns {
        let output = cmd!("echo", pattern).no_echo().output().unwrap();
        assert_eq!(output.trim(), pattern.trim());
    }
}

/// Tests path traversal attempts (Unix-specific)
#[test]
fn test_path_traversal_safety() {
    // These should be treated as literal arguments, not interpreted
    let paths = vec![
        "../../../etc/passwd",
        "/etc/passwd",
        "~/.ssh/id_rsa",
        "${HOME}/.bashrc",
    ];

    for path in paths {
        let output = cmd!("echo", path).no_echo().output().unwrap();
        assert_eq!(output.trim(), path);
    }
}

/// Tests environment variable injection (Unix-specific)
#[test]
fn test_env_var_injection() {
    // Environment variables should not be expanded in arguments
    let vars = vec![
        "$PATH",
        "${PATH}",
        "$HOME",
        "${HOME:-/tmp}",
        "$(printenv HOME)",
        "`printenv HOME`",
    ];

    for var in vars {
        let output = cmd!("echo", var).no_echo().output().unwrap();
        assert_eq!(output.trim(), var);
    }
}

/// Tests null byte and special character injection
#[test]
fn test_special_char_injection() {
    // Test null byte injection - should fail at system level
    let result = cmd!("echo", "test\0command").no_echo().output();
    assert!(result.is_err(), "Null byte in arguments should fail");

    // Test various control characters (excluding null byte)
    let chars = vec![
        "test\x07bell",
        "test\x08backspace",
        "test\x1bescape",
        "test\x7fdelete",
    ];

    for ch in chars {
        let output = cmd!("echo", ch).no_echo().output().unwrap();
        // Should not execute anything, just pass through
        assert!(!output.is_empty());
    }
}

/// Tests complex nested injection attempts
#[test]
fn test_nested_injection_attempts() {
    // Test nested command substitution
    let output = cmd!("echo", "$(echo $(whoami))")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "$(echo $(whoami))");

    // Test mixed quotes and substitution
    let output = cmd!("echo", "\"$(echo 'test')\"")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "\"$(echo 'test')\"");

    // Test backticks within other constructs
    let output = cmd!("echo", "'`id`'").no_echo().output().unwrap();
    assert_eq!(output.trim(), "'`id`'");
}

/// Tests Unicode and encoding-based injection attempts
#[test]
fn test_unicode_injection() {
    // Test Unicode variations of shell metacharacters
    let unicode_tests = vec![
        "test\u{2028}newline",     // Unicode line separator
        "test\u{2029}paragraph",   // Unicode paragraph separator
        "test\u{00A0}space",       // Non-breaking space
        "test\u{3000}ideographic", // Ideographic space
    ];

    for test in unicode_tests {
        let output = cmd!("echo", test).no_echo().output().unwrap();
        // Should treat as literal text
        assert!(output.contains("test"));
    }
}

/// Tests command argument boundary attacks
#[test]
fn test_argument_boundary_attacks() {
    // Test attempts to break out of argument boundaries
    let attacks = vec![
        "' ; ls ; echo '",
        "\" ; ls ; echo \"",
        "') ; ls ; echo ('",
        "\") ; ls ; echo (\"",
        "'; ls #",
        "\"; ls #",
    ];

    for attack in attacks {
        let output = cmd!("echo", attack).no_echo().output().unwrap();
        assert_eq!(output.trim(), attack);
    }
}
