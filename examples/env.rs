use scriptant::*;

fn main() -> Result<()> {
    env::set_current_dir("src")?;
    env::set_var("MYENV", "VALUE");
    Ok(())
}
