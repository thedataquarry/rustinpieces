use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
enum AgeBracket {
    Child,
    Youth,
    Adult,
    Senior,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Person {
    id: Option<u32>,
    age: Option<u32>,
    is_married: Option<bool>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
    age_bracket: Option<AgeBracket>,
}

impl Person {
    fn set_age_bracket(&mut self) {
        self.age_bracket = match self.age {
            Some(a) => {
                if a < 13 {
                    Some(AgeBracket::Child)
                } else if (13..=17).contains(&a) {
                    Some(AgeBracket::Youth)
                } else if (18..=59).contains(&a) {
                    Some(AgeBracket::Adult)
                } else {
                    Some(AgeBracket::Senior)
                }
            }
            None => None,
        };
    }
}

fn load_csv(file_path: &Path) -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path).expect("Unable to read from CSV");
    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let mut data: Vec<Person> = reader.deserialize().collect::<Result<_, _>>()?;

    for d in &mut data {
        d.set_age_bracket();
    }

    Ok(data)
}

fn main() {
    let file_path = Path::new("data/persons.csv");
    let persons = load_csv(file_path).expect("Unable to read/open CSV");
    for person in persons {
        println!("{:?}", person);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age_bracket_none() {
        let mut person = Person {
            id: Some(1),
            age: None,
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, None);
    }

    #[test]
    fn test_age_bracket_child_min() {
        let mut person = Person {
            id: Some(1),
            age: Some(1),
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Child));
    }

    #[test]
    fn test_age_bracket_child_max() {
        let mut person = Person {
            id: Some(1),
            age: Some(12),
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Child));
    }

    #[test]
    fn test_age_bracket_youth_min() {
        let mut person = Person {
            id: Some(1),
            age: Some(13),
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Youth));
    }

    #[test]
    fn test_age_bracket_youth_max() {
        let mut person = Person {
            id: Some(1),
            age: Some(17),
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Youth));
    }

    #[test]
    fn test_age_bracket_adult_min() {
        let mut person = Person {
            id: Some(1),
            age: Some(18),
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Adult));
    }

    #[test]
    fn test_age_bracket_adult_max() {
        let mut person = Person {
            id: Some(1),
            age: Some(59),
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Adult));
    }

    #[test]
    fn test_age_bracket_senior() {
        let mut person = Person {
            id: Some(1),
            age: Some(60),
            is_married: Some(false),
            city: Some("city".to_string()),
            state: Some("state".to_string()),
            country: Some("country".to_string()),
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Senior));
    }

    #[test]
    fn test_construct_person_obj() {
        let file_path = Path::new("data/persons.csv");
        let persons = load_csv(file_path).unwrap();
        assert_eq!(persons.len(), 10);
        assert_eq!(persons[0].id, Some(1));
    }
}
