use clap::{ArgMatches, Command};

use crate::{
  CliError, CliResponse, CliResult, Currency, GlobalContext, Total, TrackerData,
  utils::file::FilePath,
};

pub fn cli() -> Command {
  Command::new("total").about("Display total income, expenses, and net balance")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let opening_balance = tracker_data.opening_balance;
  let currency = tracker_data
    .currency
    .parse::<Currency>()
    .map_err(|e| CliError::Other(format!("Invalid currency in tracker data: {}", e)))?;

  let (income_total, expenses_total) = tracker_data.totals();

  Ok(CliResponse::new(crate::ResponseContent::Total(Total {
    currency,
    opening_balance,
    income_total,
    expenses_total,
  })))
}
