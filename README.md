[![CI/CD](https://github.com/nogibjj/Mobasserul_Haque_MiniProject8/actions/workflows/python_cicd.yml/badge.svg)](https://github.com/nogibjj/Mobasserul_Haque_MiniProject8/actions/workflows/python_cicd.yml)
[![Rust CI/CD Pipeline](https://github.com/nogibjj/Mobasserul_Haque_MiniProject8/actions/workflows/rust_cicd.yml/badge.svg)](https://github.com/nogibjj/Mobasserul_Haque_MiniProject8/actions/workflows/rust_cicd.yml)


# Performance Comparison: Python vs Rust

This project compares Python and Rust implementations for processing customer data based on metrics like execution time, memory usage, and CPU utilization. Both scripts follow the same workflow, ensuring the comparison is fair and focuses on language performance rather than differences in implementation.

# Project Overview

## Objective

The primary goal of this project is to:

1. **Rewrite a Python script for customer data processing in Rust.**
2. **Compare the performance of both implementations** in terms of:
   - Execution time
   - Memory usage
3. **Provide insights into the performance benefits Rust can offer over Python.**

# Features

## 1. Data Processing
The core functionality of the project involves processing customer data to extract meaningful insights. This includes:

### Filtering Customers:
- **Purchase Amount > 400**: Selects customers who have spent more than 400 units of currency.
- **Loyalty Score > 7**: Ensures that only loyal customers (with high loyalty scores) are included.

### Calculations:
- **Total and Average Annual Income**: Aggregates the total annual income of the filtered customers and calculates their average income.
- **Total and Average Purchase Frequency**: Computes how often the filtered customers make purchases, both in total and on average.

### Data Transformation:
- **New Column: `high_value_customer`**: 
  - Marks customers as "high value" if their `purchase_amount` exceeds 500.
  - Provides an additional data point for downstream analysis.

---

## 2. Performance Metrics
To assess and compare the efficiency of the Python and Rust implementations, the project tracks key performance metrics:

### Execution Time:
- Measures how long it takes for each script to process the dataset, highlighting speed differences between Python and Rust.

### Memory Usage:
- Tracks memory consumption during script execution, showcasing Rust's memory efficiency.

### CPU Utilization:
- **Initial CPU Usage**: Captures CPU usage before the script begins execution.
- **Final CPU Usage**: Captures CPU usage after the script completes.

These metrics provide a comprehensive understanding of resource utilization differences between the implementations.

---

## 3. Performance Reports
The results of the performance metrics are saved in Markdown reports for easy readability and comparison:

- **Python Performance Report**:
  - Saved as `Python_Performance.md`.
  - Includes execution time, memory usage, and data processing metrics for the Python script.

- **Rust Performance Report**:
  - Saved as `Rust_Performance.md`.
  - Includes execution time, memory usage, and data processing metrics for the Rust script.

These reports make it easy to evaluate the efficiency of each implementation.

---

## 4. Filtered Data Output
The filtered and processed customer data is saved to CSV files for further analysis or integration into other systems:

- **Python Output**: Saved as `Python_Filtered_Customers.csv`.
- **Rust Output**: Saved as `Rust_Filtered_Customers.csv`.

These files contain the filtered dataset with all computed metrics and the additional `high_value_customer` column.

---

By focusing on data filtering, performance metrics, and clear reporting, this project demonstrates how the same task can be handled in Python and Rust. It provides a direct comparison of their capabilities in terms of speed, memory usage, and efficiency. The project also produces reusable datasets and performance reports for future analysis.

## Directory Structure

```
## Repository Structure

```plaintext
.
├── .devcontainer/                
│   ├── devcontainer.json
│   ├── Dockerfile
├── .github/workflows/            
│   ├── python_cicd.yml           
│   ├── rust_cicd.yml             
├── data/                         
│   ├── Customer Purchasing Behaviors.csv
│   ├── Python_Filtered_Customers.csv
│   ├── Rust_Filtered_Customers.csv
├── src/                          
│   ├── lib.rs                    
│   ├── main.rs                   
├── tests/                        
│   ├── test.rs
├── .coverage                     
├── .gitignore                    
├── Cargo.lock                    
├── Cargo.toml                    
├── Makefile                      
├── Python_Performance.md         
├── README.md                     
├── Rust_Performance.md           
├── main.py                       
├── requirements.txt              
├── test_main.py                  

```

# Installation

## Prerequisites

### Python:
- **Python 3.10 or above**
- **pip** for installing Python dependencies

### Rust:
- Install Rust using [Rustup](https://rustup.rs/).

### Devcontainer (optional):
- Use **VSCode** and **Docker** to set up a development container for a seamless environment.

## Install dependencies:

```
pip install -r requirements.txt
```
## Run the Python script:

```
python main.py
```

## Run Python tests:

```
python -m unittest test_main.py
```
### Rust Setup:

## Build the Rust project:

```
cargo build --release
```
## Run the Rust program:

```
cargo run --release
```
## Run Rust tests:

```
cargo test
```
# Data Processing Workflow

## Input

- **Dataset**: `data/Customer Purchasing Behaviors.csv`
- **Columns**:
  - `user_id`: Unique customer identifier.
  - `age`: Age of the customer.
  - `annual_income`: Annual income of the customer.
  - `purchase_amount`: Total purchase amount.
  - `loyalty_score`: Loyalty score assigned to the customer.
  - `region`: Geographical region of the customer.
  - `purchase_frequency`: Number of purchases made.

---

## Processing

### 1. Filter Customers:
- `purchase_amount > 400`
- `loyalty_score > 7`

### 2. Calculate Metrics:
- **Total and average annual income**
- **Total and average purchase frequency**

### 3. Add Column:
- **`high_value_customer`**: Customers with `purchase_amount > 500`

---

## Output

### Filtered Data:
- **Python**: `data/Python_Filtered_Customers.csv`
- **Rust**: `data/Rust_Filtered_Customers.csv`

### Performance Reports:
- **Python**: `Python_Performance.md`
- **Rust**: `Rust_Performance.md`

# Performance Comparison: Python vs. Rust

## Metrics Overview

| **Metric**           | **Python**  | **Rust**     | **Improvement**    |
|-----------------------|-------------|--------------|--------------------|
| Execution Time        | 5.60 seconds | 7.41 ms      | ~756x faster       |
| Memory Usage          | 106.89 MB    | 19.68 MB     | ~5.4x better       |
| Initial CPU Usage     | N/A          | 8.80%        | N/A                |
| Final CPU Usage       | N/A          | 3.70%        | N/A                |

---

## Insights

- **Execution Time**: Rust is significantly faster due to its low-level memory management and lack of runtime overhead.
- **Memory Usage**: Rust is more efficient, reducing memory consumption by ~5.4x.
- **CPU Utilization**: Rust tracks CPU usage during the execution, providing better insight into resource usage.

---

### **Performance Analysis**

#### **1. Execution Time**
- **Python**: Took **5.60 seconds**, slowed by interpreter overhead, pandas processing, and garbage collection.
- **Rust**: Took **7.41 milliseconds**, benefiting from compiled execution and Polars' efficient memory handling.
- **Improvement**: Rust is **~756x faster** due to its compiled nature and optimized libraries.

#### **2. Memory Usage**
- **Python**: Used **106.89 MB**, driven by pandas' metadata and Python's dynamic typing overhead.
- **Rust**: Used **19.68 MB**, leveraging Polars' lightweight columnar format and Rust's memory efficiency.
- **Improvement**: Rust uses **~5.4x less memory**, making it better for resource-constrained environments.

#### **3. CPU Utilization**
- **Rust Initial CPU Usage**: **8.80%**; **Final CPU Usage**: **3.70%**.
- Rust's compiled optimizations and lightweight libraries ensure efficient CPU usage.
- Python’s CPU usage, while not directly measured, is likely higher due to interpreter overhead.



#### **Why Rust Performs Better**
- **Compiled vs. Interpreted**: Rust is compiled to machine code, while Python relies on runtime interpretation.
- **Memory Management**: Rust's manual memory handling avoids the garbage collection overhead present in Python.
- **Optimized Libraries**: Polars in Rust is built on Apache Arrow, designed for performance, unlike pandas, which adds abstraction overhead.

## CI/CD Pipelines

### Python Workflow (`python_cicd.yml`)
- **Linting**: Ensures code quality with `pylint` and `black`.
- **Testing**: Runs `unittest` with `pytest`.

### Rust Workflow (`rust_cicd.yml`)
- **Build**: Compiles the Rust code.
- **Testing**: Executes Rust tests using `cargo test`.

## Conclusion

This project demonstrates that Rust outperforms Python in terms of:

- **Speed**: Rust executes **756x faster**.
- **Memory Usage**: Rust consumes **5.4x less memory**.

However, the choice of language depends on the use case:

- **Python**: Remains the go-to language for data science and prototyping due to its simplicity and extensive libraries.
- **Rust**: Is better suited for performance-intensive applications, offering speed and memory efficiency.

By understanding these trade-offs, developers can choose the right tool for their specific requirements.

## References
* https://github.com/nogibjj/rust-data-engineering

