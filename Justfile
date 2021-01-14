venv := ".venv"
python := venv + "/bin/python"
pip := venv + "/bin/pip"
datasette := venv + "/bin/datasette"
version := `cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version'`
container_name := "rphillips/datasette-kindle-highlights:" + version

all:
  just --list

# Run with datasette
run: create-venv generate-db
  {{datasette}} highlights.db --metadata metadata.json --template-dir=templates/  --plugins-dir=plugins/

# Run with docker
run-with-docker: generate-docker-image generate-db
  docker run -ti -p 8001:8001 -v $PWD/highlights.db:/data/highlights.db {{container_name}}

# Clean
clean: clean-db clean-rust

# Clean database
clean-db:
  rm -f highlights.db

# Clean database
clean-rust:
  cargo clean

# Build Debug
build-debug:
  cargo build

# Build Release
build:
  cargo build --release

# Generate the database using ./data directory
generate-db: clean-db
  cargo run --release -- --directory=data

# Generate docker image
generate-docker-image:
  docker build -t {{container_name}} .

# Create a dev venv if not exist
create-venv:
    #!/usr/bin/env bash
    if ! [ -d "./{{venv}}" ]; then
        echo "Creating a new development virtual env: {{venv}} ..."
        python -m venv {{venv}}
        echo "Installing libraries..."
        {{pip}} install -r ./requirements.txt
    fi

# Cleanup virtual environenment
cleanup-venv:
  rm -rf {{venv}}

# Update virtualenv
update-venv: cleanup-venv create-venv
