CREATE TABLE IF NOT EXISTS book
(
    id          INTEGER PRIMARY KEY NOT NULL,
    asin        TEXT                NOT NULL,
    title       TEXT                NOT NULL,
    authors     TEXT                NOT NULL
);
CREATE TABLE IF NOT EXISTS highlight
(
    id          INTEGER PRIMARY KEY NOT NULL,
    book_id     INTEGER             NOT NULL,
    text        TEXT                NOT NULL,
    note        TEXT                ,
    location    INTEGER             NOT NULL,
    FOREIGN KEY(book_id)            REFERENCES book(id)
);
