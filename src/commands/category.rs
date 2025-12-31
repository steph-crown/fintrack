use clap::{ArgMatches, Command};

use crate::{CliResult, GlobalContext, commands::Exec};

pub fn cli() -> Command {
  Command::new("category")
    .about("Manage categories")
    .subcommand_required(true)
    .subcommands([list::cli()])
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  match args.subcommand() {
    Some((cmd, sub_args)) => {
      let exec_fn = build_exec(cmd).ok_or_else(|| crate::CliError {})?;

      exec_fn(gctx, sub_args)
    }
    // Some((_, _)) => Err(crate::CliError {}), // Unknown subcommand
    None => Err(crate::CliError {}), // Shouldn't happen due to subcommand_required
  }
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
  match cmd {
    "list" => Some(list::exec),
    _ => None,
  }
}

pub mod list;
