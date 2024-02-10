use rayon::prelude::*;
use regex::{Captures, Regex};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Record {
    id: u32,
    publication: String,
    author: String,
    date: String,
    content: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct RecordProcessed {
    id: u32,
    publication: String,
    author: String,
    date: String,
    num_male_pronouns: usize,
    num_female_pronouns: usize,
}

fn load_csv(path: &Path) -> Result<Vec<Record>, csv::Error> {
    let contents = fs::read_to_string(path).expect("Unable to read from CSV");
    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let data: Vec<Record> = rdr.deserialize().collect::<Result<_, _>>().unwrap();
    Ok(data)
}

fn clean_text(text: &str) -> String {
    let pattern1 = Regex::new(r"([’'])(s|d|ll)").unwrap();
    // Replace pattern with text
    let matched = pattern1.replace_all(text, |capture: &Captures| match &capture[2] {
        "s" => " is",
        "d" => " had",
        "ll" => " will",
        _ => "<unk>",
    });
    // Remove non-alphabetic characters
    let pattern2 = Regex::new(r"[^a-zA-Z\s]").unwrap();
    let clean_text = pattern2.replace_all(&matched, "");
    let result: String = clean_text.to_lowercase();
    result
}

fn count_gendered_pronouns(tokens: Vec<&str>) -> (usize, usize) {
    let num_male_pronouns = tokens
        .par_iter()
        .filter(|&x| *x == "he" || *x == "him" || *x == "his")
        .count();
    let num_female_pronouns = tokens
        .par_iter()
        .filter(|&x| *x == "she" || *x == "her" || *x == "hers")
        .count();
    (num_male_pronouns, num_female_pronouns)
}

fn process_record(record: &Record) -> RecordProcessed {
    let text = &record.content;
    let result: String = clean_text(text);
    let tokens: Vec<&str> = result.split_whitespace().collect();
    let (n_m, n_f) = count_gendered_pronouns(tokens);
    RecordProcessed {
        id: record.id,
        publication: record.publication.to_string(),
        author: record.author.to_string(),
        date: record.date.to_string(),
        num_male_pronouns: n_m,
        num_female_pronouns: n_f,
    }
}

fn run(input_path: &PathBuf) {
    let data = load_csv(Path::new(input_path)).unwrap();
    let records = data.par_iter().map(process_record).collect::<Vec<_>>();
    let output_path = input_path
        .clone()
        .into_os_string()
        .into_string()
        .expect("Unable to convert path");
    let output_path = output_path.replace(".csv", "_processed.csv");
    let mut wtr = csv::Writer::from_path(Path::new(&output_path)).unwrap();
    _ = records.iter().map(|x| wtr.serialize(x)).collect::<Vec<_>>();
    println!(
        "Results for {:?} to written to {:?}",
        input_path, output_path
    );
}

fn main() {
    let paths = [
        PathBuf::from("../data/articles1.csv"),
        PathBuf::from("../data/articles2.csv"),
        PathBuf::from("../data/articles3.csv"),
    ];
    for path in paths {
        run(&path);
    }
}

// Tests

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_load_csv() {
        let path = Path::new("../data/articles1.csv");
        let data = load_csv(path).unwrap();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_clean_text() {
        let text = "He's he’s he’ll she'll he’d She'd HE SHE";
        let result = clean_text(text);
        assert_eq!(result, "he is he is he will she will he had she had he she");
    }

    #[test]
    fn test_process_record() {
        let record = Record {
            id: 1,
            publication: "The New York Times".to_string(),
            author: "Rob Williamson".to_string(),
            date: "2016-12-31".to_string(),
            content: "She's swum the fastest she's ever done in her life.".to_string(),
        };
        let result = process_record(&record);
        assert!(result.num_male_pronouns == 0);
        assert!(result.num_female_pronouns == 3);
    }

    #[test]
    fn test_count_gendered_pronouns() {
        let text = "He's he’ll he’d HE she's she'll She’d SHE random text here";
        let clean_text = clean_text(text);
        let tokens = clean_text.split_whitespace().collect();
        let (n_m, n_f) = count_gendered_pronouns(tokens);
        assert_eq!(n_m, 4);
        assert_eq!(n_f, 4);
    }
}
