# datasette-kindle-highlights - freedom of highlights

![Rust](https://github.com/rphillips/datasette-kindle-highlights/workflows/Rust/badge.svg) ![Clippy](https://github.com/rphillips/datasette-kindle-highlights/workflows/Clippy/badge.svg)

datasette-kindle-highlights imports Kindle highlights and notes from [Bookcision](https://readwise.io/bookcision) into a SQLite database. This project also leverages Simon Wilson's [datasette](https://github.com/simonw/datasette) project to export the resulting database as a website. 

## Screenshots

![](https://raw.githubusercontent.com/rphillips/datasette-kindle-highlights/screenshots/screenshot1.png)

## Docker

Create the database from a directory of json files under `data`:

```
kindle_highlights --directory=data
```

Run the docker image:

```
docker run -p 8001:8001 -v $PWD/highlights.db:/data/highlights.db rphillips/datasette-kindle-highlights
```

Browse to http://localhost:8001/

## Status

This project is just at the very beginning stages. I welcome anyone to help out. I would like to add the following features:

- [x] Add search functionality
- [ ] Wire up an automated, github action, Docker image that contains both datasette and the kindle_highlights application
- [ ] Add more documentation on how to run the application, datasette, and the docker image
- [ ] Add better 'note' support to the datasette website

## License

datasette-kindle-highlights is licensed under the Apache 2.0 license. For more information, please see the LICENSE-2.0 file.
