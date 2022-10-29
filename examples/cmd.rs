use scriptant::*;

fn main() -> Result<()> {
    cmd!("echo", "a").run()?;
    cmd!("echo", "with space").run()?;

    cmd!("echo").args((0..10).map(|n| n.to_string())).run()?;

    _ = cmd!("echo", "and", "output()").output()?;

    let mut reader = cmd!("echo", "-n", "abcde").pipe()?.into_reader();
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    echo!("read_to_string:", buf);

    cmd!("echo", "with env").env("AAA", "aaa").run()?;

    cmd!("ls", "-alF").current_dir("src").run()?;

    _ = cmd!("unknown_command", "and", "run").run();

    _ = cmd!("unknown_command", "and", "output").output();

    _ = cmd!("sh", "-c", "echo from sh; exit 1").run();

    cmd!("date")
        .pipe()?
        .into_cmd(cmd!("cat"))
        .pipe()?
        .into_cmd(cmd!("cat"))
        .run()?;

    Ok(())
}
