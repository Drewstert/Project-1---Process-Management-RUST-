
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

// Record structure with eight fields
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub phone_number: String,
    pub street_address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

// Generate fictional records
pub fn generate_records(start: usize, count: usize) -> Vec<Record> {
    let mut records = Vec::new();

    for i in start..start + count {
        let first_name = if i < 100 {
            "Initial"
        } else {
            "Added"
        };

        let record = Record {
            first_name: first_name.to_string(),
            last_name: format!("Person{:03}", i + 1),
            date_of_birth: format!(
                "199{}-{:02}-{:02}",
                i % 10,
                (i % 12) + 1,
                (i % 28) + 1
            ),
            phone_number: format!("202-555-{:04}", i),
            street_address: format!("{} Main Street", i + 1),
            city: "Fictionville".to_string(),
            state: "AL".to_string(),
            zip_code: "00501".to_string(),
        };

        records.push(record);
    }

    records
}

// Get output directory
pub fn output_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("output")
}

// Get CSV file path
pub fn csv_path() -> PathBuf {
    output_dir().join("records.csv")
}

// Get JSON file path
pub fn json_path() -> PathBuf {
    output_dir().join("records.json")
}

// Create output directory
pub fn create_output_dir() -> Result<()> {
    fs::create_dir_all(output_dir())
        .context("Failed to create output directory")?;

    Ok(())
}

// Read CSV records
pub fn read_csv(path: &Path) -> Result<Vec<Record>> {
    let file = File::open(path).with_context(|| {
        format!("CSV read: cannot open {}", path.display())
    })?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut records = Vec::new();

    for result in reader.deserialize::<Record>() {
        match result {
            Ok(record) => records.push(record),

            Err(error) => {
                if matches!(error.kind(), csv::ErrorKind::Io(_)) {
                    bail!(
                        "CSV file I/O error in {}: {}",
                        path.display(),
                        error
                    );
                }

                bail!(
                    "CSV parsing error in {}: {}",
                    path.display(),
                    error
                );
            }
        }
    }

    Ok(records)
}

// Read JSON records
pub fn read_json(path: &Path) -> Result<Vec<Record>> {
    let file = File::open(path).with_context(|| {
        format!("JSON read: cannot open {}", path.display())
    })?;

    let records: Vec<Record> =
        serde_json::from_reader(file).with_context(|| {
            format!(
                "JSON parsing error in {}",
                path.display()
            )
        })?;

    Ok(records)
}

// Display records as a table
pub fn display_table(records: &[Record]) {
    println!(
        "{:<12} {:<15} {:<15} {:<16} {:<22} {:<15} {:<8} {:<10}",
        "First Name",
        "Last Name",
        "Date of Birth",
        "Phone Number",
        "Street Address",
        "City",
        "State",
        "ZIP Code"
    );

    println!("{}", "-".repeat(125));

    for record in records {
        println!(
            "{:<12} {:<15} {:<15} {:<16} {:<22} {:<15} {:<8} {:<10}",
            record.first_name,
            record.last_name,
            record.date_of_birth,
            record.phone_number,
            record.street_address,
            record.city,
            record.state,
            record.zip_code
        );
    }

    println!("\nTotal records read: {}", records.len());
}
