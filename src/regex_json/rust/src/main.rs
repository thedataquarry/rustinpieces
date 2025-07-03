use std::path::Path;

use anyhow::{bail, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    company: String,
    industry: String,
    annual_revenue: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompanyFinal {
    company: String,
    industry: String,
    annual_revenue: String,
    annual_revenue_lower: f64,
    annual_revenue_upper: f64,
}

fn get_data(path: &Path) -> Result<Vec<Company>> {
    if !path.exists() {
        bail!("File {:?} not found", path);
    }
    let contents = fs::read_to_string(path).expect("Could not load from file");
    let data: Vec<Company> = serde_json::from_str(&contents).expect("Could not parse JSON");
    Ok(data)
}

fn get_revenue_multiplier(multiplier: &str) -> f64 {
    match multiplier {
        "K" => 1e3,
        "M" => 1e6,
        "B" => 1e9,
        _ => 0.0,
    }
}

fn calculate_range(revenue_string: &str) -> Result<(f64, f64)> {
    let re = Regex::new(r"\$(\d+\.?\d*)([KMB])?-?\$?(\d+\.?\d*)([KMB])?")?;
    let captures = match re.captures(revenue_string) {
        Some(c) => c,
        None => bail!("Could not parse revenue string"),
    };
    let left_match_num = captures[1].parse::<f64>()?;
    let right_match_num = captures[3].parse::<f64>()?;

    let revenue_lower_multiplier = get_revenue_multiplier(&captures[2]);
    let revenue_upper_multiplier = get_revenue_multiplier(&captures[4]);
    let annual_revenue_lower = left_match_num * revenue_lower_multiplier;
    let annual_revenue_upper = right_match_num * revenue_upper_multiplier;

    Ok((annual_revenue_lower, annual_revenue_upper))
}

fn construct_company_final(
    company: &Company,
    annual_revenue_lower: f64,
    annual_revenue_upper: f64,
) -> Result<CompanyFinal> {
    let value: serde_json::Value = serde_json::json!({
        "company": company.company,
        "industry": company.industry,
        "annual_revenue": company.annual_revenue,
        "annual_revenue_lower": annual_revenue_lower,
        "annual_revenue_upper": annual_revenue_upper,
    });
    let company = serde_json::from_value(value)?;

    Ok(company)
}

fn run() -> Result<Vec<CompanyFinal>> {
    let data: Vec<Company> = get_data(Path::new("../data/companies.json"))?;
    let mut companies: Vec<CompanyFinal> = Vec::new();
    for company in data {
        let (annual_revenue_lower, annual_revenue_upper) =
            calculate_range(&company.annual_revenue)?;
        let company_final =
            construct_company_final(&company, annual_revenue_lower, annual_revenue_upper)?;
        companies.push(company_final);
    }
    let result = serde_json::to_string_pretty(&companies)?;
    println!("{result}");
    // Return the result as an object so it can be tested
    let company = serde_json::from_str(&result)?;

    Ok(company)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error loading data: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revenue_range() {
        let (revenue_lower, revenue_upper) = calculate_range("$1.5M-$2.5M").unwrap();
        assert!(revenue_lower > 0.0);
        assert!(revenue_lower < revenue_upper);
    }

    #[test]
    fn test_run() {
        let result = run().unwrap();
        assert_eq!(result[0].annual_revenue_lower, 10000000.0);
        assert_eq!(result[0].annual_revenue_upper, 20000000.0);
        assert_eq!(result[1].annual_revenue_lower, 7500000.0);
        assert_eq!(result[1].annual_revenue_upper, 8500000.0);
        assert_eq!(result[2].annual_revenue_lower, 500000.0);
        assert_eq!(result[2].annual_revenue_upper, 1000000.0);
        assert_eq!(result[3].annual_revenue_lower, 800000000.0);
        assert_eq!(result[3].annual_revenue_upper, 1000000000.0);
    }
}
