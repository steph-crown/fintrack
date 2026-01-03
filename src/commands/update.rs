use clap::{Arg, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::utils::parsers::{parse_category, parse_date};
use crate::{CliError, CliResponse, CliResult, GlobalContext, ResponseContent, TrackerData};

pub fn cli() -> Command {
  Command::new("update")
    .about("Modify an existing transaction record")
    .long_about("Updates one or more fields of an existing record. Only the fields you specify will be changed; others remain unchanged. Use 'fintrack list' to find record IDs.")
    .arg(
      Arg::new("record_id")
        .index(1)
        .required(true)
        .value_parser(clap::value_parser!(usize))
        .help("The ID of the record to update")
        .long_help("The unique ID number of the record you want to modify. Use 'fintrack list' to see all records and their IDs."),
    )
    .arg(
      Arg::new("category")
        .short('c')
        .long("category")
        .value_parser(parse_category)
        .help("Change the category to 'income' or 'expenses'")
        .long_help("Updates the transaction category. Use 'income' for money received or 'expenses' for money spent. Case-insensitive."),
    )
    .arg(
      Arg::new("amount")
        .short('a')
        .long("amount")
        .value_parser(clap::value_parser!(f64))
        .help("Change the transaction amount (must be greater than 0)")
        .long_help("Updates the transaction amount. Must be a positive number greater than 0. Examples: 100, 150.50, 2000.75"),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String))
        .help("Change the subcategory name")
        .long_help("Updates the subcategory. The subcategory must already exist - use 'fintrack subcategory list' to see available subcategories."),
    )
    .arg(
      Arg::new("description")
        .short('d')
        .long("description")
        .value_parser(clap::value_parser!(String))
        .help("Change the description or notes")
        .long_help("Updates the transaction description or notes. You can set this to an empty string to remove the description."),
    )
    .arg(
      Arg::new("date")
        .short('D')
        .long("date")
        .value_parser(parse_date)
        .help("Change the transaction date (DD-MM-YYYY format)")
        .long_help("Updates the transaction date. Format: DD-MM-YYYY (e.g., 30-12-2025)."),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let mut file = gctx.tracker_path().open_read_write()?;
  let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let record_id = args
    .get_usize("record_id")
    .map_err(|_| CliError::ValidationError(crate::ValidationErrorKind::RecordNotFound { id: 0 }))?;

  let category_id = args.get_category_opt("category").map(|category| {
    let category_str = category.to_string();
    tracker_data.category_id(&category_str)
  });

  let subcategory_id = args
    .get_subcategory_opt("subcategory")
    .map(|name| {
      tracker_data.subcategory_id(&name).ok_or_else(|| {
        CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound { name })
      })
    })
    .transpose()?;

  let record = tracker_data
    .records
    .iter_mut()
    .find(|r| r.id == record_id)
    .ok_or_else(|| {
      CliError::ValidationError(crate::ValidationErrorKind::RecordNotFound { id: record_id })
    })?;

  if let Some(cat_id) = category_id {
    record.category = cat_id;
  }

  if let Some(amount) = args.get_f64_opt("amount") {
    if amount <= 0.0 {
      return Err(CliError::ValidationError(
        crate::ValidationErrorKind::AmountTooSmall { amount },
      ));
    }
    record.amount = amount;
  }

  if let Some(subcat_id) = subcategory_id {
    record.subcategory = subcat_id;
  }

  if let Some(description) = args.get_string_opt("description") {
    record.description = description;
  }

  if let Some(date) = args.get_date_opt("date") {
    record.date = date.format("%d-%m-%Y").to_string();
  }

  tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

  let updated_record = record.clone();

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse::new(ResponseContent::Record {
    record: updated_record,
    tracker_data,
    is_update: true,
  }))
}
