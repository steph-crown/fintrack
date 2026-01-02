use crate::{CliError, CliResponse};

/// Write a CLI error to the given writer
pub fn write_error(err: &CliError, writer: &mut impl std::io::Write) {
  // TODO: Implement error formatting
  let _ = writeln!(writer, "Error: {:?}", err);
}

/// Write a CLI response to the given writer
pub fn write_response(res: &CliResponse, writer: &mut impl std::io::Write) {
  // TODO: Implement response formatting
  let _ = writeln!(writer, "Success");
}
