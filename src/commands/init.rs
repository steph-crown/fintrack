use clap::{Arg, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::{CliResponse, CliResult, Currency, GlobalContext, default_tracker_json};

pub fn cli() -> Command {
  Command::new("init")
    .about("Initialize a new financial tracker")
    .long_about("Creates a new tracker file in ~/.fintrack/ with default categories (Income, Expenses) and a default subcategory (Miscellaneous). You must run this command before using any other commands.")
    .arg(
      Arg::new("currency")
        .short('c')
        .long("currency")
        .value_parser(clap::value_parser!(Currency))
        .default_value("ngn")
        .help("Currency code for your tracker (NGN, USD, GBP, EUR, CAD, AUD, JPY)")
        .long_help("Sets the currency that will be used for all amounts. This cannot be changed after initialization. Defaults to NGN if not specified."),
    )
    .arg(
      Arg::new("opening")
        .short('o')
        .long("opening")
        .value_parser(clap::value_parser!(f64))
        .help("Your opening balance amount")
        .long_help("Sets your starting balance. This is the amount you have before adding any income or expenses. Defaults to 0.0 if not specified."),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let currency = args.get_currency_or_default("currency");
  let opening_balance = args.get_f64_or_default("opening");

  // std::fs::create_dir_all(gctx.backups_path())?;

  let mut file = gctx.tracker_path().create_file_if_not_exists()?;

  let default_json = default_tracker_json(currency, opening_balance);
  write_json_to_file(&default_json, &mut file)?;

  Ok(CliResponse::success())
}
