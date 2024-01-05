use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  /// Level of verbosity
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub verbose: u8,

  /// Delimiter to use when printing multiple values
  #[arg(short, long, default_value = "\n")]
  pub delimiter: String,

  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Deep inspection of CBOR files, useful for debugging, learning or repairing files.
  Inspect {
    #[arg(value_name = "INPUT_PATHS")]
    input_paths: Vec<PathBuf>,
  },

  /// Convert a file of some other type to CBOR. Uses the file extension to
  /// determine the type if not specified.
  Import {
    #[arg(value_name = "INPUT_PATHS")]
    input_paths: Vec<PathBuf>,

    /// Format of the input file. If not specified, the file extension will be
    /// used to determine the type.
    #[arg(long, value_name = "FORMAT")]
    format: Option<String>,
  },

  /// Convert CBOR files to some other type.
  Export {
    #[arg(value_name = "INPUT_PATHS")]
    input_paths: Vec<PathBuf>,

    /// Format of the output file. If not specified, the file extension will be
    /// used to determine the type.
    #[arg(long, value_name = "FORMAT", default_value = "json")]
    format: String,
  },
}

pub fn get_cli() -> Cli {
  Cli::parse()
}
