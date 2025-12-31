use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("clear").about("Delete all data and reset tracker to uninitialized state")
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
