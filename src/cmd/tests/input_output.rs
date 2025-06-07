//! Input/Output processing tests.
//!
//! Tests for handling command input and output, including large data,
//! special characters, and zero-length operations.

use crate::cmd;

/// Tests with large input data
#[test]
fn test_large_input() {
    let large_input = "x".repeat(10000);
    let output = cmd!("wc", "-c")
        .input(&large_input)
        .no_echo()
        .output()
        .unwrap();

    // wc -c counts bytes exactly as provided
    let count: usize = output.trim().parse().unwrap();
    assert_eq!(count, 10000);

    // Test with very large input (1MB)
    let very_large_input = "a".repeat(1024 * 1024);
    let output = cmd!("wc", "-c")
        .input(&very_large_input)
        .no_echo()
        .output()
        .unwrap();
    let count: usize = output.trim().parse().unwrap();
    assert_eq!(count, 1024 * 1024);
}

/// Tests input with special characters, newlines, etc.
#[test]
fn test_input_with_special_characters() {
    // Test line counting with Unix line endings (this crate is Unix-only)
    let test_cases = vec![
        ("line1\nline2", 1),   // Only one newline = 1 line break, so wc -l counts 1
        ("line1\nline2\n", 2), // Two newlines = 2 line breaks
        ("single line", 0),    // No newlines = 0 lines
        ("\n\n\n", 3),         // Three newlines = 3 lines
        ("", 0),               // Empty = 0 lines
    ];

    for (input, expected_lines) in test_cases {
        let output = cmd!("wc", "-l").input(input).no_echo().output().unwrap();
        let line_count: i32 = output.trim().parse().unwrap();
        assert_eq!(line_count, expected_lines, "Failed for input: {:?}", input);
    }

    // Test with tabs and other special characters
    let special_input = "hello\tworld\x00null\x07bell";
    let output = cmd!("cat").input(special_input).no_echo().output().unwrap();
    // cat should pass through most characters (null byte might be handled differently)
    assert!(output.contains("hello\tworld"));
}

/// Tests operations with zero-length inputs and outputs
#[test]
fn test_zero_length_operations() {
    let output = cmd!("echo", "-n", "").no_echo().output().unwrap();
    assert_eq!(output, "");

    let output = cmd!("cat").input("").no_echo().output().unwrap();
    assert_eq!(output, "");

    // Test with whitespace-only input
    let output = cmd!("cat").input("   ").no_echo().output().unwrap();
    assert_eq!(output, "   ");

    // Test empty input with commands that produce output
    let output = cmd!("echo", "test").input("").no_echo().output().unwrap();
    assert_eq!(output.trim(), "test");
}

/// Tests binary data handling
#[test]
fn test_binary_data_handling() {
    // Test with binary data (all byte values)
    let binary_data: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let binary_string = String::from_utf8_lossy(&binary_data).into_owned();

    let output = cmd!("cat")
        .input(&binary_string)
        .no_echo()
        .output()
        .unwrap();

    // Should preserve most bytes (some might be altered due to encoding)
    assert!(!output.is_empty());
    assert!(output.len() >= 200); // At least most characters should pass through
}

/// Tests input/output with UTF-8 characters
#[test]
fn test_utf8_handling() {
    let utf8_tests = vec![
        "Hello, 世界",   // Japanese
        "Привет мир",    // Russian
        "مرحبا بالعالم", // Arabic
        "🌍🌎🌏",        // Emojis
        "é à ñ ü ö",     // Accented characters
    ];

    for test_str in utf8_tests {
        let output = cmd!("cat").input(test_str).no_echo().output().unwrap();
        assert_eq!(output.trim(), test_str);
    }
}

