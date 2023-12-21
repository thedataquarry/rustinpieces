use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

mod test_main;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    age: u16,
    is_married: bool,
    city: String,
    state: String,
    country: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PersonFinal {
    id: u32,
    name: String,
    age: u16,
    is_married: bool,
    city: String,
    state: String,
    country: String,
}

fn read_csv(input_path: &Path) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(input_path).expect("Unable to read from CSV");
    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let data: Vec<Person> = reader.deserialize().collect::<Result<_, _>>()?;
    Ok(data)
}

fn construct_person_obj(persons: Vec<Person>) -> Vec<PersonFinal> {
    let mut persons_modified: Vec<PersonFinal> = Vec::new();
    for (id, person) in persons.iter().enumerate() {
        let id = id as u32 + 1;
        let person_with_id = PersonFinal {
            id,
            name: person.name.to_string(),
            age: person.age,
            is_married: person.is_married,
            city: person.city.to_string(),
            state: person.state.to_string(),
            country: person.country.to_string(),
        };
        persons_modified.push(person_with_id);
    }
    persons_modified
}

fn write_csv(output_path: &Path, result: Vec<u8>) {
    let mut file = fs::File::create(output_path).expect("Unable to create file for writer");
    file.write_all(result.as_slice())
        .expect("Unable to write to output CSV file");
}

fn main() {
    let input_path = Path::new("./data/persons.csv");
    let data = read_csv(input_path).expect("Unable to read/open CSV");
    let persons_modified = construct_person_obj(data);
    let mut wtr = csv::Writer::from_writer(vec![]);
    // Serialize the data to CSV and write it to file
    for person in persons_modified.iter() {
        wtr.serialize(person)
            .expect("Unable to serialize output CSV");
    }
    let result = wtr.into_inner().expect("Unable to construct CSV output");
    let output_path = Path::new("./data/persons_modified.csv");
    write_csv(output_path, result);
}
