use std::collections::HashSet;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};

use crate::{
  CliResponse, CliResult, GlobalContext, TrackerData,
  command_prelude::ArgMatchesExt,
  utils::file::{FilePath, write_json_to_file},
  utils::parsers::parse_category,
};

pub fn cli() -> Command {
  Command::new("delete")
    .about("Delete transaction records")
    .long_about("Removes one or more records from your tracker. You can delete by record ID(s), by category (all income or all expenses), or by subcategory (all records in a specific subcategory).")
    .arg(
      Arg::new("ids")
        .help("Delete specific records by their IDs")
        .long_help("Delete one or more specific records by their ID numbers. Use comma-separated list for multiple IDs. Example: -i 1,5,10")
        .short('i')
        .long("ids")
        .value_parser(clap::value_parser!(usize))
        .action(ArgAction::Append)
        .value_delimiter(','),
    )
    .arg(
      Arg::new("by-cat")
        .help("Delete all records in a category")
        .long_help("Deletes all records in the specified category (either 'income' or 'expenses'). Use with caution as this will remove all transactions of that type. Case-insensitive.")
        .short('c')
        .long("by-cat")
        .value_parser(parse_category),
    )
    .arg(
      Arg::new("by-subcat")
        .help("Delete all records in a subcategory")
        .long_help("Deletes all records that belong to the specified subcategory. The subcategory name is case-insensitive. Use 'fintrack subcategory list' to see available subcategories.")
        .short('s')
        .long("by-subcat")
        .value_parser(clap::value_parser!(String)),
    )
    .group(
      ArgGroup::new("delete_by")
        .args(["ids", "by-cat", "by-subcat"])
        .multiple(false)
        .required(true),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let mut file = gctx.tracker_path().open_read_write()?;
  let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

  if args.contains_id("ids") {
    let ids: Vec<usize> = args.get_vec::<usize>("ids");
    let ids_set: HashSet<usize> = ids.into_iter().collect();

    tracker_data.records.retain(|r| !ids_set.contains(&r.id));
  } else if args.contains_id("by-cat") {
    let category = args.get_category("by-cat")?;
    let category_str = category.to_string();
    let category_id = tracker_data.category_id(&category_str);

    tracker_data.records.retain(|r| r.category != category_id);
  } else if args.contains_id("by-subcat") {
    let subcategory_name = args
      .get_subcategory_opt("by-subcat")
      .ok_or_else(|| crate::CliError::Other("Subcategory not provided".to_string()))?;

    let subcategory_id = tracker_data
      .subcategory_id(subcategory_name.as_str())
      .ok_or_else(|| {
        crate::CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
          name: subcategory_name.clone(),
        })
      })?;

    tracker_data
      .records
      .retain(|r| r.subcategory != subcategory_id);
  }

  tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

  let tracker_json = serde_json::json!(tracker_data);
  write_json_to_file(&tracker_json, &mut file)?;

  Ok(CliResponse::success())
}