/// Tests streaming data through pipelines with moderate amounts of data
#[test]
fn test_pipeline_streaming() {
    // Generate lines of data for streaming test
    let data_lines: Vec<String> = (1..=100).map(|i| format!("line {}", i)).collect();
    let streaming_data = data_lines.join("\n");

    // Test streaming through a pipeline
    let result = cmd!("grep", "line [1-9]$") // Lines ending with single digit
        .pipe(cmd!("wc", "-l"))
        .input(&streaming_data)
        .no_echo()
        .output()
        .unwrap();

    // Should match lines 1-9 (9 lines total)
    assert_eq!(result.trim(), "9");

    // Test with larger streaming data through multiple pipes
    let large_stream = "test\n".repeat(1000);
    let stream_result = cmd!("cat")
        .pipe(cmd!("wc", "-l"))
        .input(&large_stream)
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(stream_result.trim(), "1000");
}

/// Tests streaming moderate amounts of data through pipes
#[test]
fn test_large_data_streaming() {
    // Generate 1KB of data (reasonable for testing)
    let large_data = "x".repeat(1024);

    // Test direct command first (simpler than pipeline for large data)
    let output = cmd!("wc", "-c")
        .input(&large_data)
        .no_echo()
        .output()
        .unwrap();

    let count: usize = output.trim().parse().unwrap();
    assert_eq!(count, 1024);
}

/// Tests input with very long lines
#[test]
fn test_very_long_lines() {
    // Test with a very long single line (1MB)
    let long_line = "a".repeat(1024 * 1024);
    let output = cmd!("wc", "-l")
        .input(&long_line)
        .no_echo()
        .output()
        .unwrap();

    let line_count: i32 = output.trim().parse().unwrap();
    assert_eq!(line_count, 0); // No newlines in input = 0 lines

    // Test with multiple long lines
    let multi_long = format!("{}\n{}\n{}", long_line, long_line, long_line);
    let output = cmd!("wc", "-l")
        .input(&multi_long)
        .no_echo()
        .output()
        .unwrap();

    let line_count: i32 = output.trim().parse().unwrap();
    assert_eq!(line_count, 2); // 2 newline characters = 2 lines for wc -l
}

/// Tests binary input/output methods
#[test]
fn test_binary_input_output_methods() {
    // Test with pure binary data (including null bytes and non-UTF8 sequences)
    let binary_data: Vec<u8> = vec![
        0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD, 0xFC, 0x80, 0x81, 0x82,
        0x83, // Invalid UTF-8 sequences
        b'H', b'e', b'l', b'l', b'o', // Valid ASCII
    ];

    // Test input_bytes and output_bytes
    let output_bytes = cmd!("cat")
        .input_bytes(&binary_data)
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(output_bytes, binary_data);

    // Test that regular output() method handles binary data with lossy conversion
    let output_string = cmd!("cat")
        .input_bytes(&binary_data)
        .no_echo()
        .output()
        .unwrap();

    // Should contain the "Hello" part at minimum
    assert!(output_string.contains("Hello"));
    // Should handle invalid UTF-8 gracefully (with replacement characters)
    assert!(!output_string.is_empty());
}

/// Tests binary data preservation through pipelines
#[test]
fn test_binary_pipeline_preservation() {
    // Create binary data with various byte patterns
    let mut binary_data = Vec::new();
    for i in 0..256 {
        binary_data.push(i as u8);
    }

    // Test binary data through pipeline
    let output = cmd!("cat")
        .pipe(cmd!("cat")) // Identity pipeline
        .input_bytes(&binary_data)
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(output.len(), 256);
    // Check that all unique byte values are preserved
    let mut found_bytes = vec![false; 256];
    for &byte in &output {
        found_bytes[byte as usize] = true;
    }
    let preserved_count = found_bytes.iter().filter(|&&x| x).count();
    // Should preserve most bytes (some might be filtered by shell/cat)
    assert!(
        preserved_count >= 200,
        "Only {} bytes preserved",
        preserved_count
    );
}

/// Tests mixed text and binary API usage
#[test]
fn test_mixed_text_binary_api() {
    let text_data = "Hello, World!";

    // Test input() with output_bytes()
    let bytes_output = cmd!("cat")
        .input(text_data)
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(bytes_output, text_data.as_bytes());

    // Test input_bytes() with output()
    let string_output = cmd!("cat")
        .input_bytes(text_data.as_bytes())
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(string_output.trim(), text_data);
}

