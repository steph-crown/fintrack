use crate::{CliError, output};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};

#[derive(Debug)]
pub struct CliResponse {
  content: Option<ResponseContent>,
}

impl CliResponse {
  pub fn new(content: ResponseContent) -> Self {
    Self {
      content: Some(content),
    }
  }

  pub fn success() -> Self {
    Self { content: None }
  }

  pub fn content(&self) -> Option<&ResponseContent> {
    self.content.as_ref()
  }
}

impl CliResponse {
  /// Write this response to the given writer
  pub fn write_to(&self, writer: &mut impl std::io::Write) -> io::Result<()> {
    output::write_response(self, writer)
  }
}

#[derive(Debug)]
pub struct Total {
  pub currency: Currency,
  pub opening_balance: f64,
  pub income_total: f64,
  pub expenses_total: f64,
}

impl Total {
  pub fn total(&self) -> f64 {
    self.opening_balance + self.income_total - self.expenses_total
  }
}

#[derive(Debug)]
pub struct DescribeData {
  pub total_records: usize,
  pub date_range: Option<(String, String)>,
  pub by_category: Vec<(String, usize, f64)>, // (name, count, total)
  pub by_subcategory: Vec<(String, usize, f64)>, // (name, count, total)
  pub average_transaction: f64,
  pub currency: Currency,
}

#[derive(Debug)]
pub enum ResponseContent {
  Message(String),
  Record {
    record: Record,
    tracker_data: TrackerData,
    is_update: bool,
  },
  List { records: Vec<Record>, tracker_data: TrackerData },
  TrackerData(TrackerData),
  Total(Total),
  Categories(Vec<(usize, String)>),
  Subcategories(Vec<(usize, String)>),
  Describe(DescribeData),
}

pub type CliResult = Result<CliResponse, CliError>;

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "UPPERCASE", ascii_case_insensitive)]
pub enum Currency {
  NGN,
  USD,
  GBP,
  EUR,
  CAD,
  AUD,
  JPY,
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
  pub id: usize,
  pub category: usize,    // ID from categories map
  pub subcategory: usize, // ID from subcategories map
  pub description: String,
  pub amount: f64,  // Always positive; sign determined by category
  pub date: String, // Format: DD-MM-YYYY
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrackerData {
  pub version: u32,
  pub currency: String,
  pub created_at: String,
  pub last_modified: String,
  pub opening_balance: f64,
  pub categories: HashMap<String, usize>,
  pub subcategories_by_id: HashMap<usize, String>,
  pub subcategories_by_name: HashMap<String, usize>,
  pub next_subcategory_id: u32,
  pub records: Vec<Record>,
  pub next_record_id: usize,
}

impl TrackerData {
  pub fn push_record(&mut self, record: Record) -> &Self {
    self.records.push(record);

    self
  }

  pub fn category_id(&self, category: &str) -> usize {
    self.categories[category]
  }

  pub fn miscellaneous_subcategory_id(&self) -> Option<usize> {
    self.subcategories_by_name.get("miscellaneous").copied()
  }

  pub fn subcategory_id(&self, name: &str) -> Option<usize> {
    self.subcategories_by_name.get(name).copied()
  }

  pub fn category_name(&self, id: usize) -> Option<&String> {
    self.categories.iter().find(|(_, v)| **v == id).map(|(k, _)| k)
  }

  pub fn subcategory_name(&self, id: usize) -> Option<&String> {
    self.subcategories_by_id.get(&id)
  }

