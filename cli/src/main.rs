use std::{fs::File, io::BufReader, process};

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

  // You can check for the existence of subcommands, and if found use their
  // matches just as you would the top level cmd

  match &cli.command {
    Some(Commands::Dump { input_paths }) => {
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

    Some(Commands::To { input_paths, format: _ }) => {
      files_exist_or_exit(input_paths);

      for input_path in input_paths {
        let reader = BufReader::new(File::open(input_path)?);
        let result: serde_json::Value = match serde_json::from_reader(reader) {
          Ok(v) => v,
          Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
          }
        };
        serde_cbor::to_writer(std::io::stdout(), &result)?;
      }
    }

    Some(Commands::From { input_paths, format: _ }) => {
      files_exist_or_exit(input_paths);

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
            serde_json::to_writer(std::io::stdout(), &v)
          })?;
      }
    }
    None => {}
  }

  println!();

  Ok(())
}
