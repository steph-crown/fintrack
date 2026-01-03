use chrono::NaiveDate;
use clap::{ArgMatches, Command};

use crate::{
  CliError, CliResponse, CliResult, Currency, DescribeData, GlobalContext, TrackerData,
  utils::file::FilePath,
};

pub fn cli() -> Command {
  Command::new("describe")
    .about("Show financial insights and statistics")
    .long_about("Provides an overview of your financial data including total records, date range, spending breakdown by category and subcategory, and average transaction amount. Includes visual charts for quick understanding of your spending patterns.")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let total_records = tracker_data.records.len();

  // Calculate date range
  let date_range = if !tracker_data.records.is_empty() {
    let mut dates: Vec<NaiveDate> = tracker_data
      .records
      .iter()
      .filter_map(|r| NaiveDate::parse_from_str(&r.date, "%d-%m-%Y").ok())
      .collect();

    if !dates.is_empty() {
      dates.sort();
      let earliest_str = dates.first().unwrap().format("%d-%m-%Y").to_string();
      let latest_str = dates.last().unwrap().format("%d-%m-%Y").to_string();
      Some((earliest_str, latest_str))
    } else {
      None
    }
  } else {
    None
  };

  // Calculate by category
  let mut category_stats: std::collections::HashMap<usize, (usize, f64)> =
    std::collections::HashMap::new();
  for record in &tracker_data.records {
    let entry = category_stats.entry(record.category).or_insert((0, 0.0));
    entry.0 += 1;
    entry.1 += record.amount;
  }

  let mut by_category: Vec<(String, usize, f64)> = category_stats
    .iter()
    .filter_map(|(&id, &(count, total))| {
      tracker_data
        .category_name(id)
        .map(|name| (name.clone(), count, total))
    })
    .collect();
  by_category.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

  // Calculate by subcategory
  let mut subcategory_stats: std::collections::HashMap<usize, (usize, f64)> =
    std::collections::HashMap::new();

  for record in &tracker_data.records {
    let entry = subcategory_stats
      .entry(record.subcategory)
      .or_insert((0, 0.0));
    entry.0 += 1;
    entry.1 += record.amount;
  }

  let mut by_subcategory: Vec<(String, usize, f64)> = subcategory_stats
    .iter()
    .filter_map(|(&id, &(count, total))| {
      tracker_data
        .subcategory_name(id)
        .map(|name| (name.clone(), count, total))
    })
    .collect();
  by_subcategory.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

  // Calculate average transaction
  let average_transaction = if total_records > 0 {
    tracker_data.records.iter().map(|r| r.amount).sum::<f64>() / total_records as f64
  } else {
    0.0
  };

  let currency = tracker_data
    .currency
    .parse::<Currency>()
    .map_err(|e| CliError::Other(format!("Invalid currency in tracker data: {}", e)))?;

  Ok(CliResponse::new(crate::ResponseContent::Describe(
    DescribeData {
      total_records,
      date_range,
      by_category,
      by_subcategory,
      average_transaction,
      currency,
    },
  )))
}
