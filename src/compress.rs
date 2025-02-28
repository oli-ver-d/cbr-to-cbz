use std::{fs::File, io::{Read, Write}};
use anyhow::Result;
use tempfile::TempDir;
use zip::{write::SimpleFileOptions, ZipWriter};

pub fn create_cbz(input_dir: &TempDir, output_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    for entry in std::fs::read_dir(input_dir.path())? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;

            zip.start_file(path.file_name().unwrap().to_string_lossy(), SimpleFileOptions::default())?; 
            zip.write_all(&buffer)?;
        }
    }

    zip.finish()?;
    Ok(())
}
