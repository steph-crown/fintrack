use std::io::prelude::*;
use std::{fs::File, path::Path};

use clap::{Arg, ArgMatches, Command};

use crate::utils::file::create_file_if_not_exists;
use crate::{CliError, CliResponse, CliResult, Currency, GlobalContext};

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
  let mut file = create_file_if_not_exists(gctx.tracker_path())?;

  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse { success: true })
}
