use chrono::NaiveDate;

/// Parse a date string in DD-MM-YYYY format
///
/// This is the standard date format used throughout the application.
/// Used as a clap value parser for date arguments.
pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
  NaiveDate::parse_from_str(s, "%d-%m-%Y")
    .map_err(|_| format!("'{}' is not in the format DD-MM-YYYY", s))
}
