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

  fn open_read(&self) -> io::Result<File> {
    File::options().read(true).open(self.as_ref())
  }

  fn delete_if_exists(&self) -> io::Result<()> {
    let path = self.as_ref();
    if !path.exists() {
      return Ok(());
    }
    if path.is_dir() {
      fs::remove_dir_all(path)?;
    } else {
      fs::remove_file(path)?;
    }
    Ok(())
  }
}

impl<P: AsRef<Path>> FilePath for P {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::TempDir;

    #[test]
    fn test_create_file_if_not_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        let result = file_path.create_file_if_not_exists();
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn test_create_file_if_not_exists_creates_parent_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("parent").join("child").join("test.json");

        let result = file_path.create_file_if_not_exists();
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn test_create_file_if_not_exists_fails_when_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        file_path.create_file_if_not_exists().unwrap();
        let result = file_path.create_file_if_not_exists();
        assert!(result.is_err());
    }

    #[test]
    fn test_read_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        fs::write(&file_path, "test content").unwrap();

        let mut file = file_path.read_file().unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        assert_eq!(content, "test content");
    }

    #[test]
    fn test_read_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.json");

        let result = file_path.read_file();
        assert!(result.is_err());
    }

    #[test]
    fn test_open_read_write() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        fs::write(&file_path, "initial content").unwrap();

        let mut file = file_path.open_read_write().unwrap();
        file.set_len(0).unwrap(); // Truncate the file
        file.seek(io::SeekFrom::Start(0)).unwrap();
        file.write_all(b"new content").unwrap();
        file.seek(io::SeekFrom::Start(0)).unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "new content");
    }

    #[test]
    fn test_delete_if_exists_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        fs::write(&file_path, "content").unwrap();
        assert!(file_path.exists());

        file_path.delete_if_exists().unwrap();
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_if_exists_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("test_dir");

        fs::create_dir_all(&dir_path).unwrap();
        assert!(dir_path.exists());

        dir_path.delete_if_exists().unwrap();
        assert!(!dir_path.exists());
    }

    #[test]
    fn test_delete_if_exists_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.json");

        // Should not error if file doesn't exist
        let result = file_path.delete_if_exists();
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_json_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        let json = serde_json::json!({
            "key": "value",
            "number": 42
        });

        let mut file = file_path.create_file_if_not_exists().unwrap();
        write_json_to_file(&json, &mut file).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let parsed: Value = serde_json::from_str(&content).unwrap();

        assert_eq!(parsed["key"], "value");
        assert_eq!(parsed["number"], 42);
    }

    #[test]
    fn test_write_json_to_file_overwrites() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.json");

        let json1 = serde_json::json!({"old": "data"});
        let json2 = serde_json::json!({"new": "data"});

        let mut file = file_path.create_file_if_not_exists().unwrap();
        write_json_to_file(&json1, &mut file).unwrap();

        let mut file = file_path.open_read_write().unwrap();
        write_json_to_file(&json2, &mut file).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let parsed: Value = serde_json::from_str(&content).unwrap();

        assert_eq!(parsed["new"], "data");
        assert!(parsed.get("old").is_none());
    }
}
