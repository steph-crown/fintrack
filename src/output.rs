use std::io;

use crate::{CliError, CliResponse, ResponseContent};

/// Write a CLI error to the given writer
pub fn write_error(err: &CliError, writer: &mut impl std::io::Write) -> io::Result<()> {
  // TODO: Implement error formatting
  writeln!(writer, "Error: {:?}", err)?;

  Ok(())
}

/// Write a CLI response to the given writer
pub fn write_response(res: &CliResponse, writer: &mut impl std::io::Write) -> io::Result<()> {
  // TODO: Implement response formatting

  let Some(content) = res.content() else {
    writeln!(writer, "Success")?;
    return Ok(());
  };

  match content {
    ResponseContent::Message(msg) => {
      writeln!(writer, "{}", msg)?;
    }
    ResponseContent::Record(t) => {
      writeln!(
        writer,
        "Created Transaction: {} - ${}",
        t.description, t.amount
      )?;
    }
    ResponseContent::List(list) => {
      for item in list {
        writeln!(writer, "- {}", item.description)?;
      }
    }
    ResponseContent::TrackerData(tracker_data) => {
      writeln!(writer, "{:#?}", tracker_data)?;
    }
    ResponseContent::Total(totals) => {
      writeln!(
        writer,
        "Opening balance: {} {}\nTotal Income: {} {}\nTotal Expenses: {} {}\n--------------------------------\nTOTAL: {} {}",
        totals.opening_balance,
        totals.currency,
        totals.income_total,
        totals.currency,
        totals.expenses_total,
        totals.currency,
        totals.total(),
        totals.currency
      )?;
    }
  }

  Ok(())
}
