use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CityTemps {
    city: String,
    country: String,
    avg_temp_jan_f: f32,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct CityTempsModified {
    city: String,
    country: String,
    avg_temp_jan_f: f32,
    avg_temp_jan_c: f32,
}

// impl block to convert temp from F to C
impl CityTempsModified {
    fn get_celsius(city_temps: &CityTemps) -> Self {
        let city = &city_temps.city;
        let country = &city_temps.country;
        let avg_temp_jan_f = city_temps.avg_temp_jan_f;
        CityTempsModified {
            city: city.to_string(),
            country: country.to_string(),
            avg_temp_jan_f,
            avg_temp_jan_c: convert_f_to_c(avg_temp_jan_f),
        }
    }
}

fn convert_f_to_c(temp_f: f32) -> f32 {
    // Convert from Fahrenheit to Celsius to 1 decimal precision
    let temp_c = (temp_f - 32.0) * 5.0 / 9.0;
    (temp_c * 10.0).round() / 10.0
}

fn read_csv(input_path: &Path) -> Result<Vec<CityTemps>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(input_path).expect("Unable to read from CSV");
    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let data: Vec<CityTemps> = reader.deserialize().collect::<Result<_, _>>()?;
    println!("Read {} records from {}", data.len(), input_path.display());
    Ok(data)
}

fn construct_city_temps_obj(city_temps: Vec<CityTemps>) -> Vec<CityTempsModified> {
    let mut city_temps_modified: Vec<CityTempsModified> = Vec::new();
    for item in city_temps.iter() {
        city_temps_modified.push(CityTempsModified::get_celsius(item));
    }
    city_temps_modified
}

fn write_csv(city_temps_modified: Vec<CityTempsModified>, output_path: &Path) {
    let mut wtr = csv::Writer::from_path(output_path).expect("Unable to create CSV writer");
    for item in city_temps_modified.iter() {
        wtr.serialize(item).expect("Unable to write CSV record");
    }
    wtr.flush().expect("Unable to flush CSV writer");
    println!(
        "Wrote {} records to {}",
        city_temps_modified.len(),
        output_path.display()
    );
}

fn main() {
    let input_path = Path::new("../data/city_temps.csv");
    let city_temps = read_csv(input_path).expect("Unable to read/open CSV");
    let city_temps_modified = construct_city_temps_obj(city_temps);
    // Write the data to a new CSV file
    let output_path = Path::new("../data/city_temps_modified.csv");
    write_csv(city_temps_modified, output_path);
}

#[cfg(test)]

mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_read_csv() {
        let input_path = Path::new("../data/city_temps.csv");
        let city_temps = read_csv(input_path).expect("Unable to read/open CSV");
        assert_eq!(city_temps.len(), 10);
    }

    #[test]
    fn test_conversion_to_celsius() {
        // Check for positive and negative temperatures in both scales
        let temps = [
            (70.2, 21.2),
            (-14.1, -25.6),
            (25.3, -3.7),
            (29.6, -1.3),
            (52.1, 11.2),
            (18.7, -7.4),
            (-37.5, -38.6),
        ];
        for (f, c) in temps.iter() {
            let temp_c = convert_f_to_c(*f);
            assert_abs_diff_eq!(temp_c, *c, epsilon = 0.05);
        }
    }

    #[test]
    fn test_write_csv() {
        let input_path = Path::new("../data/city_temps.csv");
        let city_temps = read_csv(input_path).expect("Unable to read/open CSV");
        let city_temps_modified = construct_city_temps_obj(city_temps);
        let output_path = Path::new("../data/city_temps_modified.csv");
        write_csv(city_temps_modified, output_path);
        assert!(output_path.exists());
    }
}
