use chrono::NaiveDate;
use clap::ArgMatches;

use crate::{Category, CliError, Currency};

// Constants for default values
const DEFAULT_F64: f64 = 0.0;
const DEFAULT_USIZE: usize = 0;
const DEFAULT_SUBCATEGORY: &str = "miscellaneous";

/// Extension trait for `ArgMatches` providing convenient methods for extracting values
/// with sensible defaults and error handling.
pub trait ArgMatchesExt {
  // Required value extractors (return Result for error handling)
  fn get_category(&self, id: &str) -> Result<&Category, CliError>;
  fn get_usize(&self, id: &str) -> Result<usize, CliError>;

  // Optional value extractors (return Option)
  fn get_category_opt(&self, id: &str) -> Option<&Category>;
  fn get_f64_opt(&self, id: &str) -> Option<f64>;
  fn get_usize_opt(&self, id: &str) -> Option<usize>;
  fn get_string_opt(&self, id: &str) -> Option<String>;
  fn get_subcategory_opt(&self, id: &str) -> Option<String>;
  fn get_date_opt(&self, id: &str) -> Option<NaiveDate>;
  fn get_currency_opt(&self, id: &str) -> Option<&Currency>;

  // Value extractors with defaults
  fn get_f64_or_default(&self, id: &str) -> f64;
  fn get_usize_or_default(&self, id: &str) -> usize;
  fn get_string_or_default(&self, id: &str) -> String;
  fn get_subcategory_or_default(&self, id: &str) -> String;
  fn get_currency_or_default(&self, id: &str) -> &Currency;

  // Collection extractors
  fn get_vec<T: Clone + Send + Sync + 'static>(&self, id: &str) -> Vec<T>;

  // Check if argument was provided
  fn contains_id(&self, id: &str) -> bool;
}

impl ArgMatchesExt for ArgMatches {
  fn get_category(&self, id: &str) -> Result<&Category, CliError> {
    self
      .get_one::<Category>(id)
      .ok_or_else(|| {
        CliError::ValidationError(crate::ValidationErrorKind::InvalidCategoryName {
          name: id.to_string(),
          reason: "Category not provided".to_string(),
        })
      })
  }

  fn get_usize(&self, id: &str) -> Result<usize, CliError> {
    self
      .get_one::<usize>(id)
      .copied()
      .ok_or_else(|| CliError::Other(format!("Required argument '{}' not provided", id)))
  }

  fn get_category_opt(&self, id: &str) -> Option<&Category> {
    self.get_one::<Category>(id)
  }

  fn get_f64_opt(&self, id: &str) -> Option<f64> {
    self.get_one::<f64>(id).copied()
  }

  fn get_usize_opt(&self, id: &str) -> Option<usize> {
    self.get_one::<usize>(id).copied()
  }

  fn get_string_opt(&self, id: &str) -> Option<String> {
    self.get_one::<String>(id).cloned()
  }

  fn get_subcategory_opt(&self, id: &str) -> Option<String> {
    self.get_one::<String>(id).cloned()
  }

  fn get_date_opt(&self, id: &str) -> Option<NaiveDate> {
    self.get_one::<NaiveDate>(id).copied()
  }

  fn get_currency_opt(&self, id: &str) -> Option<&Currency> {
    self.get_one::<Currency>(id)
  }

  fn get_f64_or_default(&self, id: &str) -> f64 {
    self.get_one::<f64>(id).copied().unwrap_or(DEFAULT_F64)
  }

  fn get_usize_or_default(&self, id: &str) -> usize {
    self.get_one::<usize>(id).copied().unwrap_or(DEFAULT_USIZE)
  }

  fn get_string_or_default(&self, id: &str) -> String {
    self
      .get_one::<String>(id)
      .cloned()
      .unwrap_or_default()
  }

  fn get_subcategory_or_default(&self, id: &str) -> String {
    self
      .get_one::<String>(id)
      .cloned()
      .unwrap_or_else(|| DEFAULT_SUBCATEGORY.to_string())
  }

  fn get_currency_or_default(&self, id: &str) -> &Currency {
    self.get_one::<Currency>(id).unwrap_or(&Currency::NGN)
  }

