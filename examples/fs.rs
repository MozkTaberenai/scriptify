use scriptant::*;

fn main() -> Result<()> {
    // use_default_logger();

    fs::create_dir("tmp")?;

    fs::write("tmp/a.txt", "abc")?;
    #[cfg(feature = "tracing")]
    read_content("tmp/a.txt")?;

    fs::copy("tmp/a.txt", "tmp/b.txt")?;
    #[cfg(feature = "tracing")]
    read_content("tmp/b.txt")?;

    fs::hard_link("tmp/a.txt", "tmp/h.txt")?;
    #[cfg(feature = "tracing")]
    read_content("tmp/h.txt")?;

    fs::rename("tmp/a.txt", "tmp/c.txt")?;
    #[cfg(feature = "tracing")]
    read_content("tmp/c.txt")?;

    fs::create_dir_all("tmp/d/e")?;

    #[cfg(feature = "tracing")]
    tracing::info!(dir = ?fs::read_dir("tmp")?.collect::<Vec<_>>());

    fs::remove_file("tmp/b.txt")?;

    fs::remove_dir("tmp/d/e")?;

    fs::remove_dir_all("tmp")?;

    Ok(())
}

#[cfg(feature = "tracing")]
fn read_content(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let bytes = fs::read(path)?;
    let string = fs::read_to_string(path)?;
    tracing::info!(?path, ?bytes, ?string);
    Ok(())
}
