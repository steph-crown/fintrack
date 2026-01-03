use std::collections::HashSet;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};

use crate::{
  Category, CliResponse, CliResult, GlobalContext, TrackerData,
  command_prelude::ArgMatchesExt,
  utils::file::{FilePath, write_json_to_file},
};

pub fn cli() -> Command {
  Command::new("delete")
    .about("Modify an existing record")
    .arg(
      Arg::new("ids")
        .help("Comma separated list of record ids")
        .short('i')
        .long("ids")
        .value_parser(clap::value_parser!(usize))
        .action(ArgAction::Append)
        .value_delimiter(','),
    )
    .arg(
      Arg::new("by-cat")
        .help("Specify a category. Deletes record for the category")
        .short('c')
        .long("by-cat")
        .value_parser(clap::value_parser!(Category)),
    )
    .arg(
      Arg::new("by-subcat")
        .help("Specify a subcategory. Deletes record for the subcategory")
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