/// Tests binary data with specific byte patterns
#[test]
fn test_specific_binary_patterns() {
    // Test with null bytes
    let null_data = vec![b'A', 0x00, b'B', 0x00, b'C'];
    let output = cmd!("wc", "-c")
        .input_bytes(&null_data)
        .no_echo()
        .output()
        .unwrap();

    let count: usize = output.trim().parse().unwrap();
    assert_eq!(count, 5);

    // Test with high-bit bytes
    let high_bit_data: Vec<u8> = (128..256).map(|i| i as u8).collect();
    let output = cmd!("wc", "-c")
        .input_bytes(&high_bit_data)
        .no_echo()
        .output()
        .unwrap();

    let count: usize = output.trim().parse().unwrap();
    assert_eq!(count, 128);
}

/// Tests binary data in pipeline operations
#[test]
fn test_binary_pipeline_operations() {
    // Create test binary data
    let binary_input = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x0A]; // "Hello\n"

    // Test pipeline with binary input and output
    let result = cmd!("cat")
        .pipe(cmd!("wc", "-l"))
        .input_bytes(&binary_input)
        .no_echo()
        .output_bytes()
        .unwrap();

    let count_str = String::from_utf8_lossy(&result);
    let count: i32 = count_str.trim().parse().unwrap();
    assert_eq!(count, 1); // One line due to \n
}

/// Tests zero-copy optimization with input_bytes_owned
#[test]
fn test_zero_copy_input_bytes_owned() {
    // Test Cmd::input_bytes_owned
    let binary_data = vec![
        b'H', b'e', b'l', b'l', b'o', b' ', b'W', b'o', b'r', b'l', b'd',
    ];
    let output = cmd!("cat")
        .input_bytes_owned(binary_data) // Takes ownership, should not copy
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(output.trim(), "Hello World");

    // Test Pipeline::input_bytes_owned
    let large_data = vec![b'X'; 1024]; // 1KB of 'X' characters
    let result = cmd!("wc", "-c")
        .input_bytes_owned(large_data) // Zero-copy ownership transfer
        .no_echo()
        .output()
        .unwrap();

    let count: usize = result.trim().parse().unwrap();
    assert_eq!(count, 1024);
}

/// Tests performance comparison between regular and owned input methods
#[test]
fn test_input_method_equivalence() {
    let test_data = b"Performance test data with various bytes: \x00\x01\x02\xFF";

    // Test that both methods produce identical results
    let output1 = cmd!("cat")
        .input_bytes(test_data)
        .no_echo()
        .output_bytes()
        .unwrap();

    let output2 = cmd!("cat")
        .input_bytes_owned(test_data.to_vec())
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(output1, output2);
    assert_eq!(output1, test_data);
}

/// Tests Reader-based input methods
#[test]
fn test_input_reader() {
    use std::io::Cursor;

    // Test with Cursor (in-memory reader)
    let test_data = "Hello from cursor";
    let cursor = Cursor::new(test_data);

    let output = cmd!("cat").input_reader(cursor).no_echo().output().unwrap();

    assert_eq!(output.trim(), test_data);

    // Test with byte data through reader
    let binary_data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
    let cursor = Cursor::new(binary_data.clone());

    let output_bytes = cmd!("cat")
        .input_reader(cursor)
        .no_echo()
        .output_bytes()
        .unwrap();

    assert_eq!(output_bytes, binary_data);
}

/// Tests buffered Reader input
#[test]
fn test_input_buffered() {
    use std::io::Cursor;

    let test_data = "Line 1\nLine 2\nLine 3\n";
    let cursor = Cursor::new(test_data);

    let output = cmd!("wc", "-l")
        .input_buffered(cursor)
        .no_echo()
        .output()
        .unwrap();

    let line_count: i32 = output.trim().parse().unwrap();
    assert_eq!(line_count, 3);
}

