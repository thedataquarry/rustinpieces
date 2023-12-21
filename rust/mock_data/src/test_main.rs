#![cfg(test)]

use std::path::Path;

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
    let path = Path::new("data/worldcities.csv");
    let cities = super::read_cities(&path);
    assert_eq!(cities.len(), 44691);
}

#[test]
fn test_construct_person() {
    let age_lower = 18;
    let age_upper = 65;
    let path = Path::new("data/worldcities.csv");
    let locations = super::read_cities(&path);
    let person = super::construct_person(&locations, 1);
    assert!(person.id > 0);
    assert!(person.name.split(" ").count() > 1);
    assert!(age_lower <= person.age && person.age <= age_upper);
}