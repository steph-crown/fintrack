use clap::ArgMatches;

use crate::{Category, CliError, Currency};

pub trait ArgMatchesExt {
  fn value_of_f64_or_zero(&self, id: &str) -> &f64;
  fn value_of_subcategory(&self, id: &str) -> String;
  fn value_of_string(&self, id: &str) -> String;
  fn value_of_currency_or_def(&self, id: &str) -> &Currency;
  fn value_of_category(&self, id: &str) -> Result<&Category, CliError>;
  fn value_of_date(&self, id: &str) -> Option<String>;

  // Optional versions for update command
  fn value_of_category_opt(&self, id: &str) -> Option<&Category>;
  fn value_of_f64_opt(&self, id: &str) -> Option<&f64>;
  fn value_of_subcategory_opt(&self, id: &str) -> Option<String>;
  fn value_of_string_opt(&self, id: &str) -> Option<String>;
}

impl<'a> ArgMatchesExt for ArgMatches {
  fn value_of_f64_or_zero(&self, id: &str) -> &f64 {
    self.get_one::<f64>(id).unwrap_or(&0.0)
  }

  fn value_of_subcategory(&self, id: &str) -> String {
    self
      .get_one::<String>(id)
      .map(|x| x.to_string())
      .unwrap_or("miscellaneous".to_string())
  }

  fn value_of_string(&self, id: &str) -> String {
    self
      .get_one::<String>(id)
      .map(|x| x.to_string())
      .unwrap_or("".to_string())
  }

  fn value_of_currency_or_def(&self, id: &str) -> &Currency {
    self.get_one::<Currency>(id).unwrap_or(&Currency::NGN)
  }

  fn value_of_category(&self, id: &str) -> Result<&Category, CliError> {
    self
      .get_one::<Category>(id)
      .ok_or_else(|| CliError::Other("[category] not passed".to_string()))
  }

  fn value_of_date(&self, id: &str) -> Option<String> {
    self
      .get_one::<chrono::NaiveDate>(id)
      .map(|d| d.format("%d-%m-%Y").to_string())
  }

  fn value_of_category_opt(&self, id: &str) -> Option<&Category> {
    self.get_one::<Category>(id)
  }

  fn value_of_f64_opt(&self, id: &str) -> Option<&f64> {
    self.get_one::<f64>(id)
  }

  fn value_of_subcategory_opt(&self, id: &str) -> Option<String> {
    self.get_one::<String>(id).map(|x| x.to_string())
  }

  fn value_of_string_opt(&self, id: &str) -> Option<String> {
    self.get_one::<String>(id).map(|x| x.to_string())
  }
}
