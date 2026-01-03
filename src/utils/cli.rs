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
}
