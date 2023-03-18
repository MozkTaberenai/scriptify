use scriptant::*;

fn main() -> Result<(), AnyError> {
    cmd!("echo", "a").run()?;
    cmd!("echo", "with space").run()?;

    cmd!("echo").args((0..10).map(|n| n.to_string())).run()?;

    _ = cmd!("echo", "and", "output()").output()?;

    let (mut stdout, mut child) = cmd!("echo", "-n", "abcde").pipeout().spawn()?;
    let mut out = String::new();
    stdout.read_to_string(&mut out)?;
    echo!("string", out);
    child.wait()?; // this can be omitted

    let (mut stdin, mut child) = cmd!("tr", "[:lower:]", "[:upper:]").pipein().spawn()?;
    std::thread::spawn(move || stdin.write_all(b"abcde\n").unwrap());
    child.wait()?;

    cmd!("echo", "with env").env("AAA", "aaa").run()?;

    cmd!("ls", "-alF").current_dir("src").run()?;

    if let Err(err) = cmd!("unknown_command", "and", "run").run() {
        echo!("err", err.red());
    }

    if let Err(err) = cmd!("unknown_command", "and", "output").output() {
        echo!("err", err.red());
    }

    if let Err(err) = cmd!("sh", "-c", "echo from sh; exit 1").run() {
        echo!("err", err.red());
    }

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
    echo!("string", out.trim());
    children.wait()?;

    Ok(())
}
