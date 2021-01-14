//! # kindle_highlights
//!
//! `kindle_highlights` is an application to store Bookcision exports of Kindle
//! highlights into SQLite.

use anyhow::Result;
use rusqlite::{params, Connection, OpenFlags, NO_PARAMS};
use serde::Deserialize;
use std::fs::{File, read_dir};
use std::io::BufReader;
use std::path::PathBuf;

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

/// Default SQLite Book Schema
pub static DEFAULT_SCHEMA_BOOK: &str = r#"
CREATE TABLE IF NOT EXISTS book
(
    id          INTEGER PRIMARY KEY NOT NULL,
    asin        TEXT                NOT NULL,
    title       TEXT                NOT NULL,
    authors     TEXT                NOT NULL
)
"#;

/// Default SQLite Highlight Schema
pub static DEFAULT_SCHEMA_HIGHLIGHT: &str = r#"
CREATE TABLE IF NOT EXISTS highlight
(
    id          INTEGER PRIMARY KEY NOT NULL,
    book_id     INTEGER             NOT NULL,
    text        TEXT                NOT NULL,
    note        TEXT                ,
    location    INTEGER             NOT NULL,
    FOREIGN KEY(book_id)            REFERENCES book(id)
)
"#;

/// Default SQLite Full Text Search Schema
pub static DEFAULT_SCHEMA_HIGHLIGHT_FTS: &str = r#"
CREATE VIRTUAL TABLE "highlight_fts" USING FTS4
(
    name,
    text,
    content="highlight"
)
"#;

fn add_book(db: &Connection, bh: &BookHighlights) -> Result<i64> {
    db.execute(
        r#"
INSERT INTO book ( asin, title, authors )
VALUES ( ?1, ?2, ?3 )
        "#,
        params![bh.asin, bh.title, bh.authors],
    )?;
    let book_id = db.last_insert_rowid();
    add_highlights(book_id, db, bh)?;
    Ok(book_id)
}

fn add_highlights(book_id: i64, db: &Connection, bh: &BookHighlights) -> Result<()> {
    db.execute_batch("BEGIN TRANSACTION;")?;
    let mut stmt = db.prepare(
        r#"
    INSERT INTO highlight ( book_id, text, note, location)
    VALUES ( ?1, ?2, ?3, ?4 )
        "#,
    )?;
    for hl in &bh.highlights {
        stmt.execute(params![book_id, hl.text, hl.note, hl.location.value])?;
    }
    db.execute_batch("COMMIT TRANSACTION;")?;
    Ok(())
}

pub fn init_db(db_path: &String) -> Result<Connection> {
    let db = Connection::open_with_flags(db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    db.execute(DEFAULT_SCHEMA_BOOK, NO_PARAMS)?;
    db.execute(DEFAULT_SCHEMA_HIGHLIGHT, NO_PARAMS)?;
    db.execute(DEFAULT_SCHEMA_HIGHLIGHT_FTS, NO_PARAMS)?;
    Ok(db)
}

pub fn process_directory(db: &Connection, dir: &String) -> Result<()> {
    for res in read_dir(dir)? {
        let entry = res?;
        let path = entry.path();
        match process_file(&db, &path) {
            Ok(id) => println!("id={:?} path={:?} success", id, path.display()),
            Err(err) => println!("{:?} error: {:?}", err, path.display()),
        };
    }
    finalize_search(&db)
}

/// Process a json file from Bookcision
pub fn process_file(pool: &Connection, filepath: &PathBuf) -> Result<i64> {
    let file = File::open(filepath)?;
    let buf_reader = BufReader::new(file);
    let mut de = serde_json::Deserializer::from_reader(buf_reader);
    let bh = BookHighlights::deserialize(&mut de)?;
    add_book(&pool, &bh)
}

/// Finalize the full text search database
pub fn finalize_search(db: &Connection) -> Result<()> {
    db.execute(
        r#"
    INSERT INTO "highlight_fts" (rowid, text)
    SELECT rowid, text FROM highlight
        "#,
        NO_PARAMS,
    )?;
    Ok(())
}
