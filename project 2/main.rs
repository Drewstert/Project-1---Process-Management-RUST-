use std::fs::{self, File};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    println!("==============================");
    println!("PART 1: CALL STACK");
    println!("==============================");
    call_stack_demo();

    println!("\n==============================");
    println!("PART 2: HEAP AND LIFETIME");
    println!("==============================");
    heap_demo();

    println!("\n==============================");
    println!("PART 3: BUFFERS");
    println!("==============================");
    buffer_demo()?;

    println!("\n==============================");
    println!("PART 4: LINUX MEMORY TOOLS");
    println!("==============================");
    println!("Environment: Native Linux Terminal");
    println!("Debugger: GDB");
    println!("Commands:");
    println!("  gdb ./memory_demo");
    println!("  break function_c");
    println!("  run");
    println!("  bt");

    Ok(())
}

// ============================================================
// PART 1: CALL STACK
// ============================================================

fn call_stack_demo() {
    println!("Active calls: main -> call_stack_demo");

    let result = function_a(5);

    println!("Active calls: main -> call_stack_demo");
    println!("Final returned value: {}", result);
}

fn function_a(parameter_a: i32) -> i32 {
    let local_a = parameter_a + 1;

    println!("\nCALL function_a");
    println!("Parameter: {}", parameter_a);
    println!("Local state: {}", local_a);
    println!("Active calls: main -> call_stack_demo -> function_a");

    let returned_value = function_b(local_a);

    println!("\nRETURNED to function_a");
    println!("Return value received: {}", returned_value);
    println!("Active calls: main -> call_stack_demo -> function_a");

    returned_value + 1
}

fn function_b(parameter_b: i32) -> i32 {
    let local_b = parameter_b * 2;

    println!("\nCALL function_b");
    println!("Parameter: {}", parameter_b);
    println!("Local state: {}", local_b);
    println!(
        "Active calls: main -> call_stack_demo -> function_a -> function_b"
    );

    let returned_value = function_c(local_b);

    println!("\nRETURNED to function_b");
    println!("Return value received: {}", returned_value);
    println!(
        "Active calls: main -> call_stack_demo -> function_a -> function_b"
    );

    returned_value + 1
}

fn function_c(parameter_c: i32) -> i32 {
    let local_c = parameter_c + 10;

    println!("\nCALL function_c");
    println!("Parameter: {}", parameter_c);
    println!("Local state: {}", local_c);
    println!(
        "Active calls: main -> call_stack_demo -> function_a -> function_b -> function_c"
    );

    println!("RETURN function_c -> {}", local_c);

    local_c
}

// ============================================================
// PART 2: HEAP ALLOCATION AND MEMORY LIFETIME
// ============================================================

#[derive(Debug)]
struct HeapData {
    name: String,
    numbers: Vec<i32>,
}

fn heap_demo() {
    println!("Creating heap allocation");

    // Box dynamically allocates the HeapData value.
    // String and Vec also manage dynamically allocated storage.
    let data = Box::new(HeapData {
        name: String::from("Rust heap example"),
        numbers: vec![10, 20, 30, 40],
    });

    println!("Allocated data: {:?}", data);
    println!("Name: {}", data.name);
    println!("Numbers: {:?}", data.numbers);
    println!("Vec length: {}", data.numbers.len());
    println!("Vec capacity: {}", data.numbers.capacity());

    let total: i32 = data.numbers.iter().sum();

    println!("Data used: total = {}", total);

    // Ending ownership causes Rust to drop the value and its
    // owned dynamic allocations.
    println!("Data no longer needed");
    drop(data);
    println!("Heap allocation dropped");
}

// ============================================================
// PART 3: BUFFER DEMONSTRATION
// ============================================================

#[derive(Debug, PartialEq, Eq)]
struct ProcessingResult {
    bytes_processed: usize,
    checksum: u64,
}

fn buffer_demo() -> io::Result<()> {
    let file_name = "input.txt";

    create_test_file(file_name)?;

    println!("\n--- APPROACH 1: WHOLE FILE ---");
    let whole_result = process_whole_file(file_name)?;

    println!("\n--- APPROACH 2: REUSABLE BUFFER ---");
    let chunk_result = process_with_buffer(file_name)?;

    println!("\n--- COMPARISON ---");
    println!(
        "Whole-file bytes: {}",
        whole_result.bytes_processed
    );
    println!(
        "Chunked bytes: {}",
        chunk_result.bytes_processed
    );

    println!(
        "Whole-file checksum: {}",
        whole_result.checksum
    );
    println!(
        "Chunked checksum: {}",
        chunk_result.checksum
    );

    println!(
        "Equivalent results: {}",
        whole_result == chunk_result
    );

    println!(
        "Whole-file approach: retains the entire input in a Vec."
    );
    println!(
        "Chunked approach: reuses one 32-byte buffer."
    );

    Ok(())
}

fn create_test_file(file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;

    for line_number in 0..10 {
        writeln!(
            file,
            "Line {}: Rust buffer demonstration.",
            line_number
        )?;
    }

    // Makes the final read a partial buffer.
    write!(file, "FINAL")?;

    Ok(())
}

// Approach 1:
// fs::read creates a Vec containing the entire file.
fn process_whole_file(
    file_name: &str,
) -> io::Result<ProcessingResult> {
    let data = fs::read(file_name)?;

    println!("Buffer: Vec<u8>");
    println!("Capacity: {}", data.capacity());
    println!("Valid bytes: {}", data.len());

    Ok(calculate_result(&data))
}

// Approach 2:
// One fixed-size buffer is reused until the complete file
// has been processed.
fn process_with_buffer(
    file_name: &str,
) -> io::Result<ProcessingResult> {
    const BUFFER_SIZE: usize = 32;

    let mut file = File::open(file_name)?;
    let mut buffer = [0_u8; BUFFER_SIZE];

    let mut total_bytes = 0;
    let mut checksum = 0_u64;
    let mut chunk_number = 0;

    println!("Buffer capacity: {}", buffer.len());

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        chunk_number += 1;

        println!(
            "Chunk {}: capacity = {}, valid bytes = {}",
            chunk_number,
            buffer.len(),
            bytes_read
        );

        // Only bytes returned by read() are valid for this chunk.
        let valid_data = &buffer[..bytes_read];

        total_bytes += valid_data.len();

        for byte in valid_data {
            checksum += *byte as u64;
        }
    }

    Ok(ProcessingResult {
        bytes_processed: total_bytes,
        checksum,
    })
}

fn calculate_result(data: &[u8]) -> ProcessingResult {
    let mut checksum = 0_u64;

    for byte in data {
        checksum += *byte as u64;
    }

    ProcessingResult {
        bytes_processed: data.len(),
        checksum,
    }
}