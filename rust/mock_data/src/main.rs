use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::{Fake, Rng};
use rand::seq::IteratorRandom;
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};
use unicode_normalization::UnicodeNormalization;

struct Location {
    city: String,
    state: String,
    country: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    id: u32,
    name: String,
    age: u8,
    is_married: bool,
    city: String,
    state: String,
    country: String,
}

fn convert_unicode_to_ascii(s: &str) -> String {
    // First ensure that the unicode string is normalized to NFKD form
    s.nfkd().filter(|c| c.is_ascii()).collect::<String>()
}

// Read from CSV file
fn read_cities(path: &Path) -> Vec<Location> {
    let mut rdr =
        csv::Reader::from_path(path).expect("Cannot read cities CSV file. Please check the path.");
    let mut cities: Vec<Location> = Vec::new();
    for record in rdr.records() {
        let record = record.expect("Did not obtain valid record from CSV file.");
        let city = &record[1];
        let state = convert_unicode_to_ascii(&record[7]);
        let country = &record[4];
        cities.push(Location {
            city: city.to_string(),
            state: state.to_string(),
            country: country.to_string(),
        });
    }
    cities
}

fn construct_person(locations: &[Location], id: u32) -> Person {
    let mut r = StdRng::seed_from_u64(id as u64);
    let loc = locations.iter().choose(&mut r);
    let name: String = Name(EN).fake_with_rng(&mut r);
    let age: u8 = r.gen_range(18..65);
    let is_married = r.gen_bool(0.5);
    // Return a vector
    Person {
        id,
        name,
        age,
        is_married,
        city: loc.unwrap().city.to_string(),
        state: loc.unwrap().state.to_string(),
        country: loc.unwrap().country.to_string(),
    }
}

fn write_csv(output_path: &Path, result: Vec<u8>) {
    let mut file = fs::File::create(output_path).expect("Unable to create file for writer");
    file.write_all(result.as_slice())
        .expect("Unable to write to output CSV file");
}

fn run(locations: &[Location], limit: u32) {
    let mut profiles: Vec<Person> = Vec::new();
    for id in 1..=limit {
        let person = construct_person(locations, id);
        profiles.push(person);
    }
    let mut wtr = csv::Writer::from_writer(vec![]);
    // Serialize the data to CSV and write it to file
    for person in profiles.iter() {
        wtr.serialize(person)
            .expect("Unable to serialize output CSV");
    }
    let result = wtr.into_inner().expect("Unable to construct CSV output");
    let output_path = Path::new("./data/persons.csv");
    write_csv(output_path, result);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = Path::new("./data/worldcities.csv");
    let locations = read_cities(path);
    match args.len() {
        // If no args are provided, length is 1 because the first arg is the program name
        1 => {
            run(&locations, 10);
            println!("No limit provided. Defaulting to generating 10 person profiles.");
        }
        _ => {
            let limit = args[1].parse::<u32>().unwrap();
            run(&locations, limit);
            println!("Generating {} person profiles.", limit);
        }
    }
}
