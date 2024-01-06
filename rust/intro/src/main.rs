use chrono::prelude::*;
use std::fmt;

// --- Structs and implementations ---

struct Person {
    name: String,
    age: u8,
}

// Implementations

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

// Traits

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is {} years old", self.name, self.age)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person: {}, {}", self.name, self.age)
    }
}

// Custom functions

fn approx_year_of_birth(person: &Person) -> u16 {
    let now = chrono::Utc::now();
    let year = now.year() - (person.age as i32);
    year as u16
}

// --- Run functions ---

// 1. Trait implementations
fn run1() {
    let p = Person::new("Megan", 28);
    println!("{}", p);
    println!("{:?}", p);
    /*
    Megan is 28 years old
    Person: Megan, 28
    */
}

// 2. Enumerate
fn run2() {
    let persons = vec![Person::new("James", 33), Person::new("Salima", 31)];
    for (i, p) in persons.iter().enumerate() {
        println!("Person {}: {}", i, p)
    }
    /*
    Person 0: James is 33 years old
    Person 1: Salima is 31 years old
    */
}

// 3. Zip
fn run3() {
    let names = ["Alice", "Charlie"];
    let ages = [24, 45];
    let mut persons = vec![];
    for (name, age) in names.iter().zip(ages.iter()) {
        persons.push(Person::new(name, *age));
    }
    println!("{:?}", persons);
    /*
    [Person: Alice, 24, Person: Charlie, 45]
    */
}

// 4. Tuple unpacking and indexing
fn run4() {
    let sorted_ages: (u8, u8, u8) = (18, 41, 65);
    let (youngest, _, oldest) = sorted_ages;
    println!("Youngest age: {}, oldest age: {}", youngest, oldest);
    println!("Middle age: {}", sorted_ages.1);
    /*
    Youngest age: 18, oldest age: 65
    Middle age: 41
    */
}

// 5. Anonymous functions (closures)
fn run5() {
    let mut persons = vec![Person::new("Aiko", 41), Person::new("Rohan", 18)];
    // Sort by age
    persons.sort_by_key(|p| p.age);
    let youngest_person = persons.first().unwrap();
    println!(
        "{} is the youngest person at {} years old",
        youngest_person.name, youngest_person.age
    );
    /*
    Rohan is the youngest person at 18 years old
    */
}

// 6. Single-line if-else
fn run6() {
    let persons = vec![Person::new("Josephine", 20), Person::new("Wesley", 31)];
    for person in persons {
        // check if person is born in a leap year using simplistic leap year logic
        let birth_year = approx_year_of_birth(&person);
        let person_is_born_in_leap_year = birth_year % 4 == 0;
        println!(
            "{}. Born in a leap year?: {}",
            person, person_is_born_in_leap_year
        );
    }
    /*
    Josephine is 20 years old. Born in a leap year?: true
    Wesley is 31 years old. Born in a leap year?: false
    */
}

// 7. Map and filter
fn run7() {
    let persons = vec![Person::new("Issa", 39), Person::new("Ibrahim", 26)];
    let result = persons
        .into_iter()
        .filter(|p| approx_year_of_birth(p) > 1995)
        .map(|p| (p.name, p.age))
        .collect::<Vec<(String, u8)>>();
    println!("Persons born after 1995: {:?}", result)
    /*
    Persons born after 1995: [("Ibrahim", 26)]
    */
}

fn main() {
    run1();
    run2();
    run3();
    run4();
    run5();
    run6();
    run7();
}