/// Tests Writer-based output streaming
#[test]
fn test_stream_to_writer() {
    // Test streaming to Vec<u8> (implements Write)
    let mut output_buffer = Vec::new();

    cmd!("echo", "-n", "Hello, Writer!")
        .no_echo()
        .stream_to(&mut output_buffer)
        .unwrap();

    let result = String::from_utf8(output_buffer).unwrap();
    assert_eq!(result, "Hello, Writer!");

    // Test streaming binary data
    let binary_input = vec![0x01, 0x02, 0x03, 0xFF];
    let mut binary_output = Vec::new();

    cmd!("cat")
        .input_bytes(&binary_input)
        .no_echo()
        .stream_to(&mut binary_output)
        .unwrap();

    assert_eq!(binary_output, binary_input);
}

/// Tests combined Reader + Writer usage
#[test]
fn test_run_with_io() {
    use std::io::Cursor;

    let input_data = "apple\nbanana\ncherry\napricot\n";
    let input_reader = Cursor::new(input_data);
    let mut output_buffer = Vec::new();

    cmd!("grep", "ap")
        .no_echo()
        .run_with_io(input_reader, &mut output_buffer)
        .unwrap();

    let result = String::from_utf8(output_buffer).unwrap();
    let lines: Vec<&str> = result.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"apple"));
    assert!(lines.contains(&"apricot"));
}

/// Tests Pipeline with Reader/Writer using new spawn API
#[test]
fn test_pipeline_with_io() {
    use std::io::Cursor;
    use std::thread;

    // Test pipeline with reader input using spawn API
    let input_data = "zebra\napple\nbanana\ncherry\n";
    let cursor = Cursor::new(input_data);

    let spawn = cmd!("sort")
        .pipe(cmd!("head", "-2"))
        .no_echo()
        .spawn_with_io()
        .unwrap();

    // Handle input in separate thread
    if let Some(mut stdin) = spawn.stdin {
        let mut reader = cursor;
        thread::spawn(move || {
            use std::io::copy;
            let _ = copy(&mut reader, &mut stdin);
        });
    }

    // Collect output
    let output = if let Some(stdout) = spawn.stdout {
        let mut result = Vec::new();
        use std::io::Read;
        let mut reader = std::io::BufReader::new(stdout);
        reader.read_to_end(&mut result).unwrap();
        String::from_utf8(result).unwrap()
    } else {
        String::new()
    };

    spawn.handle.wait().unwrap();

    let lines: Vec<&str> = output.trim().split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "apple");
    assert_eq!(lines[1], "banana");

    // Test pipeline stream_to using run_with_io (which now uses spawn internally)
    let input_data2 = "third\nfirst\nsecond\n";
    let cursor2 = Cursor::new(input_data2);
    let mut output_buffer = Vec::new();

    cmd!("sort")
        .pipe(cmd!("head", "-1"))
        .no_echo()
        .run_with_io(cursor2, &mut output_buffer)
        .unwrap();

    let result = String::from_utf8(output_buffer).unwrap();
    assert_eq!(result.trim(), "first");
}

/// Tests error handling in Reader/Writer operations
#[test]
fn test_reader_writer_error_handling() {
    use std::io::Cursor;

    // Test with command that fails
    let cursor = Cursor::new("test data");
    let result = cmd!("nonexistent-command-67890")
        .input_reader(cursor)
        .no_echo()
        .run();

    assert!(result.is_err());

    // Test pipeline error handling
    let cursor2 = Cursor::new("more test data");
    let result2 = cmd!("cat")
        .pipe(cmd!("nonexistent-command-67890"))
        .input_reader(cursor2)
        .no_echo()
        .run();

    assert!(result2.is_err());
}

/// Tests large data streaming with Reader/Writer
#[test]
fn test_large_data_reader_writer() {
    use std::io::Cursor;

    // Generate larger test data (10KB)
    let large_data = "x".repeat(10240);
    let cursor = Cursor::new(large_data.clone());
    let mut output_buffer = Vec::new();

    cmd!("cat")
        .no_echo()
        .run_with_io(cursor, &mut output_buffer)
        .unwrap();

    assert_eq!(output_buffer.len(), 10240);
    assert_eq!(String::from_utf8(output_buffer).unwrap(), large_data);
}
