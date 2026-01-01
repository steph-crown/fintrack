use std::io::prelude::*;

use clap::{Arg, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::create_file_if_not_exists;
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
  // let currency = args.get_one::<Currency>("currency").unwrap();
  let currency = args.value_of_currency_or_def("currency");
  let balance = args.value_of_f64_or_zero("balance");
  let mut file = create_file_if_not_exists(gctx.tracker_path())?;

  let default_json = default_tracker_json(currency, *balance);
  let json_string = serde_json::to_string_pretty(&default_json)?;

  file.write_all(json_string.as_bytes())?;

  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
