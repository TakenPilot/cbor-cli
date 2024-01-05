# CBOR CLI

Command line tool for encoding and decoding CBOR using serde. Supports import and export for JSON, YAML, and TOML. Supports deep inspection of CBOR files.

Features:

- Import and Export from JSON, YAML, TOML
- Inspect CBOR files for debugging
- Supports piped input and output (stdin and stdout)
- Supports multiple input files or multiple items in a single file
- Supports custom delimiters output

## Installation

```bash
cargo install cbor-cli
```

## Import

```bash
cbor import test.json > test.cbor
```

# Export

Example of exporting to a JSON file:

```bash
cbor export --format=json test.cbor > test.json
```

Example of importing stdin and then exporting to stdout:

```bash
cat test1.json test2.json | cbor import --format=json | cbor -d=",\n" export --format=json
```

Example of importing in one format and exporting in another:

```bash
cbor import test.json | cbor -d=",\n" export --format=yaml > test.yaml
```

## Inspect

For debugging, you can dump the structure of one or more CBOR files to stdout.

```bash
cbor inspect test.cbor
```

You can inspect the resulting data across multiple types of files if you pipe to cbor first:

```bash
cbor import ../fixtures/test.json ../fixtures/test.yaml | cbor inspect
```

## Delimiter

You can specify a unique delimiter.

```bash
cbor export --format=json --delimiter=, test.cbor > test.json
```

## TODO

- Import and Export to Parquet
- Inspect Tag support: Compression
- Inspect Tag support: Date, Time, Timestamp
- Inspect Tag support: BigNum, Fractions, Decimals
- Inspect Tag support: Geo and Spacial coordinates
- Inspect Tag support: Deferred CBOR
- Inspect Tag support: UUID
- Inspect Tag support: Base64
- Inspect Tag support: Base16
- Inspect Tag support: URI and URLs
