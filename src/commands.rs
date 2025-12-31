use crate::{CliResult, command_prelude::*};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
  vec![
    add::cli(),
    clear::cli(),
    delete::cli(),
    describe::cli(),
    dump::cli(),
    init::cli(),
    list::cli(),
    total::cli(),
    update::cli(),
  ]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
  match cmd {
    "add" => Some(add::exec),
    "clear" => Some(clear::exec),
    "delete" => Some(delete::exec),
    "describe" => Some(describe::exec),
    "dump" => Some(dump::exec),
    "init" => Some(init::exec),
    "list" => Some(list::exec),
    "total" => Some(total::exec),
    "update" => Some(update::exec),
    _ => None,
  }
}

pub mod add;
pub mod clear;
pub mod delete;
pub mod describe;
pub mod dump;
pub mod init;
pub mod list;
pub mod total;
pub mod update;
