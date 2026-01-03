use clap::{ArgMatches, Command};

use crate::{CliResult, GlobalContext, commands::Exec, invalid_subcommand_error};

pub fn cli() -> Command {
  Command::new("subcategory")
    .about("Manage your subcategories")
    .long_about("Subcategories help you organize transactions into more specific groups (e.g., 'Groceries', 'Salary', 'Rent'). You can create custom subcategories, view them, rename them, or delete them (if they have no records). The default 'Miscellaneous' subcategory cannot be deleted.")
    .subcommand_required(true)
    .subcommands(build_cli())
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  match args.subcommand() {
    Some((cmd, sub_args)) => {
      let exec_fn = build_exec(cmd).ok_or_else(|| invalid_subcommand_error(cmd))?;

      exec_fn(gctx, sub_args)
    }
    None => Err(invalid_subcommand_error("")), // Shouldn't happen due to subcommand_required
  }
}

fn build_cli() -> Vec<Command> {
  vec![add::cli(), delete::cli(), list::cli(), rename::cli()]
}

fn build_exec(cmd: &str) -> Option<Exec> {
  match cmd {
    "add" => Some(add::exec),
    "delete" => Some(delete::exec),
    "list" => Some(list::exec),
    "rename" => Some(rename::exec),
    "update" => Some(rename::exec), // Alias for rename
    _ => None,
  }
}

pub mod add;
pub mod delete;
pub mod list;
pub mod rename;
