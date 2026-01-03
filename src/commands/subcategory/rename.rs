use clap::{Arg, ArgMatches, Command};

use crate::{
  CliError, CliResponse, CliResult, GlobalContext, TrackerData,
  parsers::parse_label,
  utils::file::{FilePath, write_json_to_file},
};

pub fn cli() -> Command {
  Command::new("rename")
    .about("Rename an existing subcategory")
    .arg(
      Arg::new("old")
        .help("Current subcategory name")
        .index(1)
        .required(true)
        .value_parser(parse_label),
    )
    .arg(
      Arg::new("new")
        .help("The name you want to change subcategory to")
        .index(2)
        .required(true)
        .value_parser(parse_label),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let mut file = gctx.tracker_path().open_read_write()?;
  let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let old_name = args
    .get_one::<String>("old")
    .ok_or_else(|| CliError::Other("Old subcategory name not provided".to_string()))?;
  let new_name = args
    .get_one::<String>("new")
    .ok_or_else(|| CliError::Other("New subcategory name not provided".to_string()))?;

  let old_name_lower = old_name.to_lowercase();
  let new_name_lower = new_name.to_lowercase();
  let new_name_title = {
    let mut chars = new_name_lower.chars();
    match chars.next() {
      None => return Err(CliError::Other("Invalid new name".to_string())),
      Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
  };

  // Check if old subcategory exists
  let subcategory_id = tracker_data
    .subcategory_id(&old_name_lower)
    .ok_or_else(|| {
      CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
        name: old_name.to_string(),
      })
    })?;

  // Check if new name already exists
  if tracker_data.subcategories_by_name.contains_key(&new_name_lower) {
    return Err(CliError::ValidationError(
      crate::ValidationErrorKind::SubcategoryAlreadyExists {
        name: new_name_title.clone(),
      },
    ));
  }

  // Update both maps
  tracker_data
    .subcategories_by_id
    .insert(subcategory_id, new_name_title.clone());
  tracker_data.subcategories_by_name.remove(&old_name_lower);
  tracker_data
    .subcategories_by_name
    .insert(new_name_lower, subcategory_id);
  tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse::new(crate::ResponseContent::Message(format!(
    "Subcategory renamed: '{}' → '{}'",
    old_name, new_name_title
  ))))
}
