use chrono::{Datelike, NaiveDate};
use polars::prelude::*;
use std::collections::HashMap;

pub fn load_data_from_csv(file_path: &str) -> Result<DataFrame, PolarsError> {
    CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()
}

pub fn df_to_hashmaps(
    df: &DataFrame,
) -> Result<(HashMap<NaiveDate, f64>, HashMap<NaiveDate, f64>), PolarsError> {
    let mut income_2023: HashMap<NaiveDate, f64> = HashMap::new();
    let mut income_2024: HashMap<NaiveDate, f64> = HashMap::new();

    let date_series = df.column("date")?;
    let amount_series = df.column("amount")?;

    let date_vec: Vec<String> = date_series
        .utf8()?
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|opt_s| opt_s.unwrap().to_string())
        .collect();
    let amount_vec: Vec<f64> = amount_series
        .f64()?
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|opt_f| opt_f.unwrap())
        .collect();

    for (date_str, amount) in date_vec.into_iter().zip(amount_vec.into_iter()) {
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
        if date.year() == 2023 {
            income_2023.insert(date, amount);
        } else if date.year() == 2024 {
            income_2024.insert(date, amount);
        }
    }

    Ok((income_2023, income_2024))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_df_to_hashmaps() {
        let df = df! [
            "date" => &["2023-01-01", "2023-06-15", "2024-01-01", "2024-06-15"],
            "amount" => &[1000.0, 2000.0, 3000.0, 4000.0]
        ]
        .unwrap();

        let (income_2023, income_2024) = df_to_hashmaps(&df).unwrap();

        assert_eq!(income_2023.len(), 2);
        assert_eq!(income_2024.len(), 2);

        assert_eq!(
            income_2023.get(&NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
            Some(&1000.0)
        );
        assert_eq!(
            income_2023.get(&NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()),
            Some(&2000.0)
        );
        assert_eq!(
            income_2024.get(&NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            Some(&3000.0)
        );
        assert_eq!(
            income_2024.get(&NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()),
            Some(&4000.0)
        );
    }
}
