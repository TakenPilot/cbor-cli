# CBOR cli

This is a command line tool for encoding and decoding CBOR. It uses the
serde crate to do encoding and decoding from various file formats to and
from CBOR.

## Installation

```bash
cargo install cbor-cli
```

## Import from file format into CBOR

```bash
cbor import test.json > test.cbor
```

## Export from CBOR into some file format.

```bash
cbor export --format=json test.cbor > test.json
```

## Delimiter

The default delimiter is a newline. You can change it to a comma or any other
string of characters.

```bash
cbor to --format=json --delimiter=, test.cbor > test.json
```

## Inspect

For debugging, you can dump the structure of one or more CBOR files to stdout.

```bash
cbor inspect test.cbor
```

## TODO

- Piped input
- `cbor import test.json > test.cbor`
- `cbor export --format=json test.cbor > test.json`
- Import and Export to CSV
- Import and Export to YAML
- Import and Export to TOML
- Import and Export to XML
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
