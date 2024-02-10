use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create the wine index and update settings
    CreateIndex {
        #[clap(short, long, help = "The name to use for the index. Default: wine")]
        index_name: Option<String>,
    },

    /// Index the wine data
    IndexData {
        #[clap(short, long, help = "Index the wine data")]
        data_path: Option<PathBuf>,

        #[clap(short, long, help = "The name to use for the index. Default: wine")]
        index_name: Option<String>,

        #[clap(short, long, action, help = "Wait for the data to finish indexing")]
        wait: bool,
    },

    /// Preform a search
    Search {
        #[clap(help = "The search to preform")]
        query: String,

        #[clap(short, long, help = "Limit the number of search results")]
        limit: Option<usize>,

        #[clap(short, long, help = "Sort order for the results")]
        sort: Option<Vec<String>>,

        #[clap(short, long, help = "The name to use for the index. Default: wine")]
        index_name: Option<String>,
    },
}
