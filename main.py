import pandas as pd
import time
from memory_profiler import memory_usage

def process_customer_data(input_file, output_file):
    """Process customer data."""
    # Load the CSV file
    df = pd.read_csv(input_file)

    # Filter customers with purchase_amount > 400 and loyalty_score > 7
    filtered_df = df[(df['purchase_amount'] > 400) & (df['loyalty_score'] > 7)]

    # Calculate total and average annual_income and purchase_frequency
    total_income = filtered_df['annual_income'].sum()
    avg_income = filtered_df['annual_income'].mean()
    total_frequency = filtered_df['purchase_frequency'].sum()
    avg_frequency = filtered_df['purchase_frequency'].mean()

    # Print statistics
    print(f"Total Annual Income: {total_income}")
    print(f"Average Annual Income: {avg_income}")
    print(f"Total Purchase Frequency: {total_frequency}")
    print(f"Average Purchase Frequency: {avg_frequency}")

    # Add a new column: high_value_customer
    filtered_df['high_value_customer'] = filtered_df['purchase_amount'] > 500

    # Save the filtered and transformed data to a new CSV file
    filtered_df.to_csv(output_file, index=False)

    return total_income, avg_income, total_frequency, avg_frequency

def generate_markdown_report(exec_time, mem_usage, total_income, avg_income, total_frequency, avg_frequency, output_file):
    """Generate a Markdown file with performance analysis."""
    with open("Python_Performance.md", "w", encoding="utf-8") as f:  # Specify encoding
        f.write("# Python Performance Analysis\n\n")
        f.write("## Performance Metrics\n\n")
        f.write(f"- **Execution Time:** {exec_time:.2f} seconds\n")
        f.write(f"- **Memory Usage:** {max(mem_usage):.2f} MB\n\n")
        f.write("## Processed Data Metrics\n\n")
        f.write(f"- **Total Annual Income:** {total_income}\n")
        f.write(f"- **Average Annual Income:** {avg_income:.2f}\n")
        f.write(f"- **Total Purchase Frequency:** {total_frequency}\n")
        f.write(f"- **Average Purchase Frequency:** {avg_frequency:.2f}\n\n")
        f.write("## Output\n\n")
        f.write(f"The filtered and transformed data has been saved to `{output_file}`.\n")

def main():
    input_file = "data/Customer Purchasing Behaviors.csv"
    output_file = "data/Python_Filtered_Customers.csv"

    # Measure execution time and memory usage
    start_time = time.time()
    mem_usage = memory_usage((process_customer_data, (input_file, output_file)))
    end_time = time.time()

    exec_time = end_time - start_time

    # Run the processing function and capture metrics
    total_income, avg_income, total_frequency, avg_frequency = process_customer_data(input_file, output_file)

    # Generate the performance analysis Markdown file
    generate_markdown_report(exec_time, mem_usage, total_income, avg_income, total_frequency, avg_frequency, output_file)

if __name__ == "__main__":
    main()
