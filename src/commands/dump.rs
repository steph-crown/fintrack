use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("dump").about("Pretty-print the raw JSON data to stdout")
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
