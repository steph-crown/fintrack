use std::io::prelude::*;

use std::{
  fs::{self, File},
  io,
  path::Path,
};

use serde_json::Value;

use crate::CliError;

pub fn write_json_to_file(json: &Value, file: &mut File) -> Result<(), CliError> {
  let json_string = serde_json::to_string_pretty(&json)?;

  file.seek(io::SeekFrom::Start(0))?;
  file.set_len(0)?;
  file.write_all(json_string.as_bytes())?;

  Ok(())
}

pub trait FilePath: AsRef<Path> {
  fn create_file_if_not_exists(&self) -> io::Result<File> {
    let path = self.as_ref();
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?
    }
    File::options().write(true).create_new(true).open(path)
  }

  fn read_file(&self) -> io::Result<File> {
    File::options().read(true).open(self.as_ref())
  }

  fn open_read_write(&self) -> io::Result<File> {
    File::options().read(true).write(true).open(self.as_ref())
  }

  fn delete_if_exists(&self) -> io::Result<()> {
    let path = self.as_ref();
    if path.is_dir() {
      fs::remove_dir_all(path)?;
    } else {
      fs::remove_file(path)?;
    }
    Ok(())
  }
}

impl<P: AsRef<Path>> FilePath for P {}
