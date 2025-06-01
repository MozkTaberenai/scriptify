use scriptify::*;

fn main() -> Result<()> {
    // Basic command execution
    cmd!("echo", "Hello, World!").run()?;
    
    // Command with multiple arguments
    cmd!("echo").args(["a", "b", "c"]).run()?;
    
    // Command with environment variable
    cmd!("echo", "hello").env("USER", "alice").run()?;
    
    // Command with working directory
    cmd!("ls", "-la").cwd("src").run()?;
    
    // Get command output
    let date = cmd!("date").output()?;
    echo!("Current date:", date.trim());
    
    // Handle command that might fail
    if let Err(err) = cmd!("unknown_command").run() {
        echo!("Command failed:", err);
    }
    
    // Command piping
    cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .run()?;
    
    // Multiple pipes
    cmd!("date")
        .pipe(cmd!("rev"))
        .pipe(cmd!("tr", "[:upper:]", "[:lower:]"))
        .run()?;
    
    // Pipe with input
    let result = cmd!("tr", "[:lower:]", "[:upper:]")
        .input("hello world")
        .output()?;
    echo!("Uppercase:", result.trim());
    
    // Pipeline with input
    let result = cmd!("sort")
        .pipe(cmd!("uniq"))
        .input("apple\nbanana\napple\ncherry\nbanana")
        .output()?;
    echo!("Unique fruits:", result.trim());
    
    // Quiet execution (no echo)
    cmd!("echo", "This won't be echoed").quiet().run()?;
    
    Ok(())
}