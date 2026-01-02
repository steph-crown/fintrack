use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::{CliResponse, CliResult, ExportFileType, GlobalContext};

pub fn cli() -> Command {
  Command::new("export")
    .about("Export tracker data to CSV or JSON file")
    .arg(
      Arg::new("path")
        .help("Folder path where exported file should be created in")
        .index(1)
        .value_parser(clap::value_parser!(PathBuf)),
    )
    .arg(
      Arg::new("type")
        .help("The file type the export should be in. Defaults to json.")
        .short('t')
        .long("type")
        .value_parser(clap::value_parser!(ExportFileType)),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  println!("{:#?} fuck {:#?}", gctx, args);
  Ok(CliResponse::success())
}
