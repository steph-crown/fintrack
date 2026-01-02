use clap::{ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("dump").about("Pretty-print the raw JSON data to stdout")
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);

  // read it
  Ok(CliResponse {})
}
