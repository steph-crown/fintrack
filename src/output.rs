use std::io;

use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

use crate::{CliError, Currency, Record, ResponseContent, TrackerData, ValidationErrorKind};

/// Write a CLI error to the given writer
pub fn write_error(err: &CliError, writer: &mut impl io::Write) -> io::Result<()> {
  match err {
    CliError::FileNotFound(path) => {
      writeln!(
        writer,
        "{} File not found: {}",
        "✗".red().bold(),
        path.bright_red()
      )?;
      writeln!(
        writer,
        "{} Run 'fintrack init' to initialize the tracker",
        "Suggestion:".yellow()
      )?;
    }
    CliError::InvalidJson(msg) => {
      writeln!(
        writer,
        "{} Invalid JSON: {}",
        "✗".red().bold(),
        msg.bright_red()
      )?;
      writeln!(
        writer,
        "{} Your tracker data may be corrupted. Try restoring from backup",
        "Suggestion:".yellow()
      )?;
    }
    CliError::ValidationError(kind) => {
      write_validation_error(kind, writer)?;
    }
    CliError::PermissionDenied(path) => {
      writeln!(
        writer,
        "{} Permission denied: {}",
        "✗".red().bold(),
        path.bright_red()
      )?;
      writeln!(
        writer,
        "{} Check file permissions or run with appropriate access",
        "Suggestion:".yellow()
      )?;
    }
    CliError::CorruptedData {
      backup_restored,
      timestamp,
    } => {
      if *backup_restored {
        writeln!(
          writer,
          "{} Data was corrupted but restored from backup ({})",
          "⚠".yellow().bold(),
          timestamp.bright_yellow()
        )?;
        writeln!(
          writer,
          "{} Please verify your recent changes",
          "Suggestion:".yellow()
        )?;
      } else {
        writeln!(
          writer,
          "{} Data corruption detected and backup restoration failed",
          "✗".red().bold()
        )?;
        writeln!(
          writer,
          "{} Run 'fintrack dump' to inspect remaining data, or 'fintrack clear' to reset",
          "Suggestion:".yellow()
        )?;
      }
    }
    CliError::FileAlreadyExists => {
      writeln!(writer, "{} Tracker already initialized", "✗".red().bold())?;
      writeln!(
        writer,
        "{} Use 'fintrack clear' to start over",
        "Suggestion:".yellow()
      )?;
    }
    CliError::Other(msg) => {
      writeln!(writer, "{} {}", "✗".red().bold(), msg.bright_red())?;
    }
  }

  Ok(())
}

fn write_validation_error(
  kind: &ValidationErrorKind,
  writer: &mut impl io::Write,
) -> io::Result<()> {
  match kind {
    ValidationErrorKind::AmountTooSmall { amount } => {
      writeln!(
        writer,
        "{} Amount must be greater than 0, got: {}",
        "✗ ValidationError:".red().bold(),
        amount.to_string().bright_red()
      )?;
      writeln!(
        writer,
        "{} Re-run the command with a positive amount (e.g., --amount 500)",
        "Suggestion:".yellow()
      )?;
    }
    ValidationErrorKind::InvalidDate {
      provided,
      expected_format,
    } => {
      writeln!(
        writer,
        "{} Invalid date format: '{}'",
        "✗ ValidationError:".red().bold(),
        provided.bright_red()
      )?;
      writeln!(
        writer,
        "{} Expected format: {}",
        "Suggestion:".yellow(),
        expected_format.bright_yellow()
      )?;
    }
    ValidationErrorKind::SubcategoryNotFound { name } => {
      writeln!(
        writer,
        "{} Subcategory '{}' not found",
        "✗ ValidationError:".red().bold(),
        name.bright_red()
      )?;
      writeln!(
        writer,
        "{} Use 'fintrack subcategory list' to see available subcategories",
        "Suggestion:".yellow()
      )?;
    }
    ValidationErrorKind::SubcategoryAlreadyExists { name } => {
      writeln!(
        writer,
        "{} Subcategory '{}' already exists",
        "✗ ValidationError:".red().bold(),
        name.bright_red()
      )?;
      writeln!(
        writer,
        "{} Use a different name or check existing subcategories",
        "Suggestion:".yellow()
      )?;
    }
    ValidationErrorKind::RecordNotFound { id } => {
      writeln!(
        writer,
        "{} Record with ID {} not found",
        "✗ ValidationError:".red().bold(),
        id.to_string().bright_red()
      )?;
      writeln!(
        writer,
        "{} Use 'fintrack list' to see available records",
        "Suggestion:".yellow()
      )?;
    }
    ValidationErrorKind::SubcategoryHasRecords { name, count } => {
      writeln!(
        writer,
        "{} Cannot delete '{}' — it has {} record(s)",
        "✗ ValidationError:".red().bold(),
        name.bright_red(),
        count.to_string().bright_red()
      )?;
      writeln!(
        writer,
        "{} Delete those records first using 'fintrack delete --by-subcat {}', or manually delete individual records",
        "Suggestion:".yellow(),
        name.bright_yellow()
      )?;
    }
    ValidationErrorKind::CannotDeleteMiscellaneous => {
      writeln!(
        writer,
        "{} Cannot delete 'Miscellaneous' — it is a system subcategory",
        "✗ ValidationError:".red().bold()
      )?;
    }
    ValidationErrorKind::CategoryImmutable { category } => {
      writeln!(
        writer,
        "{} Category {} is immutable and cannot be modified",
        "✗ ValidationError:".red().bold(),
        category.to_string().bright_red()
      )?;
    }
    ValidationErrorKind::InvalidCategoryName { name, reason } => {
      writeln!(
        writer,
        "{} Invalid category name '{}': {}",
        "✗ ValidationError:".red().bold(),
        name.bright_red(),
        reason.bright_red()
      )?;
    }
    ValidationErrorKind::InvalidName { name, reason } => {
      writeln!(
        writer,
        "{} Invalid name '{}': {}",
        "✗ ValidationError:".red().bold(),
        name.bright_red(),
        reason.bright_red()
      )?;
    }
    ValidationErrorKind::InvalidAmount { reason } => {
      writeln!(
        writer,
        "{} Invalid amount: {}",
        "✗ ValidationError:".red().bold(),
        reason.bright_red()
      )?;
    }
    ValidationErrorKind::TrackerAlreadyInitialized => {
      writeln!(
        writer,
        "{} Tracker already initialized",
        "✗ ValidationError:".red().bold()
      )?;
      writeln!(
        writer,
        "{} Use 'fintrack clear' to start over",
        "Suggestion:".yellow()
      )?;
    }
    ValidationErrorKind::InvalidSubcommand { subcommand } => {
      writeln!(
        writer,
        "{} Unknown subcommand: '{}'",
        "✗ ValidationError:".red().bold(),
        subcommand.bright_red()
      )?;
      writeln!(
        writer,
        "{} Use 'fintrack --help' to see available commands",
        "Suggestion:".yellow()
      )?;
    }
  }

  Ok(())
}

