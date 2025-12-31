use clap::{Arg, ArgMatches, Command};

use crate::{CliResponse, CliResult, GlobalContext, parsers::parse_label};

pub fn cli() -> Command {
  Command::new("delete")
    .about("Delete a subcategory (only if it has no records)")
    .arg(
      Arg::new("name")
        .index(1)
        .value_parser(parse_label)
        .help("Name of subcategory"),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fucklist {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
