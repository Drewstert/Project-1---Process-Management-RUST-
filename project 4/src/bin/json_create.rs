
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Write};

use linux_rust_file_project::{
    create_output_dir,
    generate_records,
    json_path,
};

fn main() -> Result<()> {
    println!("PROGRAM 4: CREATE JSON FILE");

    create_output_dir()?;

    let path = json_path();

    // Generate 100 records
    let records = generate_records(0, 100);

    // Create JSON file
    let file = File::create(&path).with_context(|| {
        format!("Cannot create JSON file {}", path.display())
    })?;

    let mut writer = BufWriter::new(file);

    // Write JSON array
    serde_json::to_writer_pretty(
        &mut writer,
        &records,
    )?;

    // Ensure data is written
    writer.flush()?;

    println!("Successfully created JSON file.");
    println!("Records written: {}", records.len());
    println!("File location: {}", path.display());

    Ok(())
}
