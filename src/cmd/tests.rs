use super::*;
use std::ffi::OsString;
use std::path::PathBuf;

#[test]
fn test_cmd_new() {
    let cmd = Cmd::new("echo");
    assert_eq!(cmd.program, OsString::from("echo"));
    assert!(cmd.args.is_empty());
    assert!(!cmd.quiet);
}

#[test]
fn test_cmd_with_args() {
    let cmd = cmd!("echo", "hello", "world");
    assert_eq!(cmd.program, OsString::from("echo"));
    assert_eq!(cmd.args, vec![OsString::from("hello"), OsString::from("world")]);
}

#[test]
fn test_cmd_builder() {
    let cmd = Cmd::new("ls")
        .arg("-la")
        .env("TEST", "value")
        .cwd("/tmp")
        .quiet();

    assert_eq!(cmd.program, OsString::from("ls"));
    assert_eq!(cmd.args, vec![OsString::from("-la")]);
    assert_eq!(cmd.envs, vec![(OsString::from("TEST"), OsString::from("value"))]);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/tmp")));
    assert!(cmd.quiet);
}

#[test]
fn test_cmd_output() {
    let output = cmd!("echo", "test").quiet().output().unwrap();
    assert_eq!(output.trim(), "test");
}

#[test]
fn test_cmd_with_input() {
    let output = cmd!("cat").input("hello world").quiet().output().unwrap();
    assert_eq!(output.trim(), "hello world");
}

#[test]
fn test_pipeline() {
    let output = cmd!("echo", "hello")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .quiet()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "HELLO");
}

#[test]
fn test_pipeline_with_input() {
    let output = cmd!("tr", "[:lower:]", "[:upper:]")
        .input("hello world")
        .quiet()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "HELLO WORLD");
}

#[test]
fn test_environment_variable() {
    // Test that environment variables are properly set for the process
    let output = cmd!("printenv", "TEST_VAR")
        .env("TEST_VAR", "test_value")
        .quiet()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "test_value");
}

#[test]
fn test_error_handling() {
    let result = cmd!("nonexistent_command_12345").quiet().run();
    assert!(result.is_err());
}

#[test]
fn test_quiet_mode() {
    // This test mainly checks that quiet mode doesn't crash
    let result = cmd!("echo", "test").quiet().run();
    assert!(result.is_ok());
}

#[test]
fn test_multiple_pipes() {
    let output = cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipe(cmd!("rev"))
        .quiet()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "DLROW OLLEH");
}

#[test]
fn test_pipe_stderr() {
    // Test piping stderr to next command
    // First command generates stderr, second command should receive it
    let output = cmd!("sh", "-c", "echo 'error message' >&2")
        .pipe_stderr(cmd!("wc", "-c"))
        .quiet()
        .output()
        .unwrap();

    // Should count characters in the stderr message (13 chars + newline = 14)
    assert_eq!(output.trim(), "14");
}

#[test]
fn test_pipe_both() {
    // Test piping both stdout and stderr
    let output = cmd!("sh", "-c", "echo 'stdout' && echo 'stderr' >&2")
        .pipe_both(cmd!("sort"))
        .quiet()
        .output()
        .unwrap();

    // Should contain both outputs (order may vary due to threading)
    let output_str = output.trim();
    assert!(output_str.contains("stdout"));
    assert!(output_str.contains("stderr"));
}

#[test]
fn test_default_pipe_mode() {
    // Test that default pipe() creates stdout pipe mode
    let pipeline = cmd!("echo", "test").pipe(cmd!("cat"));
    assert_eq!(pipeline.connections[1].1, PipeMode::Stdout);
}

#[test]
fn test_pipe_stderr_mode() {
    // Test that pipe_stderr() creates stderr pipe mode
    let pipeline = cmd!("echo", "test").pipe_stderr(cmd!("cat"));
    assert_eq!(pipeline.connections[1].1, PipeMode::Stderr);
}

#[test]
fn test_pipe_both_mode() {
    // Test that pipe_both() creates both pipe mode
    let pipeline = cmd!("echo", "test").pipe_both(cmd!("cat"));
    assert_eq!(pipeline.connections[1].1, PipeMode::Both);
}

#[test]
fn test_direct_pipe_methods() {
    // Test all direct pipe methods for proper execution

    // Test stdout piping (default)
    let stdout_result = cmd!("echo", "native test")
        .pipe(cmd!("cat"))
        .quiet()
        .output()
        .unwrap();
    assert_eq!(stdout_result.trim(), "native test");

    // Test stderr piping
    let stderr_result = cmd!("sh", "-c", "echo 'native error' >&2")
        .pipe_stderr(cmd!("wc", "-c"))
        .quiet()
        .output()
        .unwrap();
    assert_eq!(stderr_result.trim(), "13");

    // Test both piping
    let both_result = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
        .pipe_both(cmd!("wc", "-l"))
        .quiet()
        .output()
        .unwrap();
    assert_eq!(both_result.trim(), "2");
}

