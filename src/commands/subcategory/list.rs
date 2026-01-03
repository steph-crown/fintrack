use clap::{ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext, ResponseContent, TrackerData, utils::file::FilePath};

pub fn cli() -> Command {
  Command::new("list").about("View all available subcategories")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let mut subcategories: Vec<(usize, String)> = tracker_data
    .subcategories_by_id
    .iter()
    .map(|(&id, name)| (id, name.clone()))
    .collect();

  subcategories.sort_by_key(|(id, _)| *id);

  Ok(CliResponse::new(ResponseContent::Subcategories(subcategories)))
}
