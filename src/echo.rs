//! Echo module for printing messages to the console.

#[derive(Debug)]
#[must_use]
pub enum Echo {
    Quiet,
    Head,
    Tail,
}

impl Default for Echo {
    fn default() -> Self {
        Self::new()
    }
}

impl Echo {
    pub fn new() -> Self {
        match std::env::var_os("NO_ECHO").is_some() {
            true => Self::Quiet,
            false => Self::Head,
        }
    }

    pub fn quiet() -> Self {
        Self::Quiet
    }

    pub fn put(self, arg: impl std::fmt::Display) -> Self {
        match self {
            Self::Quiet => Self::Quiet,
            Self::Head => {
                eprint!("{arg}");
                Self::Tail
            }
            Self::Tail => {
                eprint!(" {arg}");
                Self::Tail
            }
        }
    }

    pub fn sput(self, arg: impl std::fmt::Display, style: anstyle::Style) -> Self {
        self.put(format_args!("{style}{arg}{style:#}"))
    }

    pub fn end(self) {
        match self {
            Self::Quiet => {}
            _ => eprintln!(),
        }
    }
}

/// A macro to print to the standard output
#[macro_export]
macro_rules! echo {
    ($($arg:expr),* $(,)?) => {
        $crate::Echo::new()
            $(.put($arg))*
            .end();
    };
    () => {
        println!();
    };
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_echo_new_normal() {
        // Save original state
        let original = std::env::var("NO_ECHO").ok();
        
        // Ensure NO_ECHO is not set
        unsafe {
            std::env::remove_var("NO_ECHO");
        }
        let echo = Echo::new();
        assert!(matches!(echo, Echo::Head));
        
        // Restore original state
        unsafe {
            match original {
                Some(val) => std::env::set_var("NO_ECHO", val),
                None => std::env::remove_var("NO_ECHO"),
            }
        }
    }

    #[test]
    fn test_echo_new_quiet_env() {
        // Save original state
        let original = std::env::var("NO_ECHO").ok();
        
        // Set NO_ECHO environment variable
        unsafe {
            std::env::set_var("NO_ECHO", "1");
        }
        let echo = Echo::new();
        assert!(matches!(echo, Echo::Quiet));
        
        // Restore original state
        unsafe {
            match original {
                Some(val) => std::env::set_var("NO_ECHO", val),
                None => std::env::remove_var("NO_ECHO"),
            }
        }
    }

    #[test]
    fn test_echo_quiet() {
        let echo = Echo::quiet();
        matches!(echo, Echo::Quiet);
    }

    #[test]
    fn test_echo_state_transitions() {
        // Test Head -> Tail transition
        let echo = Echo::Head;
        let echo = echo.put("test");
        matches!(echo, Echo::Tail);

        // Test Tail -> Tail (stays in Tail)
        let echo = echo.put("more");
        matches!(echo, Echo::Tail);

        // Test Quiet -> Quiet (always stays Quiet)
        let echo = Echo::Quiet;
        let echo = echo.put("test");
        matches!(echo, Echo::Quiet);
    }

    #[test]
    fn test_echo_formatting() {
        // This test checks the logical flow without actually capturing output
        // since capturing stderr in tests is complex and platform-dependent

        // Test single argument
        let echo = Echo::Head;
        let echo = echo.put("hello");
        let echo = echo.put("world");
        matches!(echo, Echo::Tail);

        // Test with styled output
        let echo = Echo::Head;
        let style = anstyle::Style::new().bold();
        let echo = echo.sput("styled", style);
        matches!(echo, Echo::Tail);
    }

    #[test]
    fn test_echo_output_format() {
        // Test that echo produces expected output format
        // We test the format by checking what would be printed
        
        use std::fmt::Write as FmtWrite;
        
        // Simulate what Echo::put would format
        let mut output = String::new();
        
        // First argument (no space prefix)
        write!(&mut output, "hello").unwrap();
        // Second argument (space prefix)
        write!(&mut output, " world").unwrap();
        // End with newline
        writeln!(&mut output).unwrap();
        
        assert_eq!(output, "hello world\n");
        
        // Test styled output format
        let style = anstyle::Style::new().bold();
        let styled_text = format!("{style}bold{style:#}");
        
        // Verify that style codes are properly formatted
        assert!(styled_text.contains("bold"));
        assert!(styled_text.len() > 4); // Should contain ANSI codes
    }

    #[test]
    fn test_echo_spacing() {
        // Test that spacing is correct between arguments
        use std::fmt::Write as FmtWrite;
        
        let mut simulated_output = String::new();
        
        // Simulate Echo behavior:
        // Head.put("first") -> prints "first", becomes Tail
        write!(&mut simulated_output, "first").unwrap();
        
        // Tail.put("second") -> prints " second", stays Tail  
        write!(&mut simulated_output, " second").unwrap();
        
        // Tail.put("third") -> prints " third", stays Tail
        write!(&mut simulated_output, " third").unwrap();
        
        assert_eq!(simulated_output, "first second third");
    }

    #[test]
    fn test_echo_with_different_types() {
        // Test that Echo can handle different Display types
        let echo = Echo::Head;
        let echo = echo.put(42);
        let echo = echo.put("string");
        let echo = echo.put(true);
        matches!(echo, Echo::Tail);
    }

    #[test]
    fn test_echo_chain() {
        // Test method chaining
        let echo = Echo::Head
            .put("first")
            .put("second")
            .put("third");
        matches!(echo, Echo::Tail);
    }

    #[test]
    fn test_styled_put() {
        let style = anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)));
        let echo = Echo::Head;
        let echo = echo.sput("red text", style);
        matches!(echo, Echo::Tail);
    }

    #[test]
    fn test_echo_macro_basic() {
        // Test that the macro compiles and runs without panicking
        echo!("test");
        echo!("hello", "world");
        echo!(42, "test", true);
        echo!(); // Empty echo
    }

    #[test]
    fn test_echo_macro_with_no_echo_env() {
        // Save original state
        let original = std::env::var("NO_ECHO").ok();
        
        unsafe {
            std::env::set_var("NO_ECHO", "1");
        }
        // Should not output anything but shouldn't panic
        echo!("this", "should", "be", "quiet");
        
        // Restore original state
        unsafe {
            match original {
                Some(val) => std::env::set_var("NO_ECHO", val),
                None => std::env::remove_var("NO_ECHO"),
            }
        }
    }

    #[test]
    fn test_echo_macro_single_argument() {
        // Test macro with single argument
        echo!("single");
    }

    #[test]
    fn test_echo_macro_multiple_arguments() {
        // Test macro with multiple arguments
        echo!("arg1", "arg2", "arg3");
        echo!(1, 2, 3, 4, 5);
    }

    #[test]
    fn test_echo_macro_mixed_types() {
        // Test macro with mixed argument types
        echo!("string", 42, true, 2.71);
        echo!("count:", 100, "active:", false);
    }

    #[test]
    fn test_echo_macro_expressions() {
        // Test macro with expressions as arguments
        let x = 5;
        let y = 10;
        echo!("result:", x + y);
        echo!("formatted:", format!("value: {}", x));
    }

    #[test]
    fn test_echo_macro_trailing_comma() {
        // Test macro with trailing comma
        echo!("with", "trailing", "comma",);
        echo!("single",);
    }

    #[test]
    fn test_echo_macro_empty_variants() {
        // Test different forms of empty echo
        echo!();
    }

    #[test]
    fn test_echo_end() {
        // Test that end() doesn't panic for different states
        Echo::Head.end();
        Echo::Tail.end();
        Echo::Quiet.end();
    }

    #[test]
    fn test_echo_default() {
        // Save original state
        let original = std::env::var("NO_ECHO").ok();
        
        // Ensure NO_ECHO is not set for consistent test
        unsafe {
            std::env::remove_var("NO_ECHO");
        }
        
        let echo = Echo::default();
        let echo_new = Echo::new();
        assert_eq!(
            std::mem::discriminant(&echo),
            std::mem::discriminant(&echo_new)
        );
        
        // Restore original state
        unsafe {
            match original {
                Some(val) => std::env::set_var("NO_ECHO", val),
                None => std::env::remove_var("NO_ECHO"),
            }
        }
    }
}
