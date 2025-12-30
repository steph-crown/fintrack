// use fintrack::commands;
// use fintrack::command_prelude::*;

use clap::Command;
use fintrack::{GlobalContext, commands};

fn main() {
  let home_dir = match dirs::home_dir() {
    Some(path) => path,
    None => {
      panic!("Error");
    }
  };

  let mut gctx = GlobalContext::new(home_dir);
  // exec();
  // println!("Hello, world!");
  let matches = Command::new("fintrack")
    .bin_name("fintrack")
    .subcommand_required(true)
    .subcommands(commands::cli())
    .get_matches();

  let _ = match matches.subcommand() {
    Some((cmd, args)) => {
      if let Some(exec_fn) = commands::build_exec(cmd) {
        exec_fn(&mut gctx, args)
      } else {
        panic!("Error happened here");
      }
    }
    _ => {
      panic!("Error here too");
    }
  };
}

fn run() {}

fn exec() {}
