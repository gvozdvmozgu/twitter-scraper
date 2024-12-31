use clap::Parser;

use crate::scraper::SearchMode;

#[derive(Parser)]
pub(crate) enum Options {
    Tweets {
        query: String,
        #[clap(long, default_value_t = SearchMode::Top)]
        search_mode: SearchMode,
        #[clap(long, default_value_t = 10)]
        count: u32,
        #[clap(long)]
        cursor: Option<String>,
    },
    Profiles {
        query: String,
        #[clap(long, default_value_t = 10)]
        count: u32,
        #[clap(long)]
        cursor: Option<String>,
    },
}

pub(crate) fn from_args() -> Options {
    Options::parse()
}
