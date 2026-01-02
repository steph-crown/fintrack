use clap::{ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext, utils::file::FilePath};

pub fn cli() -> Command {
  Command::new("clear").about("Delete all data and reset tracker to uninitialized state")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  gctx.base_path().delete_if_exists()?;
  Ok(CliResponse { success: true })
}
