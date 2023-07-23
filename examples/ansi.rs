use scriptant::*;

fn main() {
    echo!("red()".red());
    echo!("red().bold()".red().bold());
    echo!("green()".green());
    echo!("green().italic()".green().italic());
    echo!("blue()".blue());
    echo!("blue().underline()".blue().underline());
    echo!("dimmed()".dimmed());
    echo!("dimmed().magenta()".dimmed().magenta());
    echo!("bright_yellow()".bright_yellow());
    echo!();

    echo!("ansi256_rgb6(r,g,b)");
    for r in 0..6 {
        for g in 0..6 {
            for b in 0..6 {
                let n = 16 + r * 36 + g * 6 + b;
                print!(
                    "{} ",
                    format_args!("{n:03}[{r}:{g}:{b}]").ansi256_rgb6(r, g, b)
                );
            }
            echo!();
        }
    }
    echo!();

    echo!("ansi256_grayscale24(lv)");
    for lv in 0..12 {
        let n = 232 + lv;
        print!("{} ", format_args!("{n:03}").ansi256_grayscale24(lv));
    }
    echo!();
    for lv in 12..24 {
        let n = 232 + lv;
        print!("{} ", format_args!("{n:03}").ansi256_grayscale24(lv));
    }
    echo!();
    echo!();

    const LABEL: &str = "count: ";
    const COUNT_POS: u16 = LABEL.len() as u16 + 1;
    print!("{LABEL}");
    for n in 1..=10000 {
        for dummy in 0..10000 {
            std::hint::black_box(dummy);
        }
        print!("{}{}", ansi::Control::CursorMoveInLine(COUNT_POS), n);
    }
    println!();
}
