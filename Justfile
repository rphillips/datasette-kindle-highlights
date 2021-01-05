run: generate
  datasette highlights.db -h 0.0.0.0 --metadata metadata.json --template-dir=templates/  --plugins-dir=plugins/

clean:
  rm -f highlights.db

generate: clean
  cargo run --release -- --directory=data
