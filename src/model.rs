use crate::{CliError, output};

pub struct CliResponse {
  pub success: bool,
  // stdout:
}

impl CliResponse {
  /// Write this response to the given writer
  pub fn write_to(&self, writer: &mut impl std::io::Write) {
    output::write_response(self, writer);
  }
}

pub type CliResult = Result<CliResponse, CliError>;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Currency {
  NGN,
  USD,
  GBP,
  EUR,
  CAD,
  AUD,
  JPY,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Category {
  Income,
  Expenses,
}
