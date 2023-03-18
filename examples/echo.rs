use scriptant::*;

fn main() -> Result<(), AnyError> {
    echo!("tag", "a", "b", "c");
    echo!("ttt", "this is yellow".yellow());
    echo!("xxx", "red bold".red().bold());
    Ok(())
}
