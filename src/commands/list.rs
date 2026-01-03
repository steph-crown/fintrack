use chrono::NaiveDate;
use clap::{Arg, ArgGroup, ArgMatches, Command};

use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::FilePath;
use crate::utils::parsers::{parse_category, parse_date};
use crate::{CliResponse, CliResult, GlobalContext, Record, ResponseContent, TrackerData};

pub fn cli() -> Command {
  Command::new("list")
    .about("View and filter your transaction records")
    .long_about("Displays all your records in a table format. You can filter by date range, category, subcategory, or limit to first/last N records. Records are sorted by date (oldest first).")
    .arg(
      Arg::new("first")
        .short('f')
        .long("first")
        .value_parser(clap::value_parser!(usize))
        .help("Show only the first N records (oldest)")
        .long_help("Limits the output to the first N records when sorted by date. Shows the oldest records. Example: -f 5 shows the first 5 records."),
    )
    .arg(
      Arg::new("last")
        .short('l')
        .long("last")
        .value_parser(clap::value_parser!(usize))
        .help("Show only the last N records (newest)")
        .long_help("Limits the output to the last N records when sorted by date. Shows the most recent records. Example: -l 10 shows the last 10 records."),
    )
    .group(
      ArgGroup::new("first_or_last")
        .args(["first", "last"])
        .multiple(false),
    )
    .arg(
      Arg::new("start")
        .short('S')
        .long("start")
        .value_parser(parse_date)
        .help("Filter records from this date onwards (DD-MM-YYYY)")
        .long_help("Shows only records on or after this date. Format: DD-MM-YYYY (e.g., 01-12-2025). Use with --end to specify a date range."),
    )
    .arg(
      Arg::new("end")
        .short('E')
        .long("end")
        .value_parser(parse_date)
        .help("Filter records up to this date (DD-MM-YYYY)")
        .long_help("Shows only records on or before this date. Format: DD-MM-YYYY (e.g., 31-12-2025). Use with --start to specify a date range."),
    )
    .arg(
      Arg::new("category")
        .short('c')
        .long("category")
        .value_parser(parse_category)
        .help("Filter by category: 'income' or 'expenses'")
        .long_help("Shows only records in the specified category. Use 'income' to see all income transactions or 'expenses' to see all expense transactions. Case-insensitive."),
    )
    .arg(
      Arg::new("subcategory")
        .short('s')
        .long("subcategory")
        .value_parser(clap::value_parser!(String))
        .help("Filter by subcategory name")
        .long_help("Shows only records in the specified subcategory. The subcategory name is case-insensitive. Use 'fintrack subcategory list' to see available subcategories."),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let start_date = args.get_date_opt("start");
  let end_date = args.get_date_opt("end");

  let category_filter = args
    .get_category_opt("category")
    .map(|cat| tracker_data.category_id(&cat.to_string()));

  let subcategory_filter = args
    .get_subcategory_opt("subcategory")
    .and_then(|name| tracker_data.subcategory_id(&name));

  let mut filtered_data: Vec<Record> = tracker_data
    .records
    .iter()
    .filter(|r| {
      // Category filter: if filter is set, record must match
      category_filter.map_or(true, |expected_id| r.category == expected_id)
        // Subcategory filter: if filter is set, record must match
        && subcategory_filter.map_or(true, |expected_id| r.subcategory == expected_id)
        // Date range filter: parse date and check bounds
        && NaiveDate::parse_from_str(&r.date, "%d-%m-%Y")
          .map(|record_date| {
            start_date.map_or(true, |start| record_date >= start)
              && end_date.map_or(true, |end| record_date <= end)
          })
          .unwrap_or(false)
    })
    .cloned()
    .collect();

  filtered_data.sort_by(|a, b| {
    let date_a = NaiveDate::parse_from_str(&a.date, "%d-%m-%Y").unwrap_or(NaiveDate::MIN);
    let date_b = NaiveDate::parse_from_str(&b.date, "%d-%m-%Y").unwrap_or(NaiveDate::MIN);
    date_a.cmp(&date_b)
  });

  if args.contains_id("first") {
    let first = args.get_usize_or_default("first");
    if first > 0 {
      filtered_data.truncate(first);
    }
  } else if args.contains_id("last") {
    let last = args.get_usize_or_default("last");
    if last > 0 && filtered_data.len() > last {
      let start_idx = filtered_data.len() - last;
      filtered_data = filtered_data.into_iter().skip(start_idx).collect();
    }
  }

  Ok(CliResponse::new(ResponseContent::List {
    records: filtered_data,
    tracker_data,
  }))
}
