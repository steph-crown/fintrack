use std::io;

use clap::Command;
use fintrack::{GlobalContext, commands};

fn main() {
  let exit_code = match run() {
    Ok(_) => 0,
    Err(e) => {
      eprintln!("Error: {}", e);
      1
    }
  };
  std::process::exit(exit_code);
}

fn run() -> Result<(), String> {
  let home_dir =
    dirs::home_dir().ok_or_else(|| "Failed to determine home directory".to_string())?;

  let mut gctx = GlobalContext::new(home_dir);

  let matches = Command::new("fintrack")
    .bin_name("fintrack")
    .about("A local-first CLI financial tracker for managing income and expenses")
    .version("1.0.0")
    .subcommand_required(true)
    .subcommands(commands::cli())
    .get_matches();

  let (cmd, args) = matches
    .subcommand()
    .expect("subcommand required but not found");

  let exec_fn = commands::build_exec(cmd).ok_or_else(|| format!("Unknown command: {}", cmd))?;

  let exec_result = exec_fn(&mut gctx, args);
  // the error expected here is not CliError, it is an io error that occured as CliResponse or CliError is being written to stdout
  process_result(&exec_result).expect("An error occured displaying response");

  Ok(())
}

fn process_result(result: &fintrack::CliResult) -> io::Result<()> {
  match result {
    Ok(res) => res.write_to(&mut std::io::stdout()),
    Err(err) => err.write_to(&mut std::io::stderr()),
  }
}
