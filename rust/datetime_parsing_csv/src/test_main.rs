#![cfg(test)]

use std::path::Path;

#[test]
fn test_read_csv() {
    let input_path = Path::new("./data/persons.csv");
    let data = super::read_csv(input_path).expect("Unable to read/open CSV");
    assert_eq!(data.len(), 6);
}

#[test]
fn test_construct_person_obj() {
    let input_path = Path::new("./data/persons.csv");
    let data = super::read_csv(input_path).expect("Unable to read/open CSV");
    let persons_modified = super::construct_person_obj(data);
    assert_eq!(persons_modified.len(), 6);
    assert!(persons_modified[0].id > 0);
}

#[test]
fn test_write_csv() {
    let input_path = Path::new("./data/persons.csv");
    let persons = super::read_csv(input_path).expect("Unable to read/open CSV");
    let persons_modified = super::construct_person_obj(persons);
    let output_path = Path::new("./data/test_persons.csv");
    super::write_csv(persons_modified, output_path);
    assert!(output_path.exists());
    // Delete the file
    std::fs::remove_file(output_path).expect("Unable to delete output file");
}
