use sqlx::sqlite::SqlitePool;

use anyhow::Result;
use serde::{Deserialize};
use structopt::StructOpt;

use async_std::fs::{File, read_dir};
use async_std::path::PathBuf;
use async_std::task;
use async_std::prelude::*;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "d", long = "directory")]
    directory: String,
}

#[derive(Deserialize, Debug)]
struct Location {
    url: String,
    value: i64,
}

#[derive(Deserialize, Debug)]
struct Highlight {
    text: String,
    #[serde(rename(deserialize = "isNoteOnly"))]
    is_note_only: bool,
    location: Location,
    note: Option<String>,
}

#[derive(Deserialize, Debug)]
struct BookHighlights {
    asin: String,
    title: String,
    authors: String,
    highlights: Vec<Highlight>,
}

#[async_std::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let pool = SqlitePool::connect("highlights.db").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let mut handles = Vec::new();
    let mut entries = read_dir(opt.directory).await?;
    while let Some(res) = entries.next().await {
        let entry = res?;
        let poold = pool.clone();
        let res = task::spawn(async move {
            process_file(&poold, &entry.path()).await;
        });
        handles.push(res);
    }

    for handle in handles {
        handle.await;
    }

    Ok(())
}

async fn process_file(pool: &SqlitePool, filepath: &PathBuf) -> Result<i64> {
    println!("{}", filepath.display());

    let mut file = File::open(filepath).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let mut de = serde_json::Deserializer::from_slice(&buffer);
    let o = BookHighlights::deserialize(&mut de).unwrap();

    add_book(&pool, &o).await
}

async fn add_book(pool: &SqlitePool, bh: &BookHighlights) -> Result<i64> {
    let mut conn = pool.acquire().await?;

    let book_id = sqlx::query!(
        r#"
INSERT INTO book ( asin, title, authors )
VALUES ( ?1, ?2, ?3 )
        "#,
       bh.asin,
       bh.title,
       bh.authors
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    for hl in &bh.highlights {
        sqlx::query!(
            r#"
    INSERT INTO highlight ( book_id, text, note, location)
    VALUES ( ?1, ?2, ?3, ?4 )
            "#,
           book_id,
           hl.text,
           hl.note,
           hl.location.value
        )
        .execute(&mut conn)
        .await?;
    }

    Ok(book_id)
}
