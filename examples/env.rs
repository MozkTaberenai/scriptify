use scriptant::*;

fn main() -> Result<()> {
    #[cfg(feature = "tracing")]
    tracing::info!(current_dir = ?std::env::current_dir()?);
    env::set_current_dir("src")?;
    #[cfg(feature = "tracing")]
    tracing::info!(current_dir = ?std::env::current_dir()?);

    #[cfg(feature = "tracing")]
    tracing::info!(MYENV = ?std::env::var("MYENV"));
    env::set_var("MYENV", "VALUE");
    #[cfg(feature = "tracing")]
    tracing::info!(MYENV = ?std::env::var("MYENV"));

    Ok(())
}
