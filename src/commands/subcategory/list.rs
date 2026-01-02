use clap::{ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("list").about("View all available categories")
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fucklist {:#?}", gctx, args);
  Ok(CliResponse {  })
}
