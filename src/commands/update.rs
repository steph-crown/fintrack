use chrono::Local;
use clap::{Arg, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::parsers::parse_date;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::{Category, CliError, CliResponse, CliResult, GlobalContext, TrackerData};

pub fn cli() -> Command {
  Command::new("update")
    .about("Modify an existing record")
    .arg(
      Arg::new("record_id")
        .index(1)
        .required(true)
        .value_parser(clap::value_parser!(usize)),
    )
    .arg(
      Arg::new("category")
        .short('c')
        .long("category")
        .value_parser(clap::value_parser!(Category)),
    )
    .arg(
      Arg::new("amount")
        .short('a')
        .long("amount")
        .value_parser(clap::value_parser!(f64)),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String)),
    )
    .arg(
      Arg::new("description")
        .short('d')
        .long("description")
        .value_parser(clap::value_parser!(String)),
    )
    .arg(
      Arg::new("date")
        .short('D')
        .long("date")
        .value_parser(parse_date),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let mut file = gctx.tracker_path().open_read_write()?;
  let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let record_id = args.get_one::<usize>("record_id").copied().ok_or_else(|| {
    CliError::ValidationError(crate::ValidationErrorKind::RecordNotFound { id: 0 })
  })?;

  let category_id = args.value_of_category_opt("category").map(|category| {
    let category_str = category.to_string();
    tracker_data.category_id(&category_str)
  });

  let subcategory_id = args
    .value_of_subcategory_opt("subcategory")
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

  if let Some(&amount) = args.value_of_f64_opt("amount") {
    record.amount = amount;
  }

  if let Some(subcat_id) = subcategory_id {
    record.subcategory = subcat_id;
  }

  if let Some(description) = args.value_of_string_opt("description") {
    record.description = description;
  }

  if let Some(date) = args.value_of_date("date") {
    record.date = date;
  }

  tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse::success())
}
