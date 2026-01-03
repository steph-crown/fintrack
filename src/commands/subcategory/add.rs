use clap::{Arg, ArgMatches, Command};

use crate::{
  CliError, CliResponse, CliResult, GlobalContext, TrackerData,
  parsers::parse_label,
  utils::file::{FilePath, write_json_to_file},
};

pub fn cli() -> Command {
  Command::new("add")
    .about("Create a new subcategory")
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

  // Normalize to lowercase for lookup, but store in Title Case
  let name_lower = name.to_lowercase();
  let name_title = {
    let mut chars = name_lower.chars();
    match chars.next() {
      None => return Err(CliError::Other("Invalid name".to_string())),
      Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
  };

  if tracker_data.subcategories_by_name.contains_key(&name_lower) {
    return Err(CliError::ValidationError(
      crate::ValidationErrorKind::SubcategoryAlreadyExists {
        name: name_title.clone(),
      },
    ));
  }

  // Check if trying to create "Miscellaneous" (system subcategory)
  if name_lower == "miscellaneous" {
    return Err(CliError::ValidationError(
      crate::ValidationErrorKind::CannotDeleteMiscellaneous,
    ));
  }

  let subcategory_id = tracker_data.next_subcategory_id as usize;
  tracker_data.subcategories_by_id.insert(subcategory_id, name_title.clone());
  tracker_data.subcategories_by_name.insert(name_lower, subcategory_id);
  tracker_data.next_subcategory_id += 1;
  tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse::new(crate::ResponseContent::Message(format!(
    "Subcategory '{}' added (ID: {})",
    name_title, subcategory_id
  ))))
}
