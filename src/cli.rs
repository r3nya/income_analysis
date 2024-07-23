pub fn print_usage() {
    println!("Income Analysis Tool");
    println!("\nUsage:");
    println!("  cargo run [OPTION]");
    println!("\nOptions:");
    println!("  (no option)  Show data for all years and overall analysis");
    println!("  all          Same as no option, show all data");
    println!("  2023         Show data for 2023");
    println!("  2024         Show data for 2024");
    println!("  help         Show this help message");
    println!("\nOutput includes:");
    println!("  - Total income");
    println!("  - Average income");
    println!("  - Highest income");
    println!("  - Lowest income");
    println!("  - 24% of total income");
    println!("\nExamples:");
    println!("  cargo run");
    println!("  cargo run -- all");
    println!("  cargo run -- 2023");
    println!("  cargo run -- 2024");
    println!("  cargo run -- help");
}
