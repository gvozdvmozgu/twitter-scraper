use clap::Parser;

use crate::scraper::SearchMode;

#[derive(Parser)]
pub(crate) enum Options {
    Tweets {
        query: String,
        #[clap(long)]
        from: Option<String>,
        #[clap(long, default_value_t = SearchMode::Top)]
        search_mode: SearchMode,
        #[clap(long, default_value_t = 50)]
        count: u32,
        #[clap(long)]
        cursor: Option<String>,
        #[clap(long, default_value_t = Output::PrettyPrint)]
        output: Output,
        #[clap(long)]
        all: bool,
    },
    Profiles {
        query: String,
        #[clap(long, default_value_t = 10)]
        count: u32,
        #[clap(long)]
        cursor: Option<String>,
        #[clap(long, default_value_t = Output::PrettyPrint)]
        output: Output,
        #[clap(long)]
        all: bool,
    },
}

#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum Output {
    PrettyPrint,
    Json,
}

pub(crate) fn from_args() -> Options {
    Options::parse()
}
