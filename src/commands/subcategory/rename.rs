use clap::{Arg, ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext, parsers::parse_label};

pub fn cli() -> Command {
  Command::new("rename")
    .about("Rename an existing subcategory")
    .arg(
      Arg::new("old")
        .help("Current subcategory name")
        .index(1)
        .value_parser(parse_label),
    )
    .arg(
      Arg::new("new")
        .help("The name you want to change subcategory to")
        .index(2)
        .value_parser(parse_label),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse {  })
}
