use clap::{Arg, ArgMatches, Command};

use crate::parsers::parse_date;
use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("add")
    .about("Record a new income or expense transaction")
    .arg(
      Arg::new("category")
        .index(1)
        .value_parser(clap::value_parser!(Category)),
    )
    .arg(
      Arg::new("amount")
        .index(2)
        .value_parser(clap::value_parser!(f64)),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("category")
        .value_parser(clap::value_parser!(String)),
    )
    .arg(
      Arg::new("description")
        .short('d')
        .long("description")
        .value_parser(clap::value_parser!(String)),
    )
    .arg(Arg::new("date").value_parser(parse_date))
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
