use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

mod test_main;

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

fn get_data(path: &Path) -> Vec<Company> {
    if !path.exists() {
        panic!("File {:?} not found", path);
    }
    let contents = fs::read_to_string(path).expect("Could not load from file");
    let data: Vec<Company> = serde_json::from_str(&contents).unwrap();
    data
}

fn get_revenue_multiplier(multiplier: &str) -> f64 {
    match multiplier {
        "K" => 1e3,
        "M" => 1e6,
        "B" => 1e9,
        _ => 0.0,
    }
}

fn calculate_range(revenue_string: &str) -> (f64, f64) {
    let re = Regex::new(r"\$(\d+\.?\d*)([KMB])?-?\$?(\d+\.?\d*)([KMB])?").unwrap();
    let captures = re.captures(revenue_string).unwrap();
    let left_match_num = captures[1].parse::<f64>().unwrap();
    let right_match_num = captures[3].parse::<f64>().unwrap();

    let revenue_lower_multiplier = get_revenue_multiplier(&captures[2]);
    let revenue_upper_multiplier = get_revenue_multiplier(&captures[4]);
    let annual_revenue_lower = left_match_num * revenue_lower_multiplier;
    let annual_revenue_upper = right_match_num * revenue_upper_multiplier;

    (annual_revenue_lower, annual_revenue_upper)
}

fn construct_company_final(
    company: &Company,
    annual_revenue_lower: f64,
    annual_revenue_upper: f64,
) -> CompanyFinal {
    let value: serde_json::Value = serde_json::json!({
        "company": company.company,
        "industry": company.industry,
        "annual_revenue": company.annual_revenue,
        "annual_revenue_lower": annual_revenue_lower,
        "annual_revenue_upper": annual_revenue_upper,
    });
    serde_json::from_value(value).unwrap()
}

fn run() -> Vec<CompanyFinal> {
    let data: Vec<Company> = get_data(Path::new("data/companies.json"));
    let mut companies: Vec<CompanyFinal> = Vec::new();
    for company in data {
        let (annual_revenue_lower, annual_revenue_upper) = calculate_range(&company.annual_revenue);
        let company_final =
            construct_company_final(&company, annual_revenue_lower, annual_revenue_upper);
        companies.push(company_final);
    }
    let result = serde_json::to_string_pretty(&companies).unwrap();
    println!("{}", result);
    // Return the result as an object so it can be tested
    serde_json::from_str(&result).unwrap()
}

fn main() {
    run();
}
