
use anyhow::{bail, Context, Result};
use std::io::Write;
use tempfile::NamedTempFile;

use linux_rust_file_project::{
    generate_records,
    json_path,
    read_json,
};

fn main() -> Result<()> {
    println!("PROGRAM 5: ADD JSON RECORDS");

    let path = json_path();

    // Read existing JSON array
    let mut records = read_json(&path)?;

    println!("Existing records: {}", records.len());

    // Expect 100 records before adding
    if records.len() != 100 {
        bail!(
            "JSON add requires 100 existing records, found {}",
            records.len()
        );
    }

    // Generate 50 new records
    let new_records = generate_records(100, 50);

    // Add new records to array
    records.extend(new_records);

    // Write to temporary file first
    let directory = path.parent().unwrap();

    let mut temp_file =
        NamedTempFile::new_in(directory)
            .context("Cannot create temporary JSON file")?;

    serde_json::to_writer_pretty(
        temp_file.as_file_mut(),
        &records,
    )
    .context("Cannot serialize JSON records")?;

    // Flush and sync temporary file
    temp_file.as_file_mut().flush()?;
    temp_file.as_file().sync_all()?;

    // Replace original file
    temp_file.persist(&path).with_context(|| {
        format!(
            "Cannot replace JSON file {}",
            path.display()
        )
    })?;

    println!("Successfully added 50 JSON records.");
    println!("Total records: {}", records.len());
    println!("File location: {}", path.display());

    Ok(())
}
