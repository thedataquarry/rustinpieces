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

#[derive(Debug)]
#[allow(dead_code)]
struct DemographicCount {
    minors: usize,
    adults: usize,
}

impl DemographicCount {
    pub fn new(persons: &Vec<Person>) -> Self {
        let mut minors = 0;
        let mut adults = 0;

        for person in persons {
            if let Some(bracket) = &person.age_bracket {
                match bracket {
                    AgeBracket::Child => minors += 1,
                    AgeBracket::Youth => minors += 1,
                    AgeBracket::Adult => adults += 1,
                    AgeBracket::Senior => adults += 1,
                }
            }
        }

        DemographicCount { minors, adults }
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
    let file_path = Path::new("../data/persons.csv");
    let persons = load_csv(file_path).expect("Unable to read/open CSV");
    let demographics = DemographicCount::new(&persons);
    println!("{demographics:?}");
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

    #[test]
    fn test_construct_person_obj() {
        let file_path = Path::new("../data/persons.csv");
        let persons = load_csv(file_path).unwrap();
        assert_eq!(persons.len(), 10);
        assert_eq!(persons[0].id, Some(1));
    }

    #[test]
    fn test_calculate_demographics() {
        let persons = vec![
            Person {
                id: Some(1),
                name: Some("Arthur Dent".to_string()),
                age: Some(28),
                age_bracket: Some(AgeBracket::Adult),
            },
            Person {
                id: Some(2),
                name: Some("Ford Prefect".to_string()),
                age: Some(72),
                age_bracket: Some(AgeBracket::Senior),
            },
            Person {
                id: Some(3),
                name: Some("Trillian".to_string()),
                age: Some(28),
                age_bracket: Some(AgeBracket::Adult),
            },
            Person {
                id: Some(4),
                name: Some("Marvin".to_string()),
                age: Some(2),
                age_bracket: Some(AgeBracket::Child),
            },
            Person {
                id: Some(5),
                name: Some("Mr. Prosser".to_string()),
                age: Some(14),
                age_bracket: Some(AgeBracket::Youth),
            },
        ];
        let demographics = DemographicCount::new(&persons);

        assert_eq!(demographics.minors, 2);
        assert_eq!(demographics.adults, 3);
    }
}
