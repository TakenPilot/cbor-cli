use std::path::Path;

/// Get a displayable string of a path.
pub fn get_path_str(path: &Path) -> &str {
  path.to_str().unwrap()
}

/// Get a list of file paths that are not existing files.
/// Will not check for the existence of directories.
pub fn get_missing_files(input_paths: &Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
  let mut missing_files = Vec::new();

  for input_path in input_paths {
    if !input_path.is_file() {
      missing_files.push(input_path.clone());
    }
  }

  missing_files
}

pub fn get_format_by_file_extension(input_path: &Path) -> Option<String> {
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

#[cfg(test)]
mod tests {
  use super::*;
  use common_testing::assert;

  #[test]
  fn test_get_path_str() {
    let path = Path::new("foo");
    assert::equal(get_path_str(path), "foo");
  }

  #[test]
  fn test_get_missing_files() {
    let input_paths = vec![
      Path::new("foo").to_path_buf(),
      Path::new("bar").to_path_buf(),
      Path::new("baz").to_path_buf(),
    ];
    let missing_files = vec![
      Path::new("foo").to_path_buf(),
      Path::new("bar").to_path_buf(),
      Path::new("baz").to_path_buf(),
    ];

    assert::equal(get_missing_files(&input_paths), missing_files);
  }

  #[test]
  fn test_get_format_by_file_extension() {
    let path = Path::new("foo.json");
    assert::equal(get_format_by_file_extension(path), Some("json".to_string()));

    let path = Path::new("foo.yaml");
    assert::equal(get_format_by_file_extension(path), Some("yaml".to_string()));

    let path = Path::new("foo.toml");
    assert::equal(get_format_by_file_extension(path), Some("toml".to_string()));

    let path = Path::new("foo.bar");
    assert::equal(get_format_by_file_extension(path), None);
  }
}
