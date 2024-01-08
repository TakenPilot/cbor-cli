#!/bin/sh
#
# Test script for the cli, and also examples of how to use it.
#

# Support json file
cargo run -q -- import --format=json ../fixtures/test.json | cargo run -- --verbose -d=",\n" export --format=json

# Support multiple json files
cargo run -q -- import --format=json ../fixtures/test.json ../fixtures/test.json | cargo run -- --verbose -d=",\n" export --format=json

# Support json stdin
cat ../fixtures/test.json | cargo run -q -- import --format=json | cargo run -- --verbose -d=",\n" export --format=json

# Support multiple json stdin
cat ../fixtures/test.json ../fixtures/test.json | cargo run -q -- import --format=json | cargo run -- --verbose -d=",\n" export --format=json

# Support yaml file
cargo run -q -- import --format=yaml ../fixtures/test.yaml | cargo run -- --verbose export --format=yaml

# Support multiple yaml files
cargo run -q -- import --format=yaml ../fixtures/test.yaml ../fixtures/test.yaml | cargo run -- export --format=yaml

# Support yaml stdin
# (Multiple input files from stdin not supported because of file format)
cat ../fixtures/test.yaml | cargo run -q -- import --format=yaml | cargo run -- --verbose export --format=yaml

# Support toml file
cargo run -q -- import --format=toml ../fixtures/test.toml | cargo run -- --verbose export --format=toml

# Support multiple toml files
# (Though multiple input files supported, variables just override each other and the last one wins. Could be useful for hierarchies?)
cargo run -q -- import --format=toml ../fixtures/test.toml ../fixtures/test.toml | cargo run -- --verbose export --format=toml

# Support toml stdin
# (Multiple input files from stdin not supported because of file format)
cat ../fixtures/test.toml | cargo run -q -- import --format=toml | cargo run -- --verbose export --format=toml

# Inspect works
cargo run -q -- import --format=toml ../fixtures/test.toml | cargo run -- --verbose inspect

# Inspect works Even across multiple files
cargo run -q -- import --format=toml ../fixtures/test.toml ../fixtures/test.toml | cargo run -- --verbose inspect
cargo run -q -- import --format=json ../fixtures/test.json ../fixtures/test.json | cargo run -- --verbose inspect
cargo run -q -- import --format=json ../fixtures/test.json ../fixtures/test.json | cargo run -- --verbose inspect

# Inspect works across multiple formats if you pipe them to cbor first
cargo run -q -- import ../fixtures/test.json ../fixtures/test.yaml | cargo run -- --verbose inspect
