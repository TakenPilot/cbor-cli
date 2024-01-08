use std::{io, process};

pub fn import_from_reader<R: io::Read, W: io::Write>(
  input_format: &str,
  mut reader: R,
  mut writer: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
  if input_format == "json" {
    serde_json::de::Deserializer::from_reader(reader)
      .into_iter::<serde_json::Value>()
      .map(|v| match v {
        Ok(v) => v,
        Err(e) => {
          eprintln!("Error: {:?} {} {} {:?}", e.classify(), e.column(), e.line(), e);
          process::exit(1);
        }
      })
      .for_each(|v| serde_cbor::to_writer(&mut writer, &v).unwrap());
  } else if input_format == "yaml" {
    let result: serde_yaml::Value = match serde_yaml::from_reader(reader) {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Error: {:?}, {:?}", e.location(), e);
        process::exit(1);
      }
    };
    serde_cbor::to_writer(writer, &result)?;
  } else if input_format == "toml" {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    let result: toml::Value = match toml::de::from_str(&s) {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Error: {} {:?} {:?}", e.message(), e.span(), e);
        process::exit(1);
      }
    };
    serde_cbor::to_writer(writer, &result)?;
  } else {
    serde_cbor::de::Deserializer::from_reader(reader)
      .into_iter::<serde_cbor::Value>()
      .map(|v| match v {
        Ok(v) => v,
        Err(e) => {
          eprintln!("Error: {:?} {} {:?}", e.classify(), e.offset(), e);
          process::exit(1);
        }
      })
      .for_each(|v| serde_cbor::to_writer(&mut writer, &v).unwrap());
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use common_testing::assert;

  #[test]
  fn test_import_from_reader_from_json() {
    // Imports some JSON data.
    let input = b"[1,2]";
    let mut output = Vec::new();
    import_from_reader("json", &input[..], &mut output).unwrap();
    // println!("{:x?}", output);
    assert::equal_hex_bytes(&output, "820102");
  }

  #[test]
  fn test_import_from_reader_from_yaml() {
    // Imports some YAML data.
    let input = b"- 1\n- 2\n";
    let mut output = Vec::new();
    import_from_reader("yaml", &input[..], &mut output).unwrap();
    // println!("{:x?}", output);
    assert::equal_hex_bytes(&output, "820102");
  }

  #[test]
  fn test_import_from_reader_from_toml() {
    // Imports some TOML data.
    let input = b"foo = 1\nbar = 2\n";
    let mut output = Vec::new();
    import_from_reader("toml", &input[..], &mut output).unwrap();
    // println!("{:x?}", output);
    assert::equal_hex_bytes(&output, "a263666f6f016362617202");
  }

  #[test]
  fn test_import_from_reader_from_cbor() {
    // Imports some CBOR data.
    let input = b"\x82\x01\x02";
    let mut output = Vec::new();
    import_from_reader("cbor", &input[..], &mut output).unwrap();
    // println!("{:x?}", output);
    assert::equal_hex_bytes(&output, "820102");
  }
}
