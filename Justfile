run: generate
  datasette highlights.db --metadata metadata.json

clean:
  rm -f highlights.db

generate:
  sqlx db create
  sqlx migrate run
  cargo run -- --directory=data
  
