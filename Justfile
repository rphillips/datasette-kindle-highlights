export DATABASE_URL = "sqlite:highlights.db"

run: generate
  datasette highlights.db --metadata metadata.json --template-dir=templates/ 

clean:
  rm -f highlights.db

generate: clean
  sqlx db create
  sqlx migrate run
  cargo sqlx prepare
  cargo run --release -- --directory=data
  
