use crate::{CliResult, command_prelude::*};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
  vec![init::cli()]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
  match cmd {
    "init" => Some(init::exec),
    _ => None,
  }
}

pub mod init;
