use chrono::NaiveDate;
use polars::prelude::*;
use std::collections::HashMap;

pub struct YearAnalysis {
    pub year: i32,
    pub total_income: f64,
    pub avg_income: f64,
    pub max_income: f64,
    pub min_income: f64,
    pub twenty_four_percent: f64,
    pub data_frame: DataFrame,
}

pub struct OverallAnalysis {
    pub total_income: f64,
    pub avg_income: f64,
    pub max_income: f64,
    pub min_income: f64,
    pub twenty_four_percent: f64,
    pub data_frame: DataFrame,
}

pub fn analyze_year_data(
    year: i32,
    income_data: &HashMap<NaiveDate, f64>,
) -> Result<YearAnalysis, PolarsError> {
    let mut dates = Vec::new();
    let mut amounts = Vec::new();

    for (date, &amount) in income_data.iter() {
        dates.push(*date);
        amounts.push(amount);
    }

    let df = df![
        "date" => dates,
        "amount" => amounts
    ]?;

    let total_income: f64 = df.column("amount")?.sum().unwrap();
    let avg_income: f64 = df.column("amount")?.mean().unwrap();
    let max_income: f64 = df.column("amount")?.max::<f64>().unwrap();
    let min_income: f64 = df.column("amount")?.min::<f64>().unwrap();
    let twenty_four_percent: f64 = total_income * 0.24;

    Ok(YearAnalysis {
        year,
        total_income,
        avg_income,
        max_income,
        min_income,
        twenty_four_percent,
        data_frame: df,
    })
}

pub fn analyze_overall_data(
    income_2023: &HashMap<NaiveDate, f64>,
    income_2024: &HashMap<NaiveDate, f64>,
) -> Result<OverallAnalysis, PolarsError> {
    let mut all_dates = Vec::new();
    let mut all_amounts = Vec::new();

    for (date, &amount) in income_2023.iter().chain(income_2024.iter()) {
        all_dates.push(*date);
        all_amounts.push(amount);
    }

    let df = df![
        "date" => all_dates,
        "amount" => all_amounts
    ]?;

    let total_income: f64 = df.column("amount")?.sum().unwrap();
    let avg_income: f64 = df.column("amount")?.mean().unwrap();
    let max_income: f64 = df.column("amount")?.max::<f64>().unwrap();
    let min_income: f64 = df.column("amount")?.min::<f64>().unwrap();
    let twenty_four_percent: f64 = total_income * 0.24;

    Ok(OverallAnalysis {
        total_income,
        avg_income,
        max_income,
        min_income,
        twenty_four_percent,
        data_frame: df,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_year_data() {
        let mut income_data = HashMap::new();
        income_data.insert(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 1000.0);
        income_data.insert(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(), 2000.0);

        let analysis = analyze_year_data(2023, &income_data).unwrap();

        assert_eq!(analysis.year, 2023);
        assert_eq!(analysis.total_income, 3000.0);
        assert_eq!(analysis.avg_income, 1500.0);
        assert_eq!(analysis.max_income, 2000.0);
        assert_eq!(analysis.min_income, 1000.0);
        assert_eq!(analysis.twenty_four_percent, 720.0);
    }

    #[test]
    fn test_analyze_overall_data() {
        let mut income_2023 = HashMap::new();
        income_2023.insert(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 1000.0);
        income_2023.insert(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(), 2000.0);

        let mut income_2024 = HashMap::new();
        income_2024.insert(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), 3000.0);
        income_2024.insert(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(), 4000.0);

        let analysis = analyze_overall_data(&income_2023, &income_2024).unwrap();

        assert_eq!(analysis.total_income, 10000.0);
        assert_eq!(analysis.avg_income, 2500.0);
        assert_eq!(analysis.max_income, 4000.0);
        assert_eq!(analysis.min_income, 1000.0);
        assert_eq!(analysis.twenty_four_percent, 2400.0);
    }
}
