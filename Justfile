run: clean generate
  datasette highlights.db --metadata metadata.json --template-dir=templates/ 

clean:
  rm -f highlights.db

generate:
  sqlx db create
  sqlx migrate run
  cargo run --release -- --directory=data
  
