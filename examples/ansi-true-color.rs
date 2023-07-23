use scriptant::*;

fn main() {
    echo!("true_color_rgb(r,g,b)");
    for r in (0u8..=255).step_by(36) {
        for g in (0u8..=255).step_by(36) {
            for b in (0u8..=255).step_by(36) {
                print!(
                    "{} ",
                    format_args!("[{r:03}:{g:03}:{b:03}]").true_color_rgb(r, g, b)
                );
            }
            echo!();
        }
    }
}
