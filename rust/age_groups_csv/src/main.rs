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
    name: Option<String>,
    age: Option<u32>,
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
    fn test_age_bracket() {
        let people_list = [
            (
                Person {
                    id: Some(1),
                    name: Some("Arthur Dent".to_string()),
                    age: None,
                    age_bracket: None,
                },
                None,
            ),
            (
                Person {
                    id: Some(1),
                    name: Some("Arthur Dent".to_string()),
                    age: Some(1),
                    age_bracket: None,
                },
                Some(AgeBracket::Child),
            ),
        ];

        for (mut person, expected) in people_list.into_iter() {
            person.set_age_bracket();
            assert_eq!(person.age_bracket, expected);
        }
    }

    /* #[test]
    fn test_age_bracket_none() {
        let mut person = Person {
            id: Some(1),
            age: None,
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
            age_bracket: None,
        };
        person.set_age_bracket();

        assert_eq!(person.age_bracket, Some(AgeBracket::Senior));
    } */

    #[test]
    fn test_construct_person_obj() {
        let file_path = Path::new("data/persons.csv");
        let persons = load_csv(file_path).unwrap();
        assert_eq!(persons.len(), 10);
        assert_eq!(persons[0].id, Some(1));
    }
}
