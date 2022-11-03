use scriptant::*;

fn main() -> Result<()> {
    cmd!("echo", "a").run()?;
    cmd!("echo", "with space").run()?;

    cmd!("echo").args((0..10).map(|n| n.to_string())).run()?;

    _ = cmd!("echo", "and", "output()").output()?;

    let (mut stdout, mut child) = cmd!("echo", "-n", "abcde").pipeout().spawn()?;
    let mut out = String::new();
    stdout.read_to_string(&mut out)?;
    echo!(out);
    child.wait()?; // this can be omitted

    let (mut stdin, mut child) = cmd!("tr", "[:lower:]", "[:upper:]").pipein().spawn()?;
    std::thread::spawn(move || stdin.write_all(b"abcde\n").unwrap());
    child.wait()?;

    cmd!("echo", "with env").env("AAA", "aaa").run()?;

    cmd!("ls", "-alF").current_dir("src").run()?;

    _ = cmd!("unknown_command", "and", "run").run();

    _ = cmd!("unknown_command", "and", "output").output();

    _ = cmd!("sh", "-c", "echo from sh; exit 1").run();

    cmd!("date")
        .pipe(cmd!("rev"))
        .pipe(cmd!("tr", "[:upper:]", "[:lower:]"))
        .run()?;

    let (mut stdin, mut children) = cmd!("rev")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipein()
        .spawn()?;
    std::thread::spawn(move || writeln!(stdin, "abcde").unwrap());
    children.wait()?;

    let (mut stdout, mut children) = cmd!("date", "-uR")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipeout()
        .spawn()?;
    let mut out = String::new();
    stdout.read_to_string(&mut out)?;
    echo!(out.trim());
    children.wait()?;

    Ok(())
}
