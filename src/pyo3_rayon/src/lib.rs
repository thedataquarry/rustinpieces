use pyo3::prelude::*;
use rayon::prelude::*;
use regex::{Captures, Regex};
use std::error::Error;

#[derive(FromPyObject)]
struct Record {
    id: u32,
    content: String,
}

struct RecordProcessed {
    id: u32,
    n_m: u64,
    n_f: u64,
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

fn count_gendered_pronouns(text: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let clean_text = clean_text(text);
    let tokens = clean_text.split_whitespace().collect::<Vec<&str>>();
    let n_m = tokens
        .par_iter()
        .filter(|&x| *x == "he" || *x == "him" || *x == "his")
        .count();
    let n_f = tokens
        .par_iter()
        .filter(|&x| *x == "she" || *x == "her" || *x == "hers")
        .count();
    Ok((n_m, n_f))
}

#[pyfunction(signature = (records))]
fn get_pronoun_counts(records: Vec<Record>) -> PyResult<Vec<RecordProcessed>> {
    let mut result = vec![];
    for record in records {
        let (n_m, n_f) = count_gendered_pronouns(&record.content).unwrap();
        let record_processed = RecordProcessed {
            id: record.id,
            n_m: n_m as u64,
            n_f: n_f as u64,
        };
        result.push(record_processed);
    }
    Ok(result)
}

#[pymodule]
fn pyo3_rayon(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_pronoun_counts, m)?)?;
    Ok(())
}
