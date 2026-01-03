use std::io::{self, Write};

use clap::{ArgMatches, Command};

use crate::{utils::file::FilePath, CliResponse, CliResult, GlobalContext, ResponseContent};

pub fn cli() -> Command {
  Command::new("clear")
    .about("Delete all data and reset tracker")
    .long_about("Permanently deletes all your financial data including all records, subcategories, and the tracker file itself. This action cannot be undone. You will be prompted to confirm before deletion. After clearing, you can run 'fintrack init' to start fresh.")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  print!("Delete ALL data? This cannot be undone. (yes/no): ");
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  let input = input.trim().to_lowercase();

  if input == "yes" || input == "y" {
    gctx.base_path().delete_if_exists()?;
    Ok(CliResponse::new(ResponseContent::Message(
      "All data cleared. Run 'fintrack init' to start over.".to_string(),
    )))
  } else {
    Ok(CliResponse::new(ResponseContent::Message(
      "Clear cancelled.".to_string(),
    )))
  }
}
