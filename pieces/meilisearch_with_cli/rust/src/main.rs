mod cli;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use colored::*;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use meilisearch_sdk::{client::Client, search::SearchResults, settings::Settings};
use serde::{Deserialize, Serialize};

use crate::cli::{Args, Command};

// Create the client
lazy_static! {
    static ref CLIENT: Client = Client::new("http://127.0.0.1:7700", Some("apiKey"));
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Wine {
    id: usize,
    title: String,
    description: String,
    taster_name: Option<String>,
    designation: Option<String>,
    variety: Option<String>,
    country: Option<String>,
    winery: Option<String>,
    points: Option<String>,
    taster_twitter_handle: Option<String>,
    price: Option<f32>,
    region_1: Option<String>,
    region_2: Option<String>,
    province: Option<String>,
}

async fn create_index(index_name: &str) -> Result<()> {
    let index_task = CLIENT.create_index(index_name, Some("id")).await?;
    index_task.wait_for_completion(&CLIENT, None, None).await?;
    let index = CLIENT.index(index_name);
    let ranking_rules = vec![
        "sort",
        "words",
        "typo",
        "proximity",
        "attribute",
        "exactness",
    ];
    let sortable_attributes = vec!["title", "country"];
    let searchable_attributes = vec![
        "title",
        "description",
        "taster_name",
        "designation",
        "variety",
        "province",
        "country",
        "winery",
    ];
    let settings = Settings::new()
        .with_ranking_rules(ranking_rules)
        .with_sortable_attributes(sortable_attributes)
        .with_searchable_attributes(searchable_attributes);
    let settings_task = index.set_settings(&settings).await?;
    settings_task
        .wait_for_completion(&CLIENT, None, None)
        .await?;

    Ok(())
}

async fn index_data(data_path: &Option<PathBuf>, index_name: &str, wait: bool) -> Result<()> {
    let file_path = match data_path {
        Some(p) => p,
        None => Path::new("../data/winemag-data-130k-v2.jsonl.gz"),
    };
    let mut documents = Vec::new();
    let wine_file = File::open(file_path)?;
    let gz = GzDecoder::new(wine_file);
    let reader = io::BufReader::new(gz);

    for line in reader.lines() {
        let json: serde_json::Value = serde_json::from_str(&line?)?;
        documents.push(json);
    }

    let tasks = CLIENT
        .index(index_name)
        .add_documents_in_batches(&documents, None, Some("id"))
        .await?;

    if wait {
        for task in tasks {
            task.wait_for_completion(&CLIENT, None, None).await?;
        }
    }

    Ok(())
}

async fn search(
    query: &str,
    limit: usize,
    sort: Option<Vec<String>>,
    index_name: &str,
) -> Result<SearchResults<Wine>> {
    let index = CLIENT.index(index_name);
    let result = match sort {
        Some(s) => {
            let str_sort = s
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .to_owned();
            index
                .search()
                .with_query(query)
                .with_limit(limit)
                .with_sort(&str_sort)
                .execute::<Wine>()
                .await?
        }
        None => {
            index
                .search()
                .with_query(query)
                .with_limit(limit)
                .execute::<Wine>()
                .await?
        }
    };

    Ok(result)
}

#[async_std::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Command::CreateIndex { index_name } => {
            let name = match &index_name {
                Some(n) => n,
                None => "wine",
            };
            match create_index(name).await {
                Ok(_) => {
                    let success = format!("Index {name} successfully created");
                    println!("{}", success.green());
                }
                Err(e) => {
                    let error = format!("Error creating index: {e}");
                    eprintln!("{}", error.red());
                    exit(1);
                }
            }
        }
        Command::IndexData {
            data_path,
            index_name,
            wait,
        } => {
            let name = match &index_name {
                Some(n) => n,
                None => "wine",
            };
            let index_result: Result<()>;
            if let Ok(progress_style) = ProgressStyle::with_template("{spinner:.green} {msg}") {
                let pb = ProgressBar::new_spinner();
                pb.enable_steady_tick(Duration::from_millis(80));
                pb.set_style(
                    progress_style.tick_strings(&["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"]),
                );
                pb.set_message("Indexing data...");
                index_result = index_data(&data_path, name, wait).await;
                pb.finish_and_clear();
            } else {
                index_result = index_data(&data_path, name, wait).await;
            }

            match index_result {
                Ok(_) => println!("{}", "Indexing complete".green()),
                Err(e) => {
                    let error = format!("Error indexing dataa: {e}");
                    eprintln!("{}", error.red());
                    exit(1);
                }
            }
        }
        Command::Search {
            query,
            limit,
            sort,
            index_name,
        } => {
            let name = match &index_name {
                Some(n) => n,
                None => "wine",
            };

            match search(&query, limit.unwrap_or(20), sort, name).await {
                Ok(s) => {
                    let hits: Vec<Wine> = s.hits.iter().map(|x| x.result.clone()).collect();
                    if let Ok(pretty) = serde_json::to_string_pretty(&hits) {
                        println!("{pretty}")
                    } else {
                        eprintln!("Error processing results");
                        exit(1)
                    }
                }
                Err(e) => {
                    eprintln!("Error searching: {e}");
                    exit(1)
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{write::GzEncoder, Compression};
    use std::{fs::create_dir_all, io::Write};
    use tempfile::tempdir;
    use uuid::Uuid;

    fn wine_data() -> Vec<Wine> {
        vec![
            Wine{
                id: 1,
                title: "Nicosia 2013 Vulkà Bianco  (Etna)".to_string(),
                description: "Aromas include tropical fruit, broom, brimstone and dried herb. The palate isn't overly expressive, offering unripened apple, citrus and dried sage alongside brisk acidity.".to_string(),
                taster_name: Some("Kerin O’Keefe".to_string()),
                designation: Some("Vulkà Bianco".to_string()),
                variety: Some("White Blend".to_string()),
                country: Some("Italy".to_string()),
                winery: Some("Nicosia".to_string()),
                points: Some("87".to_string()),
                taster_twitter_handle: Some("@kerinokeefe".to_string()),
                price: None,
                region_1: Some("Etna".to_string()),
                region_2: None,
                province: Some("Sicily & Sardinia".to_string()),
            },

            Wine{
                id: 2,
                title: "Quinta dos Avidagos 2011 Avidagos Red (Douro)".to_string(),
                description: "This is ripe and fruity, a wine that is smooth while still structured. Firm tannins are filled out with juicy red berry fruits and freshened with acidity. It's  already drinkable, although it will certainly be better from 2016.".to_string(),
                taster_name: Some("Roger Voss".to_string()),
                designation: Some("Avidagos".to_string()),
                variety: Some("Portuguese Red".to_string()),
                country: Some("Portugal".to_string()),
                winery: Some("Quinta dos Avidagos".to_string()),
                points: Some("87".to_string()),
                taster_twitter_handle: Some("@vossroger".to_string()),
                price: Some(15.0),
                region_1: None,
                region_2: None,
                province: Some("Douro".to_string()),
            },

            Wine{
                id: 3,
                title: "Rainstorm 2013 Pinot Gris (Willamette Valley)".to_string(),
                description: "Tart and snappy, the flavors of lime flesh and rind dominate. Some green pineapple pokes through, with crisp acidity underscoring the flavors. The wine was all stainless-steel fermented.".to_string(),
                taster_name: Some("Paul Gregutt".to_string()),
                designation: None,
                variety: Some("Pinot Gris".to_string()),
                country: Some("US".to_string()),
                winery: Some("Rainstorm".to_string()),
                points: Some("87".to_string()),
                taster_twitter_handle: Some("@paulgwine\\xa0".to_string()),
                price: Some(14.0),
                region_1: Some("Willamette Valley".to_string()),
                region_2: Some("Willamette Valley".to_string()),
                province: Some("Oregon".to_string()),
            },

            Wine{
                id: 4,
                title: "St. Julian 2013 Reserve Late Harvest Riesling (Lake Michigan Shore)".to_string(),
                description: "Pineapple rind, lemon pith and orange blossom start off the aromas. The palate is a bit more opulent, with notes of honey-drizzled guava and mango giving way to a slightly astringent, semidry finish.".to_string(),
                taster_name: Some("Alexander Peartree".to_string()),
                designation: Some("Reserve Late Harvest".to_string()),
                variety: Some("Riesling".to_string()),
                country: Some("US".to_string()),
                winery: Some("St. Julian".to_string()),
                points: Some("87".to_string()),
                taster_twitter_handle: None,
                price: Some(13.0),
                region_1: Some("Lake Michigan Shore".to_string()),
                region_2: None,
                province: Some("Michigan".to_string()),
            },

            Wine{
                id: 5,
                title: "Sweet Cheeks 2012 Vintner's Reserve Wild Child Block Pinot Noir (Willamette Valley)".to_string(),
                description: "Much like the regular bottling from 2012, this comes across as rather rough and tannic, with rustic, earthy, herbal characteristics. Nonetheless, if you think of it as a pleasantly unfussy country wine, it's a good companion to a hearty winter stew.".to_string(),
                taster_name: Some("Paul Gregutt".to_string()),
                designation: Some("Vintner's Reserve Wild Child Block".to_string()),
                variety: Some("Pinot Noir".to_string()),
                country: Some("US".to_string()),
                winery: Some("Sweet Cheeks".to_string()),
                points: Some("87".to_string()),
                taster_twitter_handle: Some("@paulgwine\\xa0".to_string()),
                price: Some(65.0),
                region_1: Some("Willamette Valley".to_string()),
                region_2: Some("Willamette Valley".to_string()),
                province: Some("Oregon".to_string()),
            },
        ]
    }

    #[async_std::test]
    async fn test_create_index() {
        let index_name = &Uuid::new_v4().to_string();
        create_index(&index_name.to_string()).await.unwrap();

        let index = CLIENT.get_index(index_name).await.unwrap();
        let searchable_attributes = index.get_searchable_attributes().await.unwrap();
        assert_eq!(
            searchable_attributes,
            vec!(
                "title",
                "description",
                "taster_name",
                "designation",
                "variety",
                "province",
                "country",
                "winery",
            )
        );

        CLIENT.delete_index(index_name).await.unwrap();
    }

    #[async_std::test]
    async fn test_index_data() {
        let index_name = &Uuid::new_v4().to_string();
        let dir = tempdir().unwrap().path().to_path_buf();
        create_dir_all(&dir).unwrap();
        assert!(dir.exists());
        let data_path = dir.join("data.jsonl.gz");
        let jsonl_file = File::create(&data_path).unwrap();
        let mut encoder = GzEncoder::new(jsonl_file, Compression::default());

        for wine in wine_data() {
            let json_line = serde_json::to_string(&wine).unwrap();
            encoder.write_all(json_line.as_bytes()).unwrap();
            encoder.write_all(b"\n").unwrap(); // Add a line break before adding the next value
        }

        encoder.finish().unwrap();
        assert!(data_path.exists());
        index_data(&Some(data_path), index_name, true)
            .await
            .unwrap();
        let index = CLIENT.index(index_name);
        let documents = index.get_documents::<Wine>().await.unwrap();

        assert_eq!(documents.total, 5);
        CLIENT.delete_index(index_name).await.unwrap();
    }

    #[async_std::test]
    async fn test_search() {
        let index_name = &Uuid::new_v4().to_string();
        let index = CLIENT.index(index_name);
        index
            .add_documents::<Wine>(&wine_data(), Some("id"))
            .await
            .unwrap()
            .wait_for_completion(&CLIENT, None, None)
            .await
            .unwrap();
        let result = search("Nicosa", 20, None, &index_name.to_string())
            .await
            .unwrap();
        let title = "Nicosia 2013 Vulkà Bianco  (Etna)";
        let contains_title = result.hits.iter().any(|hit| hit.result.title == title);

        assert!(contains_title);
        CLIENT.delete_index(index_name).await.unwrap();
    }
}
