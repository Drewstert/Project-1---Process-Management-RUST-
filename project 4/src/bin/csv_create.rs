
use anyhow::{Context, Result};
use std::fs::File;

use linux_rust_file_project::{
    create_output_dir,
    csv_path,
    generate_records,
};

fn main() -> Result<()> {
    println!("PROGRAM 1: CREATE CSV FILE");

    create_output_dir()?;

    let path = csv_path();

    // Create CSV file
    let file = File::create(&path).with_context(|| {
        format!("Cannot create CSV file {}", path.display())
    })?;

    // CSV writer without header row
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    // Generate 100 records
    let records = generate_records(0, 100);

    // Write all records
    for record in &records {
        writer.serialize(record)?;
    }

    // Close writer and check for write errors
    writer.into_inner().with_context(|| {
        format!("Cannot finish writing {}", path.display())
    })?;

    println!("Successfully created CSV file.");
    println!("Records written: {}", records.len());
    println!("File location: {}", path.display());

    Ok(())
}
