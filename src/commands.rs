use crate::{CliResult, command_prelude::*};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
  vec![init::cli(), add::cli(), update::cli()]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
  match cmd {
    "init" => Some(init::exec),
    "add" => Some(add::exec),
    "update" => Some(update::exec),
    _ => None,
  }
}

pub mod add;
pub mod init;
pub mod update;
