use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, CpuExt};
use mobasserul_haque_mini_project8::{process_customer_data, save_to_md, get_memory_usage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "data/Customer Purchasing Behaviors.csv";
    let output_file = "data/Rust_Filtered_Customers.csv";

    let mut system = System::new_all();
    system.refresh_all();

    let initial_cpu_usage = system.global_cpu_info().cpu_usage();
    let start = Instant::now();

    let (total_income, avg_income, total_frequency, avg_frequency) =
        process_customer_data(input_file, output_file)?;
    let duration = start.elapsed();

    let memory_usage = get_memory_usage();

    std::thread::sleep(Duration::from_secs(1));
    system.refresh_all();
    let final_cpu_usage = system.global_cpu_info().cpu_usage();

    save_to_md(
        &duration,
        memory_usage,
        initial_cpu_usage,
        final_cpu_usage,
        total_income,
        avg_income,
        total_frequency,
        avg_frequency,
        output_file,
    )?;

    Ok(())
}