/// Write a CLI response to the given writer
pub fn write_response(res: &crate::CliResponse, writer: &mut impl io::Write) -> io::Result<()> {
  let Some(content) = res.content() else {
    writeln!(writer, "{}", "✓ Success".green().bold())?;
    return Ok(());
  };

  match content {
    ResponseContent::Message(msg) => {
      writeln!(writer, "{} {}", "✓".green().bold(), msg.bright_green())?;
    }
    ResponseContent::Record {
      record,
      tracker_data,
    } => {
      writeln!(writer, "{} Record created:", "✓".green().bold())?;
      // Parse currency string to Currency enum for display
      let currency_enum = tracker_data.currency.parse::<Currency>().ok();
      // Use TrackerData to resolve category/subcategory names from IDs
      write_record_single(&record, Some(tracker_data), currency_enum.as_ref(), writer)?;
    }
    ResponseContent::List {
      records,
      tracker_data,
    } => {
      if records.is_empty() {
        writeln!(writer, "{}", "No records found.".yellow())?;
      } else {
        // Parse currency string to Currency enum for display
        let currency_enum = tracker_data.currency.parse::<Currency>().ok();
        // Use TrackerData to resolve category/subcategory names from IDs
        write_records_table(&records, Some(tracker_data), currency_enum.as_ref(), writer)?;
      }
    }
    ResponseContent::TrackerData(tracker_data) => {
      write_tracker_data(tracker_data, writer)?;
    }
    ResponseContent::Total(totals) => {
      write_total_summary(totals, writer)?;
    }
  }

  Ok(())
}

/// Write records table with TrackerData context for resolving names
/// This is a helper function that commands can use when they have TrackerData available
pub fn write_records_table_with_context(
  records: &[Record],
  tracker_data: &TrackerData,
  writer: &mut impl io::Write,
) -> io::Result<()> {
  let currency = tracker_data.currency.parse::<Currency>().ok();
  write_records_table(records, Some(tracker_data), currency.as_ref(), writer)
}

/// Write a single record with TrackerData context for resolving names
/// This is a helper function that commands can use when they have TrackerData available
pub fn write_record_single_with_context(
  record: &Record,
  tracker_data: &TrackerData,
  writer: &mut impl io::Write,
) -> io::Result<()> {
  let currency = tracker_data.currency.parse::<Currency>().ok();
  write_record_single(record, Some(tracker_data), currency.as_ref(), writer)
}

