use clap::{Arg, ArgMatches, Command};

use crate::parsers::parse_date;
use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("update")
    .about("Modify an existing record")
    .arg(
      Arg::new("record_id")
        .index(1)
        .value_parser(clap::value_parser!(usize)),
    )
    .arg(
      Arg::new("category")
        .short('c')
        .long("category")
        .value_parser(clap::value_parser!(Category)),
    )
    .arg(
      Arg::new("amount")
        .short('a')
        .long("amount")
        .value_parser(clap::value_parser!(f64)),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String)),
    )
    .arg(
      Arg::new("description")
        .short('d')
        .long("description")
        .value_parser(clap::value_parser!(String)),
    )
    .arg(
      Arg::new("date")
        .short('D')
        .long("date")
        .value_parser(parse_date),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse {  })
}
