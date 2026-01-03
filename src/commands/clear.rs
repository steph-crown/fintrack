use std::io::{self, Write};

use clap::{ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext, ResponseContent, utils::file::FilePath};

pub fn cli() -> Command {
  Command::new("clear").about("Delete all data and reset tracker to uninitialized state")
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
