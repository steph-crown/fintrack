use clap::Command;
use fintrack::{commands, GlobalContext};

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
    .subcommand_required(true)
    .subcommands(commands::cli())
    .get_matches();

  let (cmd, args) = matches
    .subcommand()
    .expect("subcommand required but not found");

  let exec_fn = commands::build_exec(cmd).ok_or_else(|| format!("Unknown command: {}", cmd))?;

  let exec_result = exec_fn(&mut gctx, args);
  // .map_err(|_| "Command execution failed".to_string())?;
  process_result(&exec_result);

  Ok(())
}

fn process_result(result: &fintrack::CliResult) {
  match result {
    Ok(res) => res.write_to(&mut std::io::stdout()),
    Err(err) => err.write_to(&mut std::io::stderr()),
  }
}
