use clap::{Arg, ArgMatches, Command};

use crate::parsers::parse_date;
use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("list")
    .about("View records with optional filtering by date, category, or subcategory")
    .arg(
      Arg::new("first")
        .short('f')
        .long("first")
        .value_parser(clap::value_parser!(usize)),
    )
    .arg(
      Arg::new("last")
        .short('l')
        .long("last")
        .value_parser(clap::value_parser!(usize)),
    )
    .arg(
      Arg::new("start")
        .short('S')
        .long("start")
        .value_parser(parse_date),
    )
    .arg(
      Arg::new("end")
        .short('E')
        .long("end")
        .value_parser(parse_date),
    )
    .arg(
      Arg::new("category")
        .short('c')
        .long("category")
        .value_parser(clap::value_parser!(Category)),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String)),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse::success())
}
