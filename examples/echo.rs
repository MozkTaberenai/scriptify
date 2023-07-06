use scriptant::*;

fn main() -> Result<(), AnyError> {
    echo!("a", "b", "c");
    echo!();
    echo!("color", "red".red(), "green".green());
    echo!("red bold".red().bold());
    err!("this", "is", "error!!");
    wrn!("this", "is", "warnning!");
    Ok(())
}
