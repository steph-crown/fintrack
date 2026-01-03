use crate::{CliError, output};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};

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

// #[derive(D)]
pub struct Total {
  pub currency: Currency,
  pub opening_balance: f64,
  pub income_total: f64,
  pub expenses_total: f64,
}

impl Total {
  pub fn total(&self) -> f64 {
    self.opening_balance + self.income_total + self.expenses_total
  }
}

pub struct DescribeData {
  pub total_records: usize,
  pub date_range: Option<(String, String)>,
  pub by_category: Vec<(String, usize, f64)>, // (name, count, total)
  pub by_subcategory: Vec<(String, usize, f64)>, // (name, count, total)
  pub average_transaction: f64,
  pub currency: Currency,
}

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

#[derive(clap::ValueEnum, Clone, Debug, strum::Display, strum::EnumString)]
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

#[derive(clap::ValueEnum, Clone, Debug, strum::Display, strum::EnumString)]
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
