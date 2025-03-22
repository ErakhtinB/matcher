use std::io::{BufRead, Cursor, Write};
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn test_cli_with_example_file() {
    let executable_path = std::env::current_dir()
        .unwrap()
        .join("target/debug/matcher");

    let example_path = std::env::current_dir().unwrap().join("example.csv");

    // Make sure the executable exists
    assert!(
        executable_path.exists(),
        "Executable not found. Run 'cargo build' first."
    );
    assert!(example_path.exists(), "Example CSV file not found.");

    // Run the matcher with the example file
    let output = Command::new(executable_path)
        .arg(example_path)
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Check that the program executed successfully
    assert!(
        output.status.success(),
        "Program execution failed: {}",
        stderr
    );

    // Verify expected output
    assert!(
        stdout.contains("Accepted"),
        "Output should contain 'Accepted'"
    );
    assert!(stdout.contains("Queued"), "Output should contain 'Queued'");

    // More detailed checks could verify the exact matching logic
    // For example, verify that orders are matched in the correct order
    // and with the correct quantities
}

#[test]
fn test_cli_with_generated_file() {
    // Create a temporary CSV file with test orders
    let temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_owned();

    // Open the file for writing
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(&temp_path)
        .unwrap();

    // Write test orders to the file
    writeln!(file, "order_type,side,price,initial_qty,user_id").unwrap();
    writeln!(file, "Lim,Buy,100,10,1").unwrap();
    writeln!(file, "Lim,Sell,95,5,2").unwrap();
    writeln!(file, "Fok,Buy,103,7,3").unwrap();
    writeln!(file, "Ioc,Sell,102,12,4").unwrap();

    let executable_path = std::env::current_dir()
        .unwrap()
        .join("target/debug/matcher");

    // Run the matcher with the temp file
    let output = Command::new(executable_path)
        .arg(&temp_path)
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Check that the program executed successfully
    assert!(
        output.status.success(),
        "Program execution failed: {}",
        stderr
    );

    // Verify output contains expected status messages
    assert!(
        stdout.contains("Accepted"),
        "Output should contain 'Accepted'"
    );
    assert!(stdout.contains("Queued"), "Output should contain 'Queued'");
    assert!(
        stdout.contains("Canceled"),
        "Output should contain 'Canceled'"
    );

    // Optional: Parse the output to verify the matching logic in detail
    let cursor = Cursor::new(stdout);
    let lines: Vec<String> = cursor.lines().map(|l| l.unwrap()).collect();

    // Verify order acceptance and execution
    assert!(lines
        .iter()
        .any(|line| line.contains("Accepted,Lim,Buy,100,10,1")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Accepted,Lim,Sell,95,5,2")));

    // The sell order should match with the buy order (price and quantity compatible)
    // So we should see execution events
}
