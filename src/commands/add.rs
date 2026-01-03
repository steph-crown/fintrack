use chrono::Local;
use clap::{Arg, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::utils::parsers::{parse_category, parse_date};
use crate::{
  CliError, CliResponse, CliResult, GlobalContext, Record, ResponseContent, TrackerData,
};

pub fn cli() -> Command {
  Command::new("add")
    .about("Record a new income or expense transaction")
    .long_about("Adds a new financial record to your tracker. Category and amount are required. The amount must be greater than 0.")
    .arg(
      Arg::new("category")
        .index(1)
        .required(true)
        .value_parser(parse_category)
        .help("Transaction category: 'income' or 'expenses' (case-insensitive)")
        .long_help("The type of transaction. Use 'income' for money received or 'expenses' for money spent. Case-insensitive (Income, INCOME, income all work)."),
    )
    .arg(
      Arg::new("amount")
        .index(2)
        .required(true)
        .value_parser(clap::value_parser!(f64))
        .help("Transaction amount (must be greater than 0)")
        .long_help("The amount of money for this transaction. Must be a positive number greater than 0. Examples: 100, 150.50, 2000.75"),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String))
        .default_value("miscellaneous")
        .help("Subcategory name for this transaction")
        .long_help("A more specific category for this transaction (e.g., 'Groceries', 'Salary', 'Rent'). Must already exist - use 'fintrack subcategory list' to see available subcategories. Defaults to 'miscellaneous' if not specified."),
    )
    .arg(
      Arg::new("description")
        .short('d')
        .long("description")
        .value_parser(clap::value_parser!(String))
        .help("Optional description or notes for this transaction")
        .long_help("Any additional notes or description you want to add to this transaction. This is optional and can be left empty."),
    )
    .arg(
      Arg::new("date")
        .short('D')
        .long("date")
        .value_parser(parse_date)
        .help("Transaction date in DD-MM-YYYY format")
        .long_help("The date when this transaction occurred. Format: DD-MM-YYYY (e.g., 30-12-2025). Defaults to today's date if not specified."),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let mut file = gctx.tracker_path().open_read_write()?;
  let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let category = args.get_category("category")?;
  let amount = args.get_f64_or_default("amount");

  if amount <= 0.0 {
    return Err(CliError::ValidationError(
      crate::ValidationErrorKind::AmountTooSmall { amount },
    ));
  }

  let subcategory_name = args.get_subcategory_or_default("subcategory");
  let description = args.get_string_or_default("description");

  let category_str = category.to_string();
  let category_id = tracker_data.category_id(&category_str);

  let subcategory_id = tracker_data
    .subcategory_id(&subcategory_name)
    .ok_or_else(|| {
      CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
        name: subcategory_name,
      })
    })?;

  let date = args
    .get_date_opt("date")
    .map(|d| d.format("%d-%m-%Y").to_string())
    .unwrap_or_else(|| Local::now().format("%d-%m-%Y").to_string());

  let record_id = tracker_data.next_record_id;
  let record = Record {
    id: record_id,
    category: category_id,
    amount,
    subcategory: subcategory_id,
    description,
    date,
  };

  tracker_data.next_record_id += 1;
  tracker_data.last_modified = chrono::Utc::now().to_rfc3339();
  tracker_data.push_record(record.clone());

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse::new(ResponseContent::Record {
    record,
    tracker_data,
    is_update: false,
  }))
}
