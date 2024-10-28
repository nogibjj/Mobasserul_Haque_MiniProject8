use polars::prelude::*;
use std::fs::File;
use std::io::{self, Write};
use std::time::Duration;
use sysinfo::{System, SystemExt, ProcessExt};

pub fn process_customer_data(input_file: &str, output_file: &str) -> Result<(f64, f64, f64, f64), Box<dyn std::error::Error>> {
    let file = File::open(input_file)?;
    let df = CsvReader::new(file)
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    let filtered_df = df
        .lazy()
        .filter(col("purchase_amount").gt(400).and(col("loyalty_score").gt(7)))
        .collect()?;

    let total_income: f64 = filtered_df.column("annual_income")?.sum().unwrap_or(0.0);
    let avg_income: f64 = filtered_df.column("annual_income")?.mean().unwrap_or(0.0);
    let total_frequency: f64 = filtered_df.column("purchase_frequency")?.sum().unwrap_or(0.0);
    let avg_frequency: f64 = filtered_df.column("purchase_frequency")?.mean().unwrap_or(0.0);

    let high_value_customer = filtered_df
        .column("purchase_amount")?
        .gt(500)?
        .into_series()
        .rename("high_value_customer")
        .clone();

    let mut filtered_df = filtered_df.hstack(&[high_value_customer])?;

    let file = File::create(output_file)?;
    CsvWriter::new(file)
        .has_header(true)
        .finish(&mut filtered_df)?;

    Ok((total_income, avg_income, total_frequency, avg_frequency))
}

pub fn save_to_md(
    exec_time: &Duration,
    mem_usage: f64,
    initial_cpu_usage: f32,
    final_cpu_usage: f32,
    total_income: f64,
    avg_income: f64,
    total_frequency: f64,
    avg_frequency: f64,
    output_file: &str,
) -> io::Result<()> {
    let mut file = File::create("Rust_Performance.md")?;

    writeln!(file, "# Rust Performance Analysis")?;
    writeln!(file, "## Execution Time")?;
    writeln!(file, "Execution time: **{:?}**", exec_time)?;
    writeln!(file, "## Resource Usage")?;
    writeln!(file, "Memory Usage: **{:.2} MB**", mem_usage)?;
    writeln!(file, "Initial CPU Usage: **{:.2}%**", initial_cpu_usage)?;
    writeln!(file, "Final CPU Usage: **{:.2}%**", final_cpu_usage)?;
    writeln!(file, "## Processed Data Metrics")?;
    writeln!(file, "- **Total Annual Income:** {:.2}", total_income)?;
    writeln!(file, "- **Average Annual Income:** {:.2}", avg_income)?;
    writeln!(file, "- **Total Purchase Frequency:** {:.2}", total_frequency)?;
    writeln!(file, "- **Average Purchase Frequency:** {:.2}", avg_frequency)?;
    writeln!(file, "## Output")?;
    writeln!(file, "The filtered and transformed data has been saved to `{}`.", output_file)?;

    println!("Markdown report saved to Rust_Performance.md");
    Ok(())
}

pub fn get_memory_usage() -> f64 {
    let s = System::new_all();
    let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
    let process = s.process(pid).expect("Failed to get process");

    // Retrieve resident memory convert to MB
    process.memory() as f64 / (1024.0 * 1024.0)
}
