use crate::{CliError, CliResponse, ResponseContent};

/// Write a CLI error to the given writer
pub fn write_error(err: &CliError, writer: &mut impl std::io::Write) {
  // TODO: Implement error formatting
  let _ = writeln!(writer, "Error: {:?}", err);
}

/// Write a CLI response to the given writer
pub fn write_response(res: &CliResponse, writer: &mut impl std::io::Write) {
  // TODO: Implement response formatting
  let _ = writeln!(writer, "Success");

  let Some(content) = &res.content() else {
    writeln!(writer, "Success");
    return;
  };

  match content {
    ResponseContent::Message(msg) => {
      writeln!(writer, "{}", msg).unwrap();
    }
    ResponseContent::Record(t) => {
      writeln!(
        writer,
        "Created Transaction: {} - ${}",
        t.description, t.amount
      )
      .unwrap();
    }
    ResponseContent::List(list) => {
      for item in list {
        writeln!(writer, "- {}", item.description).unwrap();
      }
    }
    ResponseContent::TrackerData(tracker_data) => {
      writeln!(writer, "{:#?}", tracker_data);
    }
    _ => {
      writeln!(writer, "Success");
    }
  }
}
