# Income Analysis Tool

## Overview

This Income Analysis Tool is a Rust-based command-line application designed to analyze income data from a CSV file. It provides various statistics including total income, average income, highest and lowest income, and a calculation of 24% of the total income. The tool can analyze data for specific years (2023 and 2024) or provide an overall analysis.

## Features

- Analyze income data for 2023 and 2024
- Calculate overall income statistics
- Display total income, average income, highest income, and lowest income
- Calculate and display 24% of the total income
- Command-line interface for easy use

## Usage

Run the program using Cargo:

```bash
cargo run -- [OPTION]
```

### Options:

- (no option): Show data for all years and overall analysis
- `all`: Same as no option, show all data
- `2023`: Show data for 2023
- `2024`: Show data for 2024
- `help`: Show the help message

### Examples:

```bash
cargo run
cargo run -- all
cargo run -- 2023
cargo run -- 2024
cargo run -- help
```

## Input Data

The tool expects a CSV file named `income_data.csv` in the project root directory. Check `income_data_example.csv` for an example of the expected format.

## Output

The tool will display the following information for each analysis:

- Total income
- Average income
- Highest income
- Lowest income
- 24% of total income
