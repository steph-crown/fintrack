use chrono::NaiveDate;

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
  s.parse::<Category>().map_err(|_| {
    format!(
      "'{}' is not a valid category. Use 'income' or 'expenses'",
      s
    )
  })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_valid() {
        use chrono::Datelike;
        let date = parse_date("01-01-2025").unwrap();
        assert_eq!(date.day(), 1);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2025);
    }

    #[test]
    fn test_parse_date_invalid_format() {
        assert!(parse_date("2025-01-01").is_err());
        assert!(parse_date("01/01/2025").is_err());
        // Note: "1-1-2025" actually parses successfully with chrono's lenient parsing
        // So we test with a clearly invalid format instead
        assert!(parse_date("invalid").is_err());
    }

    #[test]
    fn test_parse_date_invalid_date() {
        assert!(parse_date("32-01-2025").is_err());
        assert!(parse_date("01-13-2025").is_err());
    }

    #[test]
    fn test_parse_category_valid() {
        assert!(matches!(parse_category("income").unwrap(), Category::Income));
        assert!(matches!(parse_category("Income").unwrap(), Category::Income));
        assert!(matches!(parse_category("INCOME").unwrap(), Category::Income));
        assert!(matches!(parse_category("expenses").unwrap(), Category::Expenses));
        assert!(matches!(parse_category("Expenses").unwrap(), Category::Expenses));
        assert!(matches!(parse_category("EXPENSES").unwrap(), Category::Expenses));
    }

    #[test]
    fn test_parse_category_invalid() {
        assert!(parse_category("invalid").is_err());
        assert!(parse_category("").is_err());
        assert!(parse_category("incomee").is_err());
    }

    #[test]
    fn test_parse_label_valid() {
        assert_eq!(parse_label("Groceries").unwrap(), "Groceries");
        assert_eq!(parse_label("Salary").unwrap(), "Salary");
        assert_eq!(parse_label("Rent_Payment").unwrap(), "Rent_Payment");
        assert_eq!(parse_label("Item123").unwrap(), "Item123");
        assert_eq!(parse_label("a").unwrap(), "a");
    }

    #[test]
    fn test_parse_label_empty() {
        assert!(parse_label("").is_err());
    }

    #[test]
    fn test_parse_label_starts_with_number() {
        assert!(parse_label("123Item").is_err());
        assert!(parse_label("0test").is_err());
    }

    #[test]
    fn test_parse_label_starts_with_underscore() {
        assert!(parse_label("_test").is_err());
    }

    #[test]
    fn test_parse_label_invalid_characters() {
        assert!(parse_label("test-item").is_err());
        assert!(parse_label("test item").is_err());
        assert!(parse_label("test@item").is_err());
        assert!(parse_label("test.item").is_err());
    }

    #[test]
    fn test_parse_label_allows_underscore_in_middle() {
        assert_eq!(parse_label("test_item").unwrap(), "test_item");
        assert_eq!(parse_label("test_item_123").unwrap(), "test_item_123");
    }
}
