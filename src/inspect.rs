use std::{io, process};

pub fn inspect_from_reader<R: io::Read, W: io::Write>(reader: R, mut writer: W) -> Result<(), std::io::Error> {
  serde_cbor::de::Deserializer::from_reader(reader)
    .into_iter::<serde_cbor::Value>()
    .map(|v| match v {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Error: {:?} {} {:?}", e.classify(), e.offset(), e);
        process::exit(1);
      }
    })
    .try_for_each(|v| {
      writer.write_all(format!("{:?}\n", v).as_bytes())?;
      io::Result::Ok(())
    })
}

#[cfg(test)]
mod tests {
  use super::*;
  use common_testing::assert;

  #[test]
  fn test_inspect_from_reader() {
    // Inspects some CBOR data.
    let input = [0x82, 0x01, 0x02];
    let mut output = Vec::new();
    inspect_from_reader(&input[..], &mut output).unwrap();
    // println!("{}", String::from_utf8(output.clone()).unwrap());
    assert::equal(output, b"Array([Integer(1), Integer(2)])\n".as_slice());
  }
}
