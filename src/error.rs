use crate::output;

#[derive(Debug)]
pub struct CliError {}

impl CliError {
  /// Write this error to the given writer
  pub fn write_to(&self, writer: &mut impl std::io::Write) {
    output::write_error(self, writer);
  }
}
