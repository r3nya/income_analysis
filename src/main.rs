use clap::Parser;

mod analyzers;
mod data_loader;
mod output;

use analyzers::{analyze_overall_data, analyze_year_data};
use data_loader::{df_to_hashmap, get_available_years, load_data_from_csv};
use output::{print_overall_analysis, print_year_analysis};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: Option<i32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let df = load_data_from_csv("income_data.csv")?;
    let income_data = df_to_hashmap(&df)?;
    let available_years = get_available_years(&income_data);

    match args.year {
        Some(year) => {
            if available_years.contains(&year) {
                let analysis = analyze_year_data(year, &income_data)?;
                print_year_analysis(&analysis);
            } else {
                println!("No data available for the year {}", year);
                println!("Available years: {:?}", available_years);
            }
        }
        None => {
            // Analyze and print data for each available year
            for &year in &available_years {
                let analysis = analyze_year_data(year, &income_data)?;
                print_year_analysis(&analysis);
            }

            // Analyze and print overall data
            let overall_analysis = analyze_overall_data(&income_data)?;
            print_overall_analysis(&overall_analysis);
        }
    }

    Ok(())
}
