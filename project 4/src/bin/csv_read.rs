
use anyhow::Result;

use linux_rust_file_project::{
    csv_path,
    display_table,
    read_csv,
};

fn main() -> Result<()> {
    println!("PROGRAM 3: READ CSV FILE");

    let path = csv_path();

    println!("Reading CSV file...");
    println!("File: {}", path.display());

    // Read records into a vector
    let records = read_csv(&path)?;

    // Display records
    display_table(&records);

    Ok(())
}
