use std::{
  fs::{self, File},
  io,
  path::Path,
};

pub fn create_file_if_not_exists(path: &Path) -> io::Result<File> {
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?
  }

  File::options().write(true).create_new(true).open(path)
}
