use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("total").about("Display total income, expenses, and net balance")
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
