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

impl std::fmt::Display for Currency {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Currency::NGN => write!(f, "NGN"),
      Currency::USD => write!(f, "USD"),
      Currency::GBP => write!(f, "GBP"),
      Currency::EUR => write!(f, "EUR"),
      Currency::CAD => write!(f, "CAD"),
      Currency::AUD => write!(f, "AUD"),
      Currency::JPY => write!(f, "JPY"),
    }
  }
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Category {
  Income,
  Expenses,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ExportFileType {
  JSON,
  PDF,
  CSV,
}

// src/models.rs

pub fn default_tracker_json(currency: &Currency) -> serde_json::Value {
  serde_json::json!({
      "version": 1,
      "currency": currency.to_string(),
      "created_at": chrono::Utc::now().to_rfc3339(),
      "last_modified": chrono::Utc::now().to_rfc3339(),
      "categories": {
          "Income": 1,
          "Expenses": 2
      },
      "subcategories_by_id": {
          "1": "Miscellaneous"
      },
      "subcategories_by_name": {
          "Miscellaneous": 1
      },
      "next_subcategory_id": 2,
      "records": [],
      "next_record_id": 1
  })
}
