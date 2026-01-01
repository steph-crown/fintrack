use std::path::PathBuf;

#[derive(Debug)]
pub struct GlobalContext {
  home_path: PathBuf, // The location of the user's home directory
  base_path: PathBuf,
  tracker_path: PathBuf, // The location of the tracker.json containing the data
  config_path: PathBuf,  // The location of configuration
  backups_path: PathBuf, // The location of backups.
}

impl GlobalContext {
  pub fn new(home_dir: PathBuf) -> Self {
    let base_path = home_dir.join(".fintrack");
    let tracker_path = base_path.join("tracker.json");
    let config_path = base_path.join("/config");
    let backups_path = base_path.join("/backups");

    GlobalContext {
      home_path: home_dir,
      base_path,
      tracker_path,
      config_path,
      backups_path,
    }
  }

  pub fn tracker_path(&self) -> &PathBuf {
    &self.tracker_path
  }
}