  fn get_vec<T: Clone + Send + Sync + 'static>(&self, id: &str) -> Vec<T> {
    self
      .get_many::<T>(id)
      .map(|iter| iter.cloned().collect())
      .unwrap_or_default()
  }

  fn contains_id(&self, id: &str) -> bool {
    ArgMatches::contains_id(self, id)
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    fn create_test_command() -> Command {
        Command::new("test")
            .arg(clap::Arg::new("category").value_parser(crate::utils::parsers::parse_category))
            .arg(clap::Arg::new("amount").short('a').long("amount").value_parser(clap::value_parser!(f64)))
            .arg(clap::Arg::new("id").short('i').long("id").value_parser(clap::value_parser!(usize)))
            .arg(clap::Arg::new("text").short('t').long("text").value_parser(clap::value_parser!(String)))
            .arg(clap::Arg::new("date").short('D').long("date").value_parser(crate::utils::parsers::parse_date))
            .arg(clap::Arg::new("currency").short('c').long("currency").value_parser(clap::value_parser!(Currency)))
            .arg(clap::Arg::new("ids").long("ids").value_parser(clap::value_parser!(usize)).action(clap::ArgAction::Append))
    }

    #[test]
    fn test_get_category() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test", "income"]);

        let category = matches.get_category("category").unwrap();
        assert!(matches!(category, &Category::Income));
    }

    #[test]
    fn test_get_category_missing() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test"]);

        let result = matches.get_category("category");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_category_opt() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test", "expenses"]);

        let category = matches.get_category_opt("category").unwrap();
        assert!(matches!(category, &Category::Expenses));
    }

    #[test]
    fn test_get_category_opt_missing() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test"]);

        assert!(matches.get_category_opt("category").is_none());
    }

    #[test]
    fn test_get_f64_opt() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test", "--amount", "100.5"]);

        assert_eq!(matches.get_f64_opt("amount"), Some(100.5));
    }

    #[test]
    fn test_get_f64_or_default() {
        let cmd1 = create_test_command();
        let cmd2 = create_test_command();
        let matches1 = cmd1.get_matches_from(&["test", "--amount", "100.5"]);
        let matches2 = cmd2.get_matches_from(&["test"]);

        assert_eq!(matches1.get_f64_or_default("amount"), 100.5);
        assert_eq!(matches2.get_f64_or_default("amount"), 0.0);
    }

    #[test]
    fn test_get_usize() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test", "--id", "42"]);

        assert_eq!(matches.get_usize("id").unwrap(), 42);
    }

    #[test]
    fn test_get_usize_or_default() {
        let cmd1 = create_test_command();
        let cmd2 = create_test_command();
        let matches1 = cmd1.get_matches_from(&["test", "--id", "42"]);
        let matches2 = cmd2.get_matches_from(&["test"]);

        assert_eq!(matches1.get_usize_or_default("id"), 42);
        assert_eq!(matches2.get_usize_or_default("id"), 0);
    }

    #[test]
    fn test_get_string_or_default() {
        let cmd1 = create_test_command();
        let cmd2 = create_test_command();
        let matches1 = cmd1.get_matches_from(&["test", "--text", "hello"]);
        let matches2 = cmd2.get_matches_from(&["test"]);

        assert_eq!(matches1.get_string_or_default("text"), "hello");
        assert_eq!(matches2.get_string_or_default("text"), "");
    }

    #[test]
    fn test_get_subcategory_or_default() {
        let cmd1 = create_test_command();
        let cmd2 = create_test_command();
        let matches1 = cmd1.get_matches_from(&["test", "--text", "groceries"]);
        let matches2 = cmd2.get_matches_from(&["test"]);

        assert_eq!(matches1.get_subcategory_or_default("text"), "groceries");
        assert_eq!(matches2.get_subcategory_or_default("text"), "miscellaneous");
    }

    #[test]
    fn test_get_date_opt() {
        use chrono::Datelike;
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test", "--date", "15-01-2025"]);

        let date = matches.get_date_opt("date").unwrap();
        assert_eq!(date.day(), 15);
        assert_eq!(date.month(), 1);
        assert_eq!(date.year(), 2025);
    }

    #[test]
    fn test_get_currency_or_default() {
        let cmd1 = create_test_command();
        let cmd2 = create_test_command();
        let matches1 = cmd1.get_matches_from(&["test", "--currency", "usd"]);
        let matches2 = cmd2.get_matches_from(&["test"]);

        assert!(matches!(matches1.get_currency_or_default("currency"), &Currency::USD));
        assert!(matches!(matches2.get_currency_or_default("currency"), &Currency::NGN));
    }

    #[test]
    fn test_get_vec() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test", "--ids", "1", "--ids", "2", "--ids", "3"]);

        let ids: Vec<usize> = matches.get_vec("ids");
        assert_eq!(ids, vec![1, 2, 3]);
    }

    #[test]
    fn test_get_vec_empty() {
        let cmd = create_test_command();
        let matches = cmd.get_matches_from(&["test"]);

        let ids: Vec<usize> = matches.get_vec("ids");
        assert_eq!(ids, Vec::<usize>::new());
  }
}
