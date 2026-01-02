use clap::{ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext, TrackerData, utils::file::FilePath};

pub fn cli() -> Command {
  Command::new("total").about("Display total income, expenses, and net balance")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;
  let records = tracker_data.records;

  let initial_balance = tracker_data.opening_balance;
  let total_income: f64 = records
    .iter()
    .filter_map(|rec| {
      if rec.category == 1 {
        Some(rec.amount)
      } else {
        None
      }
    })
    .sum();

  let total_expenses: f64 = records
    .iter()
    .filter_map(|rec| {
      if rec.category == 2 {
        Some(rec.amount)
      } else {
        None
      }
    })
    .sum();

  Ok(CliResponse::success())
}
