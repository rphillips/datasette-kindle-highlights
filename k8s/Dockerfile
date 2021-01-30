FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.49 as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo run --release -- --directory=data

## frontend

FROM datasetteproject/datasette:0.53 as frontend
VOLUME /data

RUN mkdir /app

ADD metadata.json /app
COPY --from=builder /app/highlights.db /data/highlights.db
COPY templates /app/templates
COPY plugins /app/plugins

RUN pip install datasette-template-sql

ENTRYPOINT datasette --metadata /app/metadata.json -p 8001 -h 0.0.0.0 --template-dir=/app/templates/ --plugins-dir=/app/plugins/ /data/highlights.db
