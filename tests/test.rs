use mobasserul_haque_mini_project8::{process_customer_data, get_memory_usage, save_to_md};
use std::time::Duration;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_process_customer_data() {
    let input_file = "test_input.csv";
    let output_file = "test_output.csv";

    // Create a test CSV file
    let mut file = File::create(input_file).expect("Failed to create test input file");
    writeln!(
        file,
        "user_id,purchase_amount,loyalty_score,annual_income,purchase_frequency\n\
         1,450,8.5,60000,15\n\
         2,350,6.0,40000,10\n\
         3,500,9.0,75000,20"
    )
    .expect("Failed to write to test input file");

    // Call the function
    let result = process_customer_data(input_file, output_file);
    assert!(result.is_ok(), "process_customer_data returned an error");

    // Verify the output
    let (total_income, avg_income, total_frequency, avg_frequency) = result.unwrap();
    assert_eq!(total_income, 135000.0);
    assert_eq!(avg_income, 67500.0);
    assert_eq!(total_frequency, 35.0);
    assert_eq!(avg_frequency, 17.5);

    // Clean up
    fs::remove_file(input_file).expect("Failed to delete test input file");
    fs::remove_file(output_file).expect("Failed to delete test output file");
}

#[test]
fn test_get_memory_usage() {
    // Call the function
    let memory_usage = get_memory_usage();

    // Verify memory usage is reasonable (in MB)
    assert!(
        memory_usage > 0.0,
        "get_memory_usage should return a positive value"
    );
}

#[test]
fn test_save_to_md() {
    let duration = Duration::from_millis(500);
    let memory_usage = 100.0;
    let initial_cpu_usage = 10.0;
    let final_cpu_usage = 5.0;
    let total_income = 135000.0;
    let avg_income = 67500.0;
    let total_frequency = 35.0;
    let avg_frequency = 17.5;
    let output_file = "test_output.csv";

    // Create a dummy output file
    File::create(output_file).expect("Failed to create test output file");

    // Call the function
    let result = save_to_md(
        &duration,
        memory_usage,
        initial_cpu_usage,
        final_cpu_usage,
        total_income,
        avg_income,
        total_frequency,
        avg_frequency,
        output_file,
    );

    assert!(result.is_ok(), "save_to_md returned an error");

    // Verify the Markdown file exists
    let md_content =
        fs::read_to_string("Rust_Performance.md").expect("Failed to read Rust_Performance.md");

    assert!(
        md_content.contains("# Rust Performance Analysis"),
        "Markdown file does not contain the expected header"
    );
    assert!(
        md_content.contains("Memory Usage: **100.00 MB**"),
        "Markdown file does not contain the expected memory usage"
    );

    // Clean up
    fs::remove_file(output_file).expect("Failed to delete test output file");
    fs::remove_file("Rust_Performance.md").expect("Failed to delete Rust_Performance.md");
}
