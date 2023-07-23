use scriptant::*;

fn main() -> Result<(), AnyError> {
    fs::create_dir("tmp")?;

    fs::write("tmp/a.txt", "abc")?;
    show_metadata("tmp/a.txt")?;

    fs::copy("tmp/a.txt", "tmp/b.txt")?;
    show_metadata("tmp/b.txt")?;

    fs::hard_link("tmp/a.txt", "tmp/h.txt")?;
    show_metadata("tmp/h.txt")?;

    fs::rename("tmp/a.txt", "tmp/c.txt")?;
    show_metadata("tmp/c.txt")?;

    fs::create_dir_all("tmp/d/e")?;

    for entry in fs::read_dir("tmp")? {
        show_metadata(entry?.path())?;
    }

    fs::remove_file("tmp/b.txt")?;

    fs::remove_dir("tmp/d/e")?;

    fs::remove_dir_all("tmp")?;

    Ok(())
}

fn show_metadata(path: impl AsRef<Path>) -> std::io::Result<()> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path)?;
    echo!(path.display().blue(), format_args!("{metadata:?}"));
    Ok(())
}
