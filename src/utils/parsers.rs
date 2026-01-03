use chrono::NaiveDate;
use strum::EnumString;

use crate::Category;

/// Parse a date string in DD-MM-YYYY format
///
/// This is the standard date format used throughout the application.
/// Used as a clap value parser for date arguments.
pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
  NaiveDate::parse_from_str(s, "%d-%m-%Y")
    .map_err(|_| format!("'{}' is not in the format DD-MM-YYYY", s))
}

/// Parse a category string case-insensitively
///
/// Accepts "income", "Income", "INCOME", "expenses", "Expenses", "EXPENSES", etc.
/// Used as a clap value parser for category arguments.
pub fn parse_category(s: &str) -> Result<Category, String> {
  s.parse::<Category>()
    .map_err(|_| format!("'{}' is not a valid category. Use 'income' or 'expenses'", s))
}

/// Parse a label string. Used for categories and subcategories
pub fn parse_label(s: &str) -> Result<String, String> {
  if s.is_empty() {
    return Err("Input cannot be empty".to_string());
  }

  // 1. Must start with a letter (No numbers or underscores at the start)
  if !s.chars().next().unwrap().is_ascii_alphabetic() {
    return Err(format!("'{}' must start with a letter", s));
  }

  // 2. Rest can be Alphanumeric OR Underscore
  if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
    return Err(format!(
      "'{}' contains invalid symbols (only letters, numbers, and underscores allowed)",
      s
    ));
  }

  Ok(s.to_string())
}
