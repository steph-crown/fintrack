use clap::{ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext, ResponseContent, TrackerData, utils::file::FilePath};

pub fn cli() -> Command {
  Command::new("list")
    .about("List all available categories")
    .long_about("Displays all categories with their IDs. Categories are immutable and cannot be created, deleted, or renamed. There are only two: Income (ID: 1) and Expenses (ID: 2).")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let mut categories: Vec<(usize, String)> = tracker_data
    .categories
    .iter()
    .map(|(name, &id)| (id, name.clone()))
    .collect();

  categories.sort_by_key(|(id, _)| *id);

  Ok(CliResponse::new(ResponseContent::Categories(categories)))
}
