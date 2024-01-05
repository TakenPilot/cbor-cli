use std::{
  fs::File,
  io::{self, BufReader, Read},
  path::Path,
  process,
};

use cbor_cli::{
  config::{self, Commands},
  files::get_path_str,
  traits::{Terminal, TypeWrite},
};

/// Get a list of file paths that are not existing files.
/// Will not check for the existence of directories.
fn get_missing_files(input_paths: &Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
  let mut missing_files = Vec::new();

  for input_path in input_paths {
    if !input_path.is_file() {
      missing_files.push(input_path.clone());
    }
  }

  missing_files
}

/// Print out the missing files and exit with a non-zero exit code.
fn files_exist_or_exit(input_paths: &Vec<std::path::PathBuf>) {
  let missing_files = get_missing_files(input_paths);
  if !missing_files.is_empty() {
    for missing_file in missing_files {
      println!("File does not exist: {}", get_path_str(&missing_file));
    }
    process::exit(1);
  }
}

fn get_format_by_file_extension(input_path: &Path) -> Option<String> {
  match input_path.extension() {
    Some(extension) => match extension.to_str() {
      Some(extension) => match extension {
        "json" => Some("json".to_string()),
        "yaml" => Some("yaml".to_string()),
        "toml" => Some("toml".to_string()),
        _ => None,
      },
      None => None,
    },
    None => None,
  }
}

fn inspect_from_reader<T: Read>(reader: BufReader<T>) -> Result<(), Box<dyn std::error::Error>> {
  serde_cbor::de::Deserializer::from_reader(reader)
    .into_iter::<serde_cbor::Value>()
    .map(|v| match v {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Error: {:?} {} {:?}", e.classify(), e.offset(), e);
        process::exit(1);
      }
    })
    .for_each(|v| {
      println!("{:?}", v);
    });
  Ok(())
}

fn import_from_reader<T: Read>(input_format: &str, mut reader: BufReader<T>) -> Result<(), Box<dyn std::error::Error>> {
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
      .for_each(|v| serde_cbor::to_writer(std::io::stdout(), &v).unwrap());
  } else if input_format == "yaml" {
    let result: serde_yaml::Value = match serde_yaml::from_reader(reader) {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Error: {:?}, {:?}", e.location(), e);
        process::exit(1);
      }
    };
    serde_cbor::to_writer(std::io::stdout(), &result)?;
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
    serde_cbor::to_writer(std::io::stdout(), &result)?;
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
      .for_each(|v| serde_cbor::to_writer(std::io::stdout(), &v).unwrap());
  }
  Ok(())
}

fn export_from_reader<T: Read>(
  format: &str,
  delimiter: &str,
  reader: BufReader<T>,
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
        print!("{}", delimiter);
      }
      if format == "json" {
        serde_json::to_writer(io::stdout(), &v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      } else if format == "yaml" {
        serde_yaml::to_writer(io::stdout(), &v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      } else if format == "toml" {
        let s = toml::ser::to_string(&v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        print!("{}", s);
      } else {
        serde_cbor::to_writer(io::stdout(), &v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
      }
      Ok::<(), io::Error>(())
    })?;
  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut writer = Terminal {};
  let cli = config::get_cli();

  match &cli.command {
    Some(Commands::Inspect { input_paths }) => {
      files_exist_or_exit(input_paths);

      if input_paths.is_empty() {
        let reader = BufReader::new(io::stdin());
        inspect_from_reader(reader)?;
        return Ok(());
      }

      for input_path in input_paths {
        writer.write(input_path.to_string_lossy())?;
        let reader = BufReader::new(File::open(input_path)?);
        inspect_from_reader(reader)?;
      }
    }

    Some(Commands::Import { input_paths, format }) => {
      files_exist_or_exit(input_paths);

      if input_paths.is_empty() {
        // If the format is not specified, error out.
        let input_format = match format {
          Some(format) => format.to_owned(),
          None => {
            eprintln!("Error: No format specified. Use --format to specify the format.");
            process::exit(1);
          }
        };

        let reader = BufReader::new(io::stdin());
        import_from_reader(&input_format, reader)?;
        return Ok(());
      }

      for input_path in input_paths {
        // If the format is not specified, then use the file extension to determine
        // the format.
        let input_format = match format {
          Some(format) => format.to_owned(),
          None => match get_format_by_file_extension(input_path) {
            Some(format) => format,
            None => {
              eprintln!("Error: Could not determine format from file extension");
              process::exit(1);
            }
          },
        };

        let reader = BufReader::new(File::open(input_path)?);
        import_from_reader(&input_format, reader)?;
      }
    }

    Some(Commands::Export { input_paths, format }) => {
      files_exist_or_exit(input_paths);

      let delimiter = match &cli.delimiter {
        Some(delimiter) => {
          // Unescape delimiter so that "\n" and "\t" can be used.
          delimiter.to_string().replace("\\n", "\n").replace("\\t", "\t")
        }
        None => {
          if format == "yaml" {
            // Use "---" as the delimiter for YAML, which represents the
            // start of a new document.
            "\n---\n".to_string()
          } else {
            "\n".to_string()
          }
        }
      };

      if input_paths.is_empty() {
        let reader = BufReader::new(io::stdin());
        export_from_reader(format, &delimiter, reader)?;
        return Ok(());
      }

      if cli.verbose > 0 {
        writer.write(format!("Files to export: {:?}", input_paths))?;
        writer.write(format!("Exporting to {} format", format))?;
      }

      for (i, input_path) in input_paths.iter().enumerate() {
        if i > 0 {
          print!("{}", delimiter);
        }
        let reader = BufReader::new(File::open(input_path)?);
        export_from_reader(format, &delimiter, reader)?;
      }
    }
    None => {}
  }

  Ok(())
}
