use sqlx::sqlite::SqlitePool;
use sqlx::Done;

use anyhow::Result;
use serde::{Deserialize};
use structopt::StructOpt;

use async_std::fs::File;
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
async fn main() -> Result<()>{
    let opt = Opt::from_args();

    let mut file = File::open("data/Kindle.Highlights_Sapiens.A.Brief.History.of.Humankind_1609083628903.json").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let mut de = serde_json::Deserializer::from_slice(&buffer);
    let o = BookHighlights::deserialize(&mut de).unwrap();

    let pool = SqlitePool::connect("highlights.db").await?;
    add_book(&pool, &o).await?;

    Ok(())
}

async fn add_book(pool: &SqlitePool, bh: &BookHighlights) -> anyhow::Result<i64> {
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
