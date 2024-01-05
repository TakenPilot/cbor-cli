use std::io;

/// A generic write method for a specific type.
pub trait TypeWrite<T> {
  fn write(&mut self, target: T) -> io::Result<()>;
}

/// Represents Stdout.
pub struct Terminal {}

/// If they write to the Terminal, then write to Stdout.
impl<T: std::fmt::Display> TypeWrite<T> for Terminal {
  fn write(&mut self, target: T) -> io::Result<()> {
    println!("{}", target);
    Ok(())
  }
}

pub enum TypeWriter {
  Terminal(Terminal),
}
