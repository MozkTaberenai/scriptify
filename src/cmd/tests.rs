#[cfg(test)]
mod cmd_tests {
    use crate::cmd::*;

    #[test]
    fn test_cmd_new() {
        let cmd = Cmd::new("echo");
        assert_eq!(cmd.program, "echo");
        assert!(cmd.args.is_empty());
        assert!(!cmd.quiet);
    }

    #[test]
    fn test_cmd_with_args() {
        let cmd = cmd!("echo", "hello", "world");
        assert_eq!(cmd.program, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_cmd_builder() {
        let cmd = Cmd::new("ls")
            .arg("-la")
            .env("TEST", "value")
            .cwd("/tmp")
            .quiet();
        
        assert_eq!(cmd.program, "ls");
        assert_eq!(cmd.args, vec!["-la"]);
        assert_eq!(cmd.envs, vec![("TEST".to_string(), "value".to_string())]);
        assert_eq!(cmd.cwd, Some("/tmp".to_string()));
        assert!(cmd.quiet);
    }

    #[test]
    fn test_cmd_output() {
        let output = cmd!("echo", "test").quiet().output().unwrap();
        assert_eq!(output.trim(), "test");
    }

    #[test]
    fn test_cmd_with_input() {
        let output = cmd!("cat")
            .input("hello world")
            .quiet()
            .output()
            .unwrap();
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
}