#[test]
fn test_complex_mixed_pipeline() {
    // Test a complex pipeline with different pipe modes
    let output = cmd!("sh", "-c", "echo 'normal output'; echo 'error output' >&2")
        .pipe_stderr(cmd!("sed", "s/error/processed_error/"))
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipe_both(cmd!("sort"))
        .quiet()
        .output()
        .unwrap();

    // Should contain processed output
    assert!(!output.trim().is_empty());
}

#[test]
fn test_mixed_pipe_modes() {
    // Test that different pipe modes can be used in the same pipeline
    // This tests the core functionality that was requested

    // Create a pipeline that uses different pipe modes between different commands
    let output = cmd!("sh", "-c", "echo 'stdout line'; echo 'stderr line' >&2")
        .pipe_stderr(cmd!(
            "sh",
            "-c",
            "read line; echo \"processed: $line\"; echo \"more stderr\" >&2"
        ))
        .pipe(cmd!("wc", "-c"))
        .quiet()
        .output()
        .unwrap();

    // This should process the stderr from the first command through the pipeline
    assert!(!output.trim().is_empty());
}

#[test]
fn test_mixed_stderr_to_stdout_pipeline() {
    // Test stderr → stdout → combined pipeline
    let output = cmd!("sh", "-c", "echo 'error message' >&2")
        .pipe_stderr(cmd!("wc", "-c")) // stderr → stdout (character count)
        .pipe(cmd!("cat")) // stdout → stdout (pass through)
        .quiet()
        .output()
        .unwrap();

    // Should count characters in stderr message
    assert!(output.trim().parse::<i32>().unwrap() > 0);
}

#[test]
fn test_stdout_stderr_both_sequence() {
    // Test stdout → stderr → both sequence
    let output = cmd!("echo", "test data")
        .pipe(cmd!(
            "sh",
            "-c",
            "read input; echo \"$input\"; echo \"error: $input\" >&2"
        ))
        .pipe_stderr(cmd!("sed", "s/^/ERR: /"))
        .pipe_both(cmd!("wc", "-l"))
        .quiet()
        .output()
        .unwrap();

    // Should count lines from both streams
    assert_eq!(output.trim(), "1");
}

#[test]
fn test_alternating_pipe_modes() {
    // Test alternating between different pipe modes
    let output = cmd!("sh", "-c", "echo 'line1'; echo 'err1' >&2")
        .pipe_stderr(cmd!(
            "sh",
            "-c",
            "read err; echo \"processed: $err\"; echo \"more_err\" >&2"
        ))
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipe_both(cmd!("sort"))
        .quiet()
        .output()
        .unwrap();

    // Should contain processed output
    assert!(!output.trim().is_empty());
}

#[test]
fn test_long_mixed_pipeline() {
    // Test a longer pipeline with multiple mixed pipe modes
    let output = cmd!("echo", "start")
        .pipe(cmd!(
            "sh",
            "-c",
            "read input; echo \"$input-processed\"; echo \"warning\" >&2"
        ))
        .pipe_stderr(cmd!("wc", "-c"))
        .pipe(cmd!("sh", "-c", "read count; echo \"chars: $count\""))
        .pipe_both(cmd!("wc", "-w"))
        .quiet()
        .output()
        .unwrap();

    // Should count words in the final output
    assert!(output.trim().parse::<i32>().unwrap() >= 1);
}

#[test]
fn test_cmd_parse() {
    // Test basic parsing
    let cmd = Cmd::parse("echo hello world");
    assert_eq!(cmd.program, OsString::from("echo"));
    assert_eq!(cmd.args, vec![OsString::from("hello"), OsString::from("world")]);

    // Test parsing with multiple spaces
    let cmd = Cmd::parse("ls  -la   /tmp");
    assert_eq!(cmd.program, OsString::from("ls"));
    assert_eq!(cmd.args, vec![OsString::from("-la"), OsString::from("/tmp")]);

    // Test empty string
    let cmd = Cmd::parse("");
    assert_eq!(cmd.program, OsString::from(""));
    assert!(cmd.args.is_empty());

    // Test single word
    let cmd = Cmd::parse("pwd");
    assert_eq!(cmd.program, OsString::from("pwd"));
    assert!(cmd.args.is_empty());

    // Test with leading/trailing whitespace
    let cmd = Cmd::parse("  echo test  ");
    assert_eq!(cmd.program, OsString::from("echo"));
    assert_eq!(cmd.args, vec![OsString::from("test")]);
}

