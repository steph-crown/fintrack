use clap::{ArgMatches, Command};

use crate::{CliResult, GlobalContext, commands::Exec};

pub fn cli() -> Command {
  Command::new("subcategory")
    .about("Manage subcategories: list, add, delete, or rename")
    .subcommand_required(true)
    .subcommands(build_cli())
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  match args.subcommand() {
    Some((cmd, sub_args)) => {
      let exec_fn = build_exec(cmd).ok_or_else(|| crate::CliError {})?;

      exec_fn(gctx, sub_args)
    }
    None => Err(crate::CliError {}), // Shouldn't happen due to subcommand_required
  }
}

fn build_cli() -> Vec<Command> {
  vec![add::cli(), list::cli()]
}

fn build_exec(cmd: &str) -> Option<Exec> {
  match cmd {
    "add" => Some(add::exec),
    "list" => Some(list::exec),
    _ => None,
  }
}

pub mod add;
pub mod list;
