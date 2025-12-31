use crate::{CliResult, command_prelude::*};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
  vec![add::cli(), delete::cli(), init::cli(), update::cli()]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
  match cmd {
    "add" => Some(add::exec),
    "delete" => Some(delete::exec),
    "init" => Some(init::exec),
    "update" => Some(update::exec),
    _ => None,
  }
}

pub mod add;
pub mod delete;
pub mod init;
pub mod update;
