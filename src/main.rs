use polars::prelude::*;
use std::env;
use std::process;

mod analyzers;
mod cli;
mod data_loader;
mod output;

use analyzers::{analyze_overall_data, analyze_year_data};
use data_loader::load_data_from_csv;
use output::{print_overall_analysis, print_year_analysis};

fn main() -> Result<(), PolarsError> {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(String::as_str) == Some("help") {
        cli::print_usage();
        process::exit(0);
    }

    let df = load_data_from_csv("income_data.csv")?;

    // Convert DataFrame to HashMaps
    let (income_2023, income_2024) = data_loader::df_to_hashmaps(&df)?;

    match args.get(1).map(String::as_str) {
        None | Some("all") => {
            // Default behavior: show all data
            let analysis_2023 = analyze_year_data(2023, &income_2023)?;
            print_year_analysis(&analysis_2023);

            let analysis_2024 = analyze_year_data(2024, &income_2024)?;
            print_year_analysis(&analysis_2024);

            let overall_analysis = analyze_overall_data(&income_2023, &income_2024)?;
            print_overall_analysis(&overall_analysis);
        }
        Some("2023") => {
            let analysis_2023 = analyze_year_data(2023, &income_2023)?;
            print_year_analysis(&analysis_2023);
        }
        Some("2024") => {
            let analysis_2024 = analyze_year_data(2024, &income_2024)?;
            print_year_analysis(&analysis_2024);
        }
        Some(_) => {
            println!("Invalid argument. Use 'help' to see available options.");
            cli::print_usage();
            process::exit(1);
        }
    }

    Ok(())
}
