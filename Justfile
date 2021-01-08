venv := ".venv"
python := venv + "/bin/python"
pip := venv + "/bin/pip"
datasette := venv + "/bin/datasette"
container_name := "rphillips/datasette-kindle-highlights"

all:
  just --list

# Run with datasette
run: create-venv generate-db
  {{datasette}} highlights.db -h 0.0.0.0 --metadata metadata.json --template-dir=templates/  --plugins-dir=plugins/

# Run with docker
run-with-docker: generate-docker-image generate-db
  docker run -ti -p 8001:8001 -v $PWD/highlights.db:/data/highlights.db {{container_name}}

# Clean
clean: cleanup-venv
  rm -f highlights.db
  cargo clean

# Build Debug
build-debug:
  cargo build

# Build Release
build:
  cargo build --release

# Generate the database using ./data directory
generate-db: clean
  cargo run --release -- --directory=data

# Generate the database using ./data directory
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
