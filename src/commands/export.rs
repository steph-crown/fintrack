use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use chrono::Utc;
use clap::{Arg, ArgMatches, Command};

use crate::{
  CliError, CliResponse, CliResult, ExportFileType, GlobalContext, TrackerData,
  utils::file::FilePath,
};

pub fn cli() -> Command {
  Command::new("export")
    .about("Export tracker data to CSV or JSON file")
    .arg(
      Arg::new("path")
        .help("Folder path where exported file should be created")
        .index(1)
        .required(true)
        .value_parser(clap::value_parser!(PathBuf)),
    )
    .arg(
      Arg::new("type")
        .help("The file type the export should be in. Defaults to json.")
        .short('t')
        .long("type")
        .value_parser(clap::value_parser!(ExportFileType))
        .default_value("json"),
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
  let file = gctx.tracker_path().open_read()?;
  let tracker_data: TrackerData = serde_json::from_reader(&file)?;

  let export_path = args
    .get_one::<PathBuf>("path")
    .ok_or_else(|| CliError::Other("Export path not provided".to_string()))?;

  let file_type = args
    .get_one::<ExportFileType>("type")
    .ok_or_else(|| CliError::Other("File type not provided".to_string()))?;

  // Validate path exists and is a directory
  if !export_path.exists() {
    return Err(CliError::Other(format!(
      "Path does not exist: {}",
      export_path.display()
    )));
  }
  if !export_path.is_dir() {
    return Err(CliError::Other(format!(
      "Path is not a directory: {}",
      export_path.display()
    )));
  }

  // Generate filename with timestamp
  let timestamp_str = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
  let extension = match file_type {
    ExportFileType::CSV => "csv",
    ExportFileType::JSON => "json",
    ExportFileType::PDF => "pdf",
  };
  let filename = format!("fintrack_export_{}.{}", timestamp_str, extension);
  let file_path = export_path.join(&filename);

  // Export based on file type
  match file_type {
    ExportFileType::CSV => export_to_csv(&tracker_data, &file_path)?,
    ExportFileType::JSON => export_to_json(&tracker_data, &file_path)?,
    ExportFileType::PDF => {
      return Err(CliError::Other("PDF export not yet implemented".to_string()))
    }
  }

  Ok(CliResponse::new(crate::ResponseContent::Message(format!(
    "Data exported to: {}",
    file_path.display()
  ))))
}

fn export_to_csv(tracker_data: &TrackerData, file_path: &PathBuf) -> Result<(), CliError> {
  let mut file = File::create(file_path)?;

  // Write CSV header
  writeln!(file, "ID,Category,Subcategory,Amount,Currency,Date,Description")?;

  // Write records
  for record in &tracker_data.records {
    let category_name = tracker_data
      .category_name(record.category)
      .map(|s| s.as_str())
      .unwrap_or_else(|| "Unknown");
    let subcategory_name = tracker_data
      .subcategory_name(record.subcategory)
      .map(|s| s.as_str())
      .unwrap_or_else(|| "Unknown");

    // Escape commas and quotes in description
    let description = record
      .description
      .replace('"', "\"\"")
      .replace('\n', " ")
      .replace('\r', " ");

    writeln!(
      file,
      "{},{},{},{},{},{},\"{}\"",
      record.id,
      category_name,
      subcategory_name,
      record.amount,
      tracker_data.currency,
      record.date,
      description
    )?;
  }

  Ok(())
}

fn export_to_json(tracker_data: &TrackerData, file_path: &PathBuf) -> Result<(), CliError> {
  let json_string = serde_json::to_string_pretty(tracker_data)?;
  let mut file = File::create(file_path)?;
  file.write_all(json_string.as_bytes())?;
  Ok(())
}
