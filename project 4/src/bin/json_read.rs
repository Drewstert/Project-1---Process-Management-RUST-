
use anyhow::Result;

use linux_rust_file_project::{
    display_table,
    json_path,
    read_json,
};

fn main() -> Result<()> {
    println!("PROGRAM 6: READ JSON FILE");

    let path = json_path();

    println!("Reading JSON file...");
    println!("File: {}", path.display());

    // Read JSON into a vector
    let records = read_json(&path)?;

    // Display all records
    display_table(&records);

    Ok(())
}
