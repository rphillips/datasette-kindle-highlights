#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::if_not_else, clippy::non_ascii_literal)]

use anyhow::Result;
use kindle_highlights::{process_directory, init_db};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "d", long = "directory")]
    directory: String,
    #[structopt(long = "database", default_value = "highlights.db")]
    db_path: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let db = init_db(&opt.db_path)?;
    process_directory(&db, &opt.directory)
}