/// Write a single record in a formatted line
fn write_record_single(
  record: &Record,
  tracker_data: Option<&TrackerData>,
  currency: Option<&Currency>,
  writer: &mut impl io::Write,
) -> io::Result<()> {
  let category_name = tracker_data
    .and_then(|td| td.category_name(record.category))
    .map(|s| s.as_str())
    .unwrap_or_else(|| "Unknown");

  let subcategory_name = tracker_data
    .and_then(|td| td.subcategory_name(record.subcategory))
    .map(|s| s.as_str())
    .unwrap_or_else(|| "Unknown");

  let currency_str = currency.map(|c| format!(" {}", c)).unwrap_or_default();

  writeln!(
    writer,
    "  ID: {} | {} | {} | {}{} | {} | {}",
    record.id.to_string().cyan(),
    category_name.bright_white(),
    subcategory_name.bright_white(),
    format_amount(record.amount).bright_white(),
    currency_str.bright_white(),
    record.date.bright_white(),
    if record.description.is_empty() {
      "(no description)".dimmed()
    } else {
      record.description.bright_white()
    }
  )?;
  Ok(())
}

/// Write records as a formatted table
fn write_records_table(
  records: &[Record],
  tracker_data: Option<&TrackerData>,
  currency: Option<&Currency>,
  writer: &mut impl io::Write,
) -> io::Result<()> {
  // Always show currency - use provided currency or fallback to empty string
  let currency_str = currency
    .map(|c| format!(" {}", c))
    .unwrap_or_else(|| "".to_string());

  let table_data: Vec<RecordRow> = records
    .iter()
    .map(|r| {
      let category_name = tracker_data
        .and_then(|td| td.category_name(r.category))
        .cloned()
        .unwrap_or_else(|| format!("Category {}", r.category));

      let subcategory_name = tracker_data
        .and_then(|td| td.subcategory_name(r.subcategory))
        .cloned()
        .unwrap_or_else(|| format!("Subcategory {}", r.subcategory));

      RecordRow {
        id: r.id.to_string(),
        category: category_name,
        subcategory: subcategory_name,
        amount: format!("{}{}", format_amount(r.amount), currency_str),
        date: r.date.clone(),
        description: if r.description.is_empty() {
          "(no description)".to_string()
        } else {
          r.description.clone()
        },
      }
    })
    .collect();

  let table = Table::new(&table_data).with(Style::modern()).to_string();

  writeln!(writer, "{}", table)?;
  Ok(())
}

/// Write tracker data (for dump command)
fn write_tracker_data(tracker_data: &TrackerData, writer: &mut impl io::Write) -> io::Result<()> {
  let json_string = serde_json::to_string_pretty(tracker_data)?;
  writeln!(writer, "{}", json_string)?;
  Ok(())
}

/// Write total summary with formatting
fn write_total_summary(totals: &crate::Total, writer: &mut impl io::Write) -> io::Result<()> {
  writeln!(writer, "{}", "Financial Summary:".bright_white().bold())?;
  writeln!(
    writer,
    "  {} {}",
    "Opening Balance:".bright_white(),
    format!(
      "{} {}",
      format_amount(totals.opening_balance),
      totals.currency
    )
    .bright_green()
  )?;
  writeln!(
    writer,
    "  {} {}",
    "Total Income:".bright_white(),
    format!("{} {}", format_amount(totals.income_total), totals.currency).bright_green()
  )?;
  writeln!(
    writer,
    "  {} {}",
    "Total Expenses:".bright_white(),
    format!(
      "{} {}",
      format_amount(totals.expenses_total),
      totals.currency
    )
    .bright_red()
  )?;
  writeln!(writer, "  {}", "──────────────────────────────".dimmed())?;
  writeln!(
    writer,
    "  {} {}",
    "Net Balance:".bright_white().bold(),
    format!("{} {}", format_amount(totals.total()), totals.currency)
      .bright_cyan()
      .bold()
  )?;
  Ok(())
}

/// Format amount with thousand separators and 2 decimal places
fn format_amount(amount: f64) -> String {
  let formatted = format!("{:.2}", amount);
  let parts: Vec<&str> = formatted.split('.').collect();
  let integer_part = parts[0];
  let decimal_part = parts.get(1).unwrap_or(&"00");

  // Add thousand separators
  let mut result = String::new();
  let chars: Vec<char> = integer_part.chars().rev().collect();
  for (i, ch) in chars.iter().enumerate() {
    if i > 0 && i % 3 == 0 {
      result.push(',');
    }
    result.push(*ch);
  }
  result = result.chars().rev().collect();
  format!("{}.{}", result, decimal_part)
}

/// Table row structure for records
#[derive(Tabled)]
struct RecordRow {
  #[tabled(rename = "ID")]
  id: String,
  #[tabled(rename = "Category")]
  category: String,
  #[tabled(rename = "Subcategory")]
  subcategory: String,
  #[tabled(rename = "Amount")]
  amount: String,
  #[tabled(rename = "Date")]
  date: String,
  #[tabled(rename = "Description")]
  description: String,
}
