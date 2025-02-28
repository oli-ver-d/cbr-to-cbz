use anyhow::{Ok, Result};
use unrar::Archive;
use tempfile::{tempdir, TempDir};

pub fn extract_cbr(cbr_path: &str) -> Result<TempDir> {
    let temp_dir = tempdir()?;
    let out_path = temp_dir.path().to_path_buf();

    let mut archive = Archive::new(cbr_path)
        .open_for_processing()?;

    while let Some(header) = archive.read_header()? {
        archive = if header.entry().is_file() {
            header.extract_with_base(&out_path)?
        } else {
            header.skip()?
        };
    }

    Ok(temp_dir)
}
