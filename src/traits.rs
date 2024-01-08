use std::io::{self, Stdout};

/// A generic write method for a specific type.
pub trait WriteStr<T> {
  fn write_str(&mut self, target: T) -> io::Result<()>;
}

/// If they write to the Terminal, then write to Stdout.
impl<T: std::fmt::Display> WriteStr<T> for Stdout {
  fn write_str(&mut self, target: T) -> io::Result<()> {
    println!("{}", target);
    Ok(())
  }
}
