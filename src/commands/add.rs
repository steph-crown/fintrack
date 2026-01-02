use chrono::Local;
use clap::{Arg, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::parsers::parse_date;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::{Category, CliError, CliResponse, CliResult, GlobalContext, Record, TrackerData};

pub fn cli() -> Command {
  Command::new("add")
    .about("Record a new income or expense transaction")
    .arg(
      Arg::new("category")
        .index(1)
        .required(true)
        .value_parser(clap::value_parser!(Category)),
    )
    .arg(
      Arg::new("amount")
        .index(2)
        .required(true)
        .value_parser(clap::value_parser!(f64)),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String))
        .default_value("miscellaneous"),
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

  let category = args.value_of_category("category")?;
  let amount = *args.value_of_f64_or_zero("amount");
  let subcategory_name = args.value_of_subcategory("subcategory");
  let description = args.value_of_string("description");

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
    .value_of_date("date")
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
  tracker_data.push_record(record);

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse { success: true })
}
