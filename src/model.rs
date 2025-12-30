use crate::CliError;

pub struct CliResponse {
  pub success: bool,
  // stdout:
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
