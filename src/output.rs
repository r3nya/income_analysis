use crate::analyzers::YearAnalysis;

pub fn print_year_analysis(analysis: &YearAnalysis) {
    println!("\nIncome Data for {}:", analysis.year);
    println!("{}", analysis.data_frame);
    println!(
        "Total Income for {}: ${:.2}",
        analysis.year, analysis.total_income
    );
    println!(
        "Average Income for {}: ${:.2}",
        analysis.year, analysis.avg_income
    );
    println!(
        "Highest Income for {}: ${:.2}",
        analysis.year, analysis.max_income
    );
    println!(
        "Lowest Income for {}: ${:.2}",
        analysis.year, analysis.min_income
    );
    println!(
        "24% of Total Income for {}: ${:.2}",
        analysis.year, analysis.twenty_four_percent
    );
}

pub fn print_overall_analysis(analysis: &YearAnalysis) {
    println!("\nOverall Income Data:");
    println!("{}", analysis.data_frame);
    println!("Total Income (Overall): ${:.2}", analysis.total_income);
    println!("Average Income (Overall): ${:.2}", analysis.avg_income);
    println!("Highest Income (Overall): ${:.2}", analysis.max_income);
    println!("Lowest Income (Overall): ${:.2}", analysis.min_income);
    println!(
        "24% of Total Income (Overall): ${:.2}",
        analysis.twenty_four_percent
    );
}
