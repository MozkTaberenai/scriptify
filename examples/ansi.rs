use scriptant::*;

fn main() -> Result<(), AnyError> {
    println!(
        "{} {} {} {} {} {} {}",
        "red".red(),
        "red bold".red().bold(),
        "green".green(),
        "green italic".green().italic(),
        "blue".blue(),
        "blue underline".blue().underline(),
        "dimmed".dimmed(),
    );

    println!("{}", "8bit color 6".color_8bit(6));
    println!("{}", "rgb color 64 128 255".color_rgb(64, 128, 255));

    Ok(())
}
