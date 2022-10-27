use scriptant::*;

fn main() -> Result<()> {
    cmd!("echo", "a").run()?;

    cmd!("echo").args((0..10).map(|n| n.to_string())).run()?;

    _ = cmd!("echo", "a", "bbb", "c c  c").output()?;

    cmd!("echo", "with env").env("AAA", "aaa").run()?;

    cmd!("ls", "-alF").current_dir("src").run()?;

    _ = cmd!("unknown_command").run();

    _ = cmd!("unknown_command2").output();

    _ = cmd!("sh", "-c", "exit 1").run();

    cmd!("date").pipe(cmd!("cat"))?.pipe(cmd!("cat"))?.run()?;

    Ok(())
}
