use clap::ArgMatches;

use crate::Currency;

pub trait ArgMatchesExt {
  fn value_of_f64_or_zero(&self, id: &str) -> &f64;
  fn value_of_currency_or_def(&self, id: &str) -> &Currency;
}

impl<'a> ArgMatchesExt for ArgMatches {
  fn value_of_f64_or_zero(&self, id: &str) -> &f64 {
    self.get_one::<f64>(id).unwrap_or(&0.0)
  }

  fn value_of_currency_or_def(&self, id: &str) -> &Currency {
    self.get_one::<Currency>(id).unwrap_or(&Currency::NGN)
  }
}
