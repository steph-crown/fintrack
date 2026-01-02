use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("describe").about("Show exploratory data analysis of your spending")
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse {  })
}
