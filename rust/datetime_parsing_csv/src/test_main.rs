#![cfg(test)]

use std::path::Path;

#[test]
fn test_read_csv() {
    let input_path = Path::new("./data/persons.csv");
    let data = super::read_csv(input_path).expect("Unable to read/open CSV");
    assert_eq!(data.len(), 6);
    // To do: Test that the `isMarried` key exists in each struct
}

#[test]
fn test_construct_person_obj() {
    let input_path = Path::new("./data/persons.csv");
    let data = super::read_csv(input_path).expect("Unable to read/open CSV");
    let persons_modified = super::construct_person_obj(data);
    assert_eq!(persons_modified.len(), 6);
    assert!(persons_modified[0].id > 0);
}
