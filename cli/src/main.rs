use std::{
  fs::File,
  io::{self, BufReader},
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut writer = Terminal {};
  let cli = config::get_cli();

  match &cli.command {
    Some(Commands::Inspect { input_paths }) => {
      files_exist_or_exit(input_paths);

      for input_path in input_paths {
        writer.write(input_path.to_string_lossy())?;
        let reader = BufReader::new(File::open(input_path)?);
        serde_cbor::de::Deserializer::from_reader(reader)
          .into_iter::<serde_cbor::Value>()
          .map(|v| match v {
            Ok(v) => v,
            Err(e) => {
              eprintln!("Error: {:?} {} {:?}", e.classify(), e.offset(), e);
              process::exit(1);
            }
          })
          .for_each(|v| writer.write(format!("{:?}", v)).unwrap());
      }
    }

    Some(Commands::Import { input_paths, format }) => {
      files_exist_or_exit(input_paths);

      for input_path in input_paths {
        if format == "json" {
          let reader = BufReader::new(File::open(input_path)?);
          let result: serde_json::Value = match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(e) => {
              eprintln!("Error: {}", e);
              process::exit(1);
            }
          };
          serde_cbor::to_writer(std::io::stdout(), &result)?;
        } else if format == "yaml" {
          let reader = BufReader::new(File::open(input_path)?);
          let result: serde_yaml::Value = match serde_yaml::from_reader(reader) {
            Ok(v) => v,
            Err(e) => {
              eprintln!("Error: {}", e);
              process::exit(1);
            }
          };
          serde_cbor::to_writer(std::io::stdout(), &result)?;
        } else if format == "toml" {
          let s = std::fs::read_to_string(input_path)?;
          let result: toml::Value = match toml::de::from_str(&s) {
            Ok(v) => v,
            Err(e) => {
              eprintln!("Error: {}", e);
              process::exit(1);
            }
          };
          serde_cbor::to_writer(std::io::stdout(), &result)?;
        } else {
          let reader = BufReader::new(File::open(input_path)?);
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
      }
    }

    Some(Commands::Export { input_paths, format }) => {
      files_exist_or_exit(input_paths);

      if cli.verbose > 0 {
        writer.write(format!("Files to export: {:?}", input_paths))?;
        writer.write(format!("Exporting to {} format", format))?;
      }

      for (i, input_path) in input_paths.iter().enumerate() {
        if i > 0 {
          print!("{}", cli.delimiter);
        }
        let reader = BufReader::new(File::open(input_path)?);
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
              print!("{}", cli.delimiter);
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
      }
    }
    None => {}
  }

  Ok(())
}
