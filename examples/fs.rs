use scriptant::*;

fn main() -> Result<()> {
    fs::create_dir("tmp")?;

    fs::write("tmp/a.txt", "abc")?;
    file_info("tmp/a.txt")?;

    fs::copy("tmp/a.txt", "tmp/b.txt")?;
    file_info("tmp/b.txt")?;

    fs::hard_link("tmp/a.txt", "tmp/h.txt")?;
    file_info("tmp/h.txt")?;

    fs::rename("tmp/a.txt", "tmp/c.txt")?;
    file_info("tmp/c.txt")?;

    fs::create_dir_all("tmp/d/e")?;

    for entry in fs::read_dir("tmp")? {
        let entry = entry?;
        echo!("=>".magenta(), entry.file_name().to_string_lossy());
    }

    fs::remove_file("tmp/b.txt")?;

    fs::remove_dir("tmp/d/e")?;

    fs::remove_dir_all("tmp")?;

    Ok(())
}

fn file_info(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;
    echo!("=>".magenta(), format!("{} bytes", metadata.len()));
    Ok(())
}
