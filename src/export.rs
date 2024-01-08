use std::{io, process};

pub fn export_from_reader<R: io::Read, W: io::Write>(
  format: &str,
  delimiter: &str,
  reader: R,
  mut writer: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
  serde_cbor::de::Deserializer::from_reader(reader)
    .into_iter::<serde_cbor::Value>()
    .map(|v| match v {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Error: {:?} {} {:?}", e.classify(), e.offset(), e);
        process::exit(1);
      }
    })
    .enumerate()
    .try_for_each(|(i, v)| {
      if i > 0 {
        writer.write_all(delimiter.as_bytes())?;
      }
      if format == "json" {
        serde_json::to_writer(&mut writer, &v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      } else if format == "yaml" {
        serde_yaml::to_writer(&mut writer, &v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      } else if format == "toml" {
        let s = toml::ser::to_string(&v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writer.write_all(s.as_bytes())?;
      } else {
        serde_cbor::to_writer(&mut writer, &v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      }
      Ok::<(), io::Error>(())
    })?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use common_testing::assert;

  #[test]
  fn test_export_from_reader_to_json() {
    // Exports some CBOR data.
    let input = [0x82, 0x01, 0x02];
    let mut output = Vec::new();
    export_from_reader("json", "\n", &input[..], &mut output).unwrap();
    // println!("{}", String::from_utf8(output.clone()).unwrap());
    assert::equal(output, b"[1,2]".as_slice());
  }

  #[test]
  fn test_export_from_reader_to_yaml() {
    // Exports some CBOR data.
    let input = [0x82, 0x01, 0x02];
    let mut output = Vec::new();
    export_from_reader("yaml", "\n", &input[..], &mut output).unwrap();
    // println!("{}", String::from_utf8(output.clone()).unwrap());
    assert::equal(output, b"- 1\n- 2\n".as_slice());
  }

  #[test]
  fn test_export_from_reader_to_cbor() {
    // Exports some CBOR data.
    let input = [0x82, 0x01, 0x02];
    let mut output = Vec::new();
    export_from_reader("cbor", "\n", &input[..], &mut output).unwrap();
    // println!("{}", String::from_utf8(output.clone()).unwrap());
    assert::equal(output, b"\x82\x01\x02".as_slice());
  }

  #[test]
  fn test_export_from_reader_to_json_given_object() {
    // Exports
    let input = [0xa1, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61];
    let mut output = Vec::new();
    export_from_reader("json", "\n", &input[..], &mut output).unwrap();
    // println!("{}", String::from_utf8(output.clone()).unwrap());
    assert::equal(output, b"{\"a\":\"a\"}\n\"a\"".as_slice());
  }

  #[test]
  fn test_export_from_reader_to_yaml_given_object() {
    // Exports
    let input = [0xa1, 0x61, 0x61, 0x61, 0x61, 0x61, 0x61];
    let mut output = Vec::new();
    export_from_reader("yaml", "\n", &input[..], &mut output).unwrap();
    // println!("{}", String::from_utf8(output.clone()).unwrap());
    assert::equal(output, b"a: a\n\na\n".as_slice());
  }
}
