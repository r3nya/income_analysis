use chrono::{Datelike, NaiveDate};
use polars::prelude::*;
use std::collections::HashMap;

pub fn load_data_from_csv(file_path: &str) -> Result<DataFrame, PolarsError> {
    CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()
}

pub fn df_to_hashmap(df: &DataFrame) -> Result<HashMap<NaiveDate, f64>, PolarsError> {
    let mut income_data: HashMap<NaiveDate, f64> = HashMap::new();

    let date_series = df.column("date")?;
    let amount_series = df.column("amount")?;

    let date_vec: Vec<String> = date_series
        .utf8()?
        .into_iter()
        .flatten()
        .map(|s| s.to_string())
        .collect();
    let amount_vec: Vec<f64> = amount_series.f64()?.into_iter().flatten().collect();

    for (date_str, amount) in date_vec.into_iter().zip(amount_vec.into_iter()) {
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
        income_data.insert(date, amount);
    }

    Ok(income_data)
}

pub fn get_available_years(income_data: &HashMap<NaiveDate, f64>) -> Vec<i32> {
    income_data
        .keys()
        .map(|date| date.year())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_df_to_hashmap() {
        let df = df! [
            "date" => &["2023-01-01", "2023-06-15", "2024-01-01", "2024-06-15"],
            "amount" => &[1000.0, 2000.0, 3000.0, 4000.0]
        ]
        .unwrap();

        let income_data = df_to_hashmap(&df).unwrap();

        assert_eq!(income_data.len(), 4);

        assert_eq!(
            income_data.get(&NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            Some(&1000.0)
        );
        assert_eq!(
            income_data.get(&NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()),
            Some(&2000.0)
        );
        assert_eq!(
            income_data.get(&NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            Some(&3000.0)
        );
        assert_eq!(
            income_data.get(&NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()),
            Some(&4000.0)
        );
    }

    #[test]
    fn test_get_available_years() {
        let mut income_data = HashMap::new();
        income_data.insert(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 1000.0);
        income_data.insert(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap(), 2000.0);
        income_data.insert(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), 3000.0);
        income_data.insert(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(), 4000.0);

        let mut years = get_available_years(&income_data);
        years.sort_unstable();
        assert_eq!(years, vec![2023, 2024]);
    }
}
