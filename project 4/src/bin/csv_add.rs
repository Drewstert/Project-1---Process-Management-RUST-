
use anyhow::{bail, Context, Result};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use linux_rust_file_project::{
    csv_path,
    generate_records,
    read_csv,
};

fn main() -> Result<()> {
    println!("PROGRAM 2: APPEND CSV RECORDS");

    let path = csv_path();

    // Check existing records before appending
    let existing = read_csv(&path)?;

    if existing.len() != 100 {
        bail!(
            "CSV add requires 100 existing records, found {}",
            existing.len()
        );
    }

    // Open existing file in append mode
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(&path)
        .with_context(|| {
            format!("Cannot open CSV file {}", path.display())
        })?;

    let length = file.metadata()?.len();

    // Ensure added records begin on a new line
    if length > 0 {
        file.seek(SeekFrom::End(-1))?;

        let mut last_byte = [0u8; 1];
        file.read_exact(&mut last_byte)?;

        if last_byte[0] != b'\n' {
            file.write_all(b"\n")?;
        }
    }

    // Create CSV writer
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    // Generate 50 new records
    let records = generate_records(100, 50);

    for record in &records {
        writer.serialize(record)?;
    }

    writer.into_inner().with_context(|| {
        format!("Cannot finish appending to {}", path.display())
    })?;

    println!("Successfully added 50 CSV records.");
    println!("Expected total records: 150");
    println!("File location: {}", path.display());

    Ok(())
}
