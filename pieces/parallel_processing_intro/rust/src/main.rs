use std::{
    fs,
    io::{self, Write},
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::{bail, Result};
use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::{Fake, Rng};
use rand::seq::IteratorRandom;
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

struct Location {
    city: String,
    state: String,
    country: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    id: usize,
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

fn construct_person(locations: &[Location], id: usize) -> Result<Person> {
    let mut r = StdRng::seed_from_u64(id as u64);
    let loc = match locations.iter().choose(&mut r) {
        Some(l) => l,
        None => bail!("No locations found"),
    };
    let name: String = Name(EN).fake_with_rng(&mut r);
    let age: u8 = r.gen_range(18..65);
    let is_married = r.gen_bool(0.5);
    Ok(Person {
        id,
        name,
        age,
        is_married,
        city: loc.city.to_string(),
        state: loc.state.to_string(),
        country: loc.country.to_string(),
    })
}

fn write_csv(output_path: &Path, result: Vec<u8>) -> Result<()> {
    let mut file = fs::File::create(output_path)?;
    file.write_all(result.as_slice())?;

    Ok(())
}

fn spinner(message_receiver: Arc<Mutex<mpsc::Receiver<&'static str>>>) {
    let rate = Duration::from_millis(100);
    loop {
        if let Ok(message) = message_receiver.lock().unwrap().try_recv() {
            if message == "done" {
                break;
            }
        }

        print!("Generating data |\r");
        io::stdout().flush().expect("Error generatring spinner");
        thread::sleep(rate);
        print!("Generating data /\r");
        io::stdout().flush().expect("Error generatring spinner");
        thread::sleep(rate);
        print!("Generating data -\r");
        io::stdout().flush().expect("Error generatring spinner");
        thread::sleep(rate);
        print!("Generating data \\\r");
        io::stdout().flush().expect("Error generatring spinner");
        thread::sleep(rate);
    }
}

fn generate_data(
    message_sender: mpsc::Sender<&'static str>,
    num: usize,
    output_path: &Path,
) -> Result<()> {
    let path = Path::new("../data/worldcities.csv");
    let locations = read_cities(path);

    let mut profiles: Vec<Person> = Vec::new();
    for id in 1..=num {
        let person = construct_person(&locations, id)?;
        profiles.push(person);
    }
    let mut wtr = csv::Writer::from_writer(vec![]);
    // Serialize the data to CSV and write it to file
    for person in profiles.iter() {
        wtr.serialize(person)?;
    }

    let result = wtr.into_inner()?;
    write_csv(output_path, result)?;

    message_sender.send("done")?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num: usize = match args.len() {
        // If no args are provided, length is 1 because the first arg is the program name
        1 => 1000,
        _ => args[1].parse::<usize>().unwrap(),
    };
    let (message_sender, message_receiver) = mpsc::channel();
    let message_receiver_mutex = Arc::new(Mutex::new(message_receiver));
    let output_path = Path::new("../data/persons.csv");

    let generator_handler = thread::spawn(move || {
        let result = generate_data(message_sender.clone(), num, output_path);
        if let Err(e) = result {
            eprintln!("Error generating data: {e}");
        };
    });
    let spinner_handle = thread::spawn(move || {
        spinner(message_receiver_mutex);
    });

    generator_handler.join().expect("Error generating data");
    spinner_handle.join().expect("Spinner error");

    println!("Generated {num} fake profiles.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::create_dir_all;
    use tempfile::tempdir;

    #[test]
    fn test_convert_unicode_to_ascii() {
        let name_list = [
            ("São Paulo", "Sao Paulo"),
            ("İstanbul", "Istanbul"),
            ("Mahārāshtra", "Maharashtra"),
            ("Středočeský Kraj", "Stredocesky Kraj"),
            ("Dar‘ā", "Dara"),
            ("Île-de-France", "Ile-de-France"),
        ];
        for (unicode, ascii) in name_list.iter() {
            assert_eq!(super::convert_unicode_to_ascii(unicode), *ascii);
        }
    }

    #[test]
    fn test_read_cities() {
        let path = Path::new("../data/worldcities.csv");
        let cities = super::read_cities(path);
        assert_eq!(cities.len(), 44691);
    }

    #[test]
    fn test_construct_person() {
        let age_lower = 18;
        let age_upper = 65;
        let path = Path::new("../data/worldcities.csv");
        let locations = super::read_cities(path);
        let person = super::construct_person(&locations, 1).unwrap();
        assert!(person.id > 0);
        assert!(person.name.split(' ').count() > 1);
        assert!(age_lower <= person.age && person.age <= age_upper);
    }

    #[test]
    fn test_generate_data() {
        let dir = tempdir().unwrap().path().to_path_buf();
        let output_path = dir.join("persons.csv");
        let (message_sender, message_receiver) = mpsc::channel();
        let message_receiver_mutex = Arc::new(Mutex::new(message_receiver));
        create_dir_all(&dir).unwrap();
        let path_clone = output_path.clone();
        let result = thread::spawn(move || {
            generate_data(message_sender, 10, &path_clone).unwrap();
        });

        result.join().unwrap();
        let received_message = message_receiver_mutex.lock().unwrap().recv().unwrap();
        assert_eq!(received_message, "done");
        assert!(output_path.exists());
    }
}