#[test]
fn test_empty_command_handling() {
    // Test empty program name
    let cmd = Cmd::new("");
    assert_eq!(cmd.program, OsString::from(""));
    let result = cmd.quiet().run();
    assert!(result.is_err());
}

#[test]
fn test_empty_pipeline() {
    // Test pipeline with no connections
    let pipeline = Pipeline {
        connections: vec![],
        input: None,
        quiet: true,
    };
    let result = pipeline.output().unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_large_input() {
    // Test with large input data
    let large_input = "x".repeat(10000);
    let output = cmd!("wc", "-c")
        .input(&large_input)
        .quiet()
        .output()
        .unwrap();

    // Should count the characters correctly (plus newline)
    let count: usize = output.trim().parse().unwrap();
    assert_eq!(count, 10000);
}

#[test]
fn test_invalid_working_directory() {
    // Test with non-existent working directory
    let result = cmd!("pwd").cwd("/nonexistent/directory/path").quiet().run();
    assert!(result.is_err());
}

#[test]
fn test_multiple_environment_variables() {
    // Test multiple environment variables
    let output = cmd!("sh", "-c", "echo $VAR1 $VAR2 $VAR3")
        .env("VAR1", "value1")
        .env("VAR2", "value2")
        .env("VAR3", "value3")
        .quiet()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "value1 value2 value3");
}

#[test]
fn test_args_method() {
    // Test adding multiple arguments at once
    let cmd = Cmd::new("ls").args(vec!["-l", "-a", "-h"]);
    assert_eq!(cmd.args, vec![OsString::from("-l"), OsString::from("-a"), OsString::from("-h")]);

    // Test with empty iterator
    let cmd = Cmd::new("echo").args(Vec::<&str>::new());
    assert!(cmd.args.is_empty());
}

#[test]
fn test_command_not_found_error() {
    // Test specific error message for command not found
    let result = cmd!("this_command_definitely_does_not_exist_12345")
        .quiet()
        .run();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.message.contains("Failed to spawn command"));
}

#[test]
fn test_permission_denied_scenario() {
    // Test trying to write to a read-only location (Unix-specific)
    #[cfg(unix)]
    {
        let result = cmd!("touch", "/root/test_file_permission_denied")
            .quiet()
            .run();
        // This should either fail with permission denied or succeed if running as root
        // We just ensure it doesn't panic
        let _ = result;
    }
}

#[test]
fn test_pipeline_single_command() {
    // Test pipeline with only one command (effectively no pipeline)
    let output = cmd!("echo", "single").quiet().output().unwrap();
    assert_eq!(output.trim(), "single");
}

#[test]
fn test_input_with_special_characters() {
    // Test input with special characters, newlines, etc.
    let special_input = "line1\nline2\ttab\r\nwindows_line";
    let output = cmd!("wc", "-l")
        .input(special_input)
        .quiet()
        .output()
        .unwrap();

    // Should count the lines correctly
    let line_count: i32 = output.trim().parse().unwrap();
    assert!(line_count >= 2); // At least 2 lines due to \n characters
}

#[test]
fn test_pipeline_input_override() {
    // Test that pipeline input overrides individual command input
    let output = cmd!("cat")
        .input("original")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .input("pipeline_input")
        .quiet()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "PIPELINE_INPUT");
}

#[test]
fn test_exit_code_handling() {
    // Test command that exits with non-zero status
    let result = cmd!("sh", "-c", "exit 1").quiet().run();
    assert!(result.is_err());
}

#[test]
fn test_very_long_pipeline() {
    // Test a very long pipeline to ensure no stack overflow or resource issues
    let output = cmd!("echo", "start")
        .pipe(cmd!("sed", "s/^/step0_/"))
        .pipe(cmd!("sed", "s/^/step1_/"))
        .pipe(cmd!("sed", "s/^/step2_/"))
        .pipe(cmd!("sed", "s/^/step3_/"))
        .pipe(cmd!("sed", "s/^/step4_/"))
        .quiet()
        .output()
        .unwrap();
    assert!(output.contains("step4_step3_step2_step1_step0_start"));
}

#[test]
fn test_concurrent_execution() {
    // Test that multiple commands can be executed concurrently without interference
    use std::thread;

    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                cmd!("echo", &format!("thread_{}", i))
                    .quiet()
                    .output()
                    .unwrap()
            })
        })
        .collect();

    let results: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Each thread should get its own output
    assert_eq!(results.len(), 3);
    for (i, result) in results.iter().enumerate() {
        assert!(result.contains(&format!("thread_{}", i)));
    }
}

