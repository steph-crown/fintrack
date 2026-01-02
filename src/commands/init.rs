use clap::{Arg, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::{CliResponse, CliResult, Currency, GlobalContext, default_tracker_json};

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
    .arg(
      Arg::new("balance")
        .short('b')
        .long("balance")
        .value_parser(clap::value_parser!(f64)),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let currency = args.value_of_currency_or_def("currency");
  let balance = args.value_of_f64_or_zero("balance");
  let mut file = gctx.tracker_path().create_file_if_not_exists()?;

  let default_json = default_tracker_json(currency, *balance);
  write_json_to_file(&default_json, &mut file)?;

  Ok(CliResponse {  })
}
