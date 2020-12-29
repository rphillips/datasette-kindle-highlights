run: generate
  datasette highlights.db --metadata metadata.json --template-dir=templates/ 

clean:
  rm -f highlights.db

generate: clean
  cargo run --release -- --directory=data
