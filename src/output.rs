use crate::analyzers::YearAnalysis;

pub fn print_analysis(analysis: &YearAnalysis) {
    let time_period = if analysis.year == 0 {
        "Overall".to_string()
    } else {
        analysis.year.to_string()
    };

    println!("\nIncome Data for {}:", time_period);
    println!("{}", analysis.data_frame);
    println!(
        "Total Income ({}): ${:.2}",
        time_period, analysis.total_income
    );
    println!(
        "Average Income ({}): ${:.2}",
        time_period, analysis.avg_income
    );
    println!(
        "Highest Income ({}): ${:.2}",
        time_period, analysis.max_income
    );
    println!(
        "Lowest Income ({}): ${:.2}",
        time_period, analysis.min_income
    );
    println!(
        "24% of Total Income ({}): ${:.2}",
        time_period, analysis.twenty_four_percent
    );
}
