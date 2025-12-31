use clap::{Arg, ArgMatches, Command};

use crate::{CliResponse, CliResult, Currency, GlobalContext};

pub fn cli() -> Command {
  Command::new("init")
    .about("Initialize a new tracker with currency and default categories")
    .arg(
      Arg::new("currency")
        .short('c')
        .long("currency")
        .value_parser(clap::value_parser!(Currency))
        .default_value("ngn"),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  // attempt to create file
  // if file already exists, return an Err
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
