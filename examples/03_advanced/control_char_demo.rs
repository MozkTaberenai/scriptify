//! Simple demo showing control character escaping in command echoes

use scriptify::*;

fn main() {
    println!("=== Control Character Escaping Demo ===\n");

    // Tab characters
    println!("Tab characters:");
    cmd!("echo", "hello\tworld\twith\ttabs").run().unwrap();
    println!();

    // Newline characters
    println!("Newline characters:");
    cmd!("echo", "line1\nline2\nline3").run().unwrap();
    println!();

    // Mixed control characters
    println!("Mixed control characters:");
    cmd!("echo", "text\twith\nmixed\rcontrol").run().unwrap();
    println!();

    // Control chars with quotes
    println!("Control chars with single quotes:");
    cmd!("echo", "can't\thandle\nthis").run().unwrap();
    println!();

    // Complex example
    println!("Complex example:");
    cmd!("echo", "path/to/file\twith\nspecial chars")
        .env("TEST_VAR", "value\twith\ttabs")
        .run()
        .unwrap();
    println!();

    println!("Notice how \\t, \\n, \\r are displayed in the command echo");
    println!("but the actual output shows the real control characters!");
}