#[test]
fn test_pipe_mode_combinations() {
    // Test all combinations of pipe modes in a single pipeline
    let output = cmd!("sh", "-c", "echo 'out1'; echo 'err1' >&2")
        .pipe_stderr(cmd!(
            "sh",
            "-c",
            "read line; echo \"stderr_to_stdout: $line\"; echo 'err2' >&2"
        ))
        .pipe_both(cmd!(
            "sh",
            "-c",
            "while read line; do echo \"combined: $line\"; done"
        ))
        .pipe(cmd!("wc", "-l"))
        .quiet()
        .output()
        .unwrap();

    // Should count the processed lines
    let line_count: i32 = output.trim().parse().unwrap();
    assert!(line_count >= 1);
}

#[test]
fn test_quiet_mode_suppresses_echo() {
    // Test that quiet mode actually suppresses the command echo
    // This is harder to test directly since echo goes to stdout/stderr
    // but we can at least verify the quiet flag is respected
    let cmd_quiet = cmd!("echo", "test").quiet();
    let cmd_normal = cmd!("echo", "test");

    assert!(cmd_quiet.quiet);
    assert!(!cmd_normal.quiet);

    // Both should produce the same output, but quiet should not echo the command
    let output_quiet = cmd_quiet.output().unwrap();
    let output_normal = cmd_normal.output().unwrap();

    assert_eq!(output_quiet.trim(), "test");
    assert_eq!(output_normal.trim(), "test");
    assert_eq!(output_quiet, output_normal);
}

#[test]
fn test_pipeline_quiet_propagation() {
    // Test that quiet mode propagates correctly through pipelines
    let pipeline_quiet = cmd!("echo", "test")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .quiet();

    let pipeline_normal = cmd!("echo", "test").pipe(cmd!("tr", "[:lower:]", "[:upper:]"));

    assert!(pipeline_quiet.quiet);
    assert!(!pipeline_normal.quiet);

    let output_quiet = pipeline_quiet.output().unwrap();
    let output_normal = pipeline_normal.output().unwrap();

    assert_eq!(output_quiet.trim(), "TEST");
    assert_eq!(output_normal.trim(), "TEST");
}

#[test]
fn test_quiet_mode_inheritance() {
    // Test that quiet mode is inherited when creating pipelines
    let quiet_cmd = cmd!("echo", "hello").quiet();
    let pipeline = quiet_cmd.pipe(cmd!("cat"));

    assert!(pipeline.quiet);

    let normal_cmd = cmd!("echo", "hello");
    let pipeline2 = normal_cmd.pipe(cmd!("cat"));

    assert!(!pipeline2.quiet);
}

#[test]
fn test_builder_pattern_completeness() {
    // Test that all builder methods work correctly in combination
    let cmd = Cmd::new("test_program")
        .arg("arg1")
        .args(vec!["arg2", "arg3"])
        .env("VAR1", "value1")
        .env("VAR2", "value2")
        .cwd("/tmp")
        .input("test input")
        .quiet();

    assert_eq!(cmd.program, OsString::from("test_program"));
    assert_eq!(cmd.args, vec![OsString::from("arg1"), OsString::from("arg2"), OsString::from("arg3")]);
    assert_eq!(cmd.envs.len(), 2);
    assert_eq!(cmd.envs[0], (OsString::from("VAR1"), OsString::from("value1")));
    assert_eq!(cmd.envs[1], (OsString::from("VAR2"), OsString::from("value2")));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/tmp")));
    assert_eq!(cmd.input, Some("test input".to_string()));
    assert!(cmd.quiet);
}

#[test]
fn test_error_message_quality() {
    // Test that error messages are informative
    let result = cmd!("nonexistent_command_xyz123").quiet().run();
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert!(error.message.contains("Failed to spawn command"));
    assert!(error.message.contains("nonexistent_command_xyz123"));
}

#[test]
fn test_zero_length_operations() {
    // Test operations with zero-length inputs and outputs
    let output = cmd!("echo", "-n", "").quiet().output().unwrap();
    assert_eq!(output, "");

    let output = cmd!("cat").input("").quiet().output().unwrap();
    assert_eq!(output, "");
}

#[test]
fn test_command_substitution_safety() {
    // Test that special shell characters are handled safely
    let output = cmd!("echo", "$(echo test)").quiet().output().unwrap();
    // Should output literally, not execute the command substitution
    assert_eq!(output.trim(), "$(echo test)");

    let output = cmd!("echo", "; echo injected").quiet().output().unwrap();
    assert_eq!(output.trim(), "; echo injected");
}
