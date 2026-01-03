use clap::{Arg, ArgMatches, Command};

use crate::{
  CliError, CliResponse, CliResult, GlobalContext, TrackerData,
  parsers::parse_label,
  utils::file::{FilePath, write_json_to_file},
};

pub fn cli() -> Command {
  Command::new("delete")
    .about("Delete a subcategory (only if it has no records)")
    .arg(
      Arg::new("name")
        .index(1)
        .required(true)
        .value_parser(parse_label)
        .help("Name of subcategory"),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let mut file = gctx.tracker_path().open_read_write()?;
  let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let name = args
    .get_one::<String>("name")
    .ok_or_else(|| CliError::Other("Subcategory name not provided".to_string()))?;

  let name_lower = name.to_lowercase();

  if name_lower == "miscellaneous" {
    return Err(CliError::ValidationError(
      crate::ValidationErrorKind::CannotDeleteMiscellaneous,
    ));
  }

  let subcategory_id = tracker_data
    .subcategory_id(&name_lower)
    .ok_or_else(|| {
      CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
        name: name.to_string(),
      })
    })?;

  let record_count = tracker_data
    .records
    .iter()
    .filter(|r| r.subcategory == subcategory_id)
    .count();

  if record_count > 0 {
    return Err(CliError::ValidationError(
      crate::ValidationErrorKind::SubcategoryHasRecords {
        name: name.to_string(),
        count: record_count,
      },
    ));
  }

  tracker_data.subcategories_by_id.remove(&subcategory_id);
  tracker_data.subcategories_by_name.remove(&name_lower);
  tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse::new(crate::ResponseContent::Message(format!(
    "Subcategory '{}' deleted",
    name
  ))))
}