  pub fn totals(&self) -> (f64, f64) {
    self.records.iter().fold((0.0, 0.0), |mut acc, r| {
      if r.category == 1 {
        acc.0 += r.amount;
      } else {
        acc.1 += r.amount;
      }

      acc
    })
  }
}

pub fn default_tracker_json(currency: &Currency, opening_balance: f64) -> serde_json::Value {
  serde_json::json!({
      "version": 1,
      "currency": currency.to_string(),
      "opening_balance": opening_balance,
      "created_at": chrono::Utc::now().to_rfc3339(),
      "last_modified": chrono::Utc::now().to_rfc3339(),
      "categories": {
          "income": 1,
          "expenses": 2
      },
      "subcategories_by_id": {
          "1": "miscellaneous"
      },
      "subcategories_by_name": {
          "miscellaneous": 1
      },
      "records": [],
      "next_record_id": 1,
      "next_subcategory_id": 2
  })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tracker_data() -> TrackerData {
        let mut categories = HashMap::new();
        categories.insert("income".to_string(), 1);
        categories.insert("expenses".to_string(), 2);

        let mut subcategories_by_id = HashMap::new();
        subcategories_by_id.insert(1, "miscellaneous".to_string());

        let mut subcategories_by_name = HashMap::new();
        subcategories_by_name.insert("miscellaneous".to_string(), 1);

        TrackerData {
            version: 1,
            currency: "USD".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
            last_modified: "2025-01-01T00:00:00Z".to_string(),
            opening_balance: 1000.0,
            categories,
            subcategories_by_id,
            subcategories_by_name,
            next_subcategory_id: 2,
            records: Vec::new(),
            next_record_id: 1,
        }
    }

    #[test]
    fn test_push_record() {
        let mut tracker = create_test_tracker_data();
        let record = Record {
            id: 1,
            category: 1,
            subcategory: 1,
            description: "Test".to_string(),
            amount: 100.0,
            date: "01-01-2025".to_string(),
        };

        tracker.push_record(record.clone());
        assert_eq!(tracker.records.len(), 1);
        assert_eq!(tracker.records[0].id, 1);
    }

    #[test]
    fn test_category_id() {
        let tracker = create_test_tracker_data();
        assert_eq!(tracker.category_id("income"), 1);
        assert_eq!(tracker.category_id("expenses"), 2);
    }

    #[test]
    fn test_subcategory_id() {
        let tracker = create_test_tracker_data();
        assert_eq!(tracker.subcategory_id("miscellaneous"), Some(1));
        assert_eq!(tracker.subcategory_id("nonexistent"), None);
    }

    #[test]
    fn test_miscellaneous_subcategory_id() {
        let tracker = create_test_tracker_data();
        assert_eq!(tracker.miscellaneous_subcategory_id(), Some(1));
    }

    #[test]
    fn test_category_name() {
        let tracker = create_test_tracker_data();
        assert_eq!(tracker.category_name(1), Some(&"income".to_string()));
        assert_eq!(tracker.category_name(2), Some(&"expenses".to_string()));
        assert_eq!(tracker.category_name(999), None);
    }

    #[test]
    fn test_subcategory_name() {
        let mut tracker = create_test_tracker_data();
        tracker.subcategories_by_id.insert(2, "groceries".to_string());

        assert_eq!(tracker.subcategory_name(1), Some(&"miscellaneous".to_string()));
        assert_eq!(tracker.subcategory_name(2), Some(&"groceries".to_string()));
        assert_eq!(tracker.subcategory_name(999), None);
    }

    #[test]
    fn test_totals_empty() {
        let tracker = create_test_tracker_data();
        let (income, expenses) = tracker.totals();
        assert_eq!(income, 0.0);
        assert_eq!(expenses, 0.0);
    }

    #[test]
    fn test_totals_with_records() {
        let mut tracker = create_test_tracker_data();

        tracker.records.push(Record {
            id: 1,
            category: 1, // income
            subcategory: 1,
            description: "Salary".to_string(),
            amount: 500.0,
            date: "01-01-2025".to_string(),
        });

        tracker.records.push(Record {
            id: 2,
            category: 2, // expenses
            subcategory: 1,
            description: "Food".to_string(),
            amount: 100.0,
            date: "02-01-2025".to_string(),
        });

        tracker.records.push(Record {
            id: 3,
            category: 1, // income
            subcategory: 1,
            description: "Bonus".to_string(),
            amount: 200.0,
            date: "03-01-2025".to_string(),
        });

        let (income, expenses) = tracker.totals();
        assert_eq!(income, 700.0);
        assert_eq!(expenses, 100.0);
    }

    #[test]
    fn test_currency_display() {
        assert_eq!(Currency::USD.to_string(), "USD");
        assert_eq!(Currency::NGN.to_string(), "NGN");
        assert_eq!(Currency::GBP.to_string(), "GBP");
    }

    #[test]
    fn test_currency_from_str() {
        assert_eq!("USD".parse::<Currency>().unwrap(), Currency::USD);
        assert_eq!("usd".parse::<Currency>().unwrap(), Currency::USD);
        assert_eq!("Usd".parse::<Currency>().unwrap(), Currency::USD);
        assert!("INVALID".parse::<Currency>().is_err());
    }

    #[test]
    fn test_category_display() {
        assert_eq!(Category::Income.to_string(), "income");
        assert_eq!(Category::Expenses.to_string(), "expenses");
    }

    #[test]
    fn test_category_from_str() {
        assert_eq!("income".parse::<Category>().unwrap(), Category::Income);
        assert_eq!("Income".parse::<Category>().unwrap(), Category::Income);
        assert_eq!("INCOME".parse::<Category>().unwrap(), Category::Income);
        assert_eq!("expenses".parse::<Category>().unwrap(), Category::Expenses);
        assert!("invalid".parse::<Category>().is_err());
    }

    #[test]
    fn test_total_calculation() {
        let total = Total {
            currency: Currency::USD,
            opening_balance: 1000.0,
            income_total: 500.0,
            expenses_total: 200.0,
        };

        assert_eq!(total.total(), 1300.0); // 1000 + 500 - 200
    }

    #[test]
    fn test_default_tracker_json() {
        let json = default_tracker_json(&Currency::USD, 1000.0);

        assert_eq!(json["currency"], "USD");
        assert_eq!(json["opening_balance"], 1000.0);
        assert_eq!(json["categories"]["income"], 1);
        assert_eq!(json["categories"]["expenses"], 2);
        assert_eq!(json["subcategories_by_name"]["miscellaneous"], 1);
        assert_eq!(json["records"].as_array().unwrap().len(), 0);
        assert_eq!(json["next_record_id"], 1);
        assert_eq!(json["next_subcategory_id"], 2);
    }
}
