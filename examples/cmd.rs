use scriptant::*;

fn main() -> Result<(), AnyError> {
    cmd!("echo", "a").run()?;
    cmd!("echo", "with space").run()?;
    cmd!("echo").args((0..10).map(|n| n.to_string())).run()?;
    cmd!("echo", "with env").env("AAA", "aaa").run()?;
    cmd!("ls", "-alF").current_dir("src").run()?;

    if let Err(err) = cmd!("unknown_command", "and", "run").run() {
        echo!(err);
    }

    cmd!("date")
        .pipe(cmd!("rev"))
        .pipe(cmd!("tr", "[:upper:]", "[:lower:]"))
        .run()?;

    let out = cmd!("echo", "pipe input").read_to_string()?;
    echo!("pipe output:".blue(), out.trim());

    b"pipe input from slice\n"
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .run()?;

    let pipe_input = "abcde";
    echo!("pipe input:".blue(), pipe_input);
    pipe_input
        .as_bytes()
        .pipe(cmd!("rev"))
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .run()?;

    let out = cmd!("date", "-uR")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .read_to_string()?;
    echo!("pipe output:".blue(), out.trim());

    let (mut stdin, handle) = cmd!("tr", "[:lower:]", "[:upper:]").write_spawn()?;
    writeln!(stdin, "x")?;
    stdin.write_all(b"y\n")?;
    stdin.write_all("z\n".as_bytes())?;
    drop(stdin);
    handle.wait()?;

    cmd!("echo", "xyz")
        .pipe(BufWriter::new(std::fs::File::create("tmp.txt")?))
        .spawn()?
        .wait()?;
    cmd!("cat", "tmp.txt").run()?;
    fs::remove_file("tmp.txt")?;

    Ok(())
}
