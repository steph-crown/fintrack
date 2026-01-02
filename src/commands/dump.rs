use clap::{ArgMatches, Command};

use crate::{
  CliResponse, CliResult, GlobalContext, ResponseContent, TrackerData, utils::file::FilePath,
};

pub fn cli() -> Command {
  Command::new("dump").about("Pretty-print the raw JSON data to stdout")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;

  Ok(CliResponse::new(ResponseContent::TrackerData(tracker_data)))
}
