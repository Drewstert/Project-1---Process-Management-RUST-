Project 4 README

# Linux Rust File Project — Setup and Run Steps

## Requirements

- Ubuntu (or another Linux)
- Rust and Cargo (`cargo --version` should print a version)
- Internet access for the first build (Cargo downloads the dependencies)

Dependencies, already listed in `Cargo.toml`: `anyhow`, `csv`, `serde`, `serde_json`, `tempfile`.

## 1. Go to the project folder

```bash
cd "project 4"
```

## 2. Make sure the manifest is named `Cargo.toml`

Linux is case-sensitive, so a lowercase `cargo.toml` won't be found.

```bash
mv cargo.toml Cargo.toml
```

Skip this step if the file is already named `Cargo.toml`.

## 3. Build

```bash
cargo build
```

## 4. Run the CSV programs (in this order)

```bash
cargo run --bin csv_create   # Program 1: creates the CSV file with 100 records
cargo run --bin csv_add      # Program 2: appends 50 records (150 total)
cargo run --bin csv_read     # Program 3: prints all 150 records as a table
```

## 5. Run the JSON programs (in this order)

```bash
cargo run --bin json_create  # Program 4: creates the JSON file with 100 records
cargo run --bin json_add     # Program 5: adds 50 records (150 total)
cargo run --bin json_read    # Program 6: prints all 150 records as a table
```

Each add program only runs when the file has exactly 100 records. To run an add program again, run its create program first.

## Output files

Both files are written to the `output/` folder inside the project folder (next to `Cargo.toml`):

- `output/records.csv`
- `output/records.json`

The folder is created automatically by the create programs.

## Reset for a fresh run

```bash
rm -rf output
```

This only deletes the project's `output/` folder. Then start again from step 4.