use std::path::Path;

/// Get a displayable string of a path.
pub fn get_path_str(path: &Path) -> &str {
  path.to_str().unwrap()
}
