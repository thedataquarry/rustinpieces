use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use fake::{faker::name::raw::Name, locales::EN, Fake, Rng};
use pyo3::{
    create_exception,
    exceptions::{PyException, PyValueError},
    prelude::*,
};
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

create_exception!(_pyo3_mock_data, FileNotFoundError, PyException);

#[derive(Debug)]
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

fn check_valid_file(filename: &PathBuf) -> PyResult<()> {
    if !filename.is_file() {
        return Err(FileNotFoundError::new_err(format!(
            "File not found: {:?}",
            filename
        )));
    }

    Ok(())
}

fn construct_person(locations: &[Location], id: u32) -> PyResult<Person> {
    let mut r = StdRng::seed_from_u64(id as u64);
    match locations.iter().choose(&mut r) {
        Some(loc) => {
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
        None => Err(PyValueError::new_err("No location found")),
    }
}

fn convert_unicode_to_ascii(s: &str) -> String {
    // First ensure that the unicode string is normalized to NFKD form
    s.nfkd().filter(|c| c.is_ascii()).collect::<String>()
}

fn read_cities(path: &Path) -> PyResult<Vec<Location>> {
    let rdr = csv::Reader::from_path(path);
    match rdr {
        Ok(mut r) => {
            let mut cities: Vec<Location> = Vec::new();
            for record in r.records() {
                match record {
                    Ok(re) => {
                        let city = &re[1];
                        let state = convert_unicode_to_ascii(&re[7]);
                        let country = &re[4];
                        cities.push(Location {
                            city: city.to_string(),
                            state: state.to_string(),
                            country: country.to_string(),
                        });
                    }
                    Err(_) => {
                        return Err(PyValueError::new_err(
                            "Did not obtain valid record from CSV file.",
                        ))
                    }
                }
            }

            Ok(cities)
        }
        Err(_) => Err(PyValueError::new_err(
            "Cannot read cities CSV file. Please check the path.",
        )),
    }
}

fn run(locations: &[Location], limit: u32, output_filename: &Option<PathBuf>) -> PyResult<()> {
    let mut profiles: Vec<Person> = Vec::new();
    for id in 1..=limit {
        let person = construct_person(locations, id)?;
        profiles.push(person);
    }
    let mut wtr = csv::Writer::from_writer(vec![]);
    // Serialize the data to CSV and write it to file
    for person in profiles.iter() {
        if wtr.serialize(person).is_err() {
            return Err(PyValueError::new_err("Unable to serialize output CSV"));
        }
    }
    let result = wtr.into_inner();
    match result {
        Ok(r) => {
            let output_path = match output_filename {
                Some(p) => p,
                None => Path::new("./data/persons.csv"),
            };
            write_csv(output_path, r)?;
            Ok(())
        }
        Err(_) => Err(PyValueError::new_err("Unable to construct CSV output")),
    }
}

fn write_csv(output_path: &Path, result: Vec<u8>) -> PyResult<()> {
    match fs::File::create(output_path) {
        Ok(mut file) => {
            if file.write_all(result.as_slice()).is_err() {
                return Err(PyValueError::new_err("Unable to write to output CSV file"));
            }
            Ok(())
        }
        Err(_) => Err(PyValueError::new_err("Unable to create file for writer")),
    }
}

#[pyfunction(signature = (filename, limit=10, output_filename=None))]
fn generate_fake_persons(
    filename: PathBuf,
    limit: u32,
    output_filename: Option<PathBuf>,
) -> PyResult<()> {
    check_valid_file(&filename)?;
    let locations = read_cities(&filename)?;
    println!("Generating {:?} person profiles.", limit);
    run(&locations, limit, &output_filename)?;

    Ok(())
}

#[pymodule]
fn _pyo3_mock_data(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_fake_persons, m)?)?;
    m.add("FileNotFoundError", py.get_type::<FileNotFoundError>())?;

    Ok(())
}
