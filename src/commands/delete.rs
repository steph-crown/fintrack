use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};

use crate::{Category, CliResponse, CliResult, GlobalContext};

pub fn cli() -> Command {
  Command::new("delete")
    .about("Modify an existing record")
    .arg(
      Arg::new("ids")
        .help("Comma separated list of record ids")
        .short('i')
        .long("ids")
        .value_parser(clap::value_parser!(usize))
        .action(ArgAction::Append)
        .value_delimiter(','),
    )
    .arg(
      Arg::new("by-cat")
        .help("Specify a category. Deletes record for the category")
        .short('c')
        .long("category")
        .value_parser(clap::value_parser!(Category)),
    )
    .arg(
      Arg::new("by-subcat")
        .help("Specify a subcategory. Deletes record for the subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String)),
    )
    .group(
      ArgGroup::new("delete_by")
        .args(["ids", "by-cat", "by-subcat"])
        .multiple(false)
        .required(true),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse {  })
}
