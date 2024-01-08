use std::{
  fs::File,
  io::{self, BufReader, Write},
  process,
};

use cbor_cli::{
  config::{self, Commands},
  export::export_from_reader,
  files::{get_format_by_file_extension, get_missing_files, get_path_str},
  import::import_from_reader,
  inspect::inspect_from_reader,
  traits::WriteStr,
};

/// Print out the missing files and exit with a non-zero exit code.
fn files_exist_or_exit(input_paths: &Vec<std::path::PathBuf>) {
  let missing_files = get_missing_files(input_paths);
  if !missing_files.is_empty() {
    for missing_file in missing_files {
      eprintln!("File does not exist: {}", get_path_str(&missing_file));
    }
    process::exit(1);
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut writer = std::io::stdout();
  let cli = config::get_cli();

  match &cli.command {
    Some(Commands::Inspect { input_paths }) => {
      files_exist_or_exit(input_paths);

      if input_paths.is_empty() {
        let reader = BufReader::new(io::stdin());
        inspect_from_reader(reader, &writer)?;
        return Ok(());
      }

      for input_path in input_paths {
        writer.write_str(input_path.to_string_lossy())?;
        let reader = BufReader::new(File::open(input_path)?);
        inspect_from_reader(reader, &writer)?;
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
        import_from_reader(&input_format, reader, &mut writer)?;
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
        import_from_reader(&input_format, reader, &mut writer)?;
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
        export_from_reader(format, &delimiter, reader, &mut writer)?;
        return Ok(());
      }

      if cli.verbose > 0 {
        writer.write_str(format!("Files to export: {:?}", input_paths))?;
        writer.write_str(format!("Exporting to {} format", format))?;
      }

      for (i, input_path) in input_paths.iter().enumerate() {
        if i > 0 {
          writer.write_all(delimiter.as_bytes())?;
        }
        let reader = BufReader::new(File::open(input_path)?);
        export_from_reader(format, &delimiter, reader, &mut writer)?;
      }
    }
    None => {}
  }

  Ok(())
}
