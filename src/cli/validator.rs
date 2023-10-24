//! Validate CLI Arguments

extern crate std;

use std::path::{Path, PathBuf};

pub fn file_exists(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    if !*path.try_exists().as_ref().map_err(ToString::to_string)? {
        return Err(String::from("Ensure the file exists"));
    }
    if !path.is_file() {
        return Err(String::from("Ensure the path is a file"));
    }
    Ok(path.to_owned())
}
