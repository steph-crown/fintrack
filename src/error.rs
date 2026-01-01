use crate::output;

#[derive(Debug)]
pub enum ValidationErrorKind {
  AmountTooSmall {
    amount: f64,
  },
  InvalidDate {
    provided: String,
    expected_format: String,
  },
  SubcategoryNotFound {
    name: String,
  },
  SubcategoryAlreadyExists {
    name: String,
  },
  RecordNotFound {
    id: usize,
  },
  SubcategoryHasRecords {
    name: String,
    count: usize,
  },
  CannotDeleteMiscellaneous,
  CategoryImmutable {
    category: usize,
  },
  InvalidCategoryName {
    name: String,
    reason: String,
  },
  InvalidName {
    name: String,
    reason: String,
  },
  InvalidAmount {
    reason: String,
  },
  TrackerAlreadyInitialized,
  InvalidSubcommand {
    subcommand: String,
  },
}

#[derive(Debug)]
pub enum CliError {
  FileNotFound(String),
  InvalidJson(String),
  ValidationError(ValidationErrorKind),
  PermissionDenied(String),
  CorruptedData {
    backup_restored: bool,
    timestamp: String,
  },
  FileAlreadyExists,
  Other(String),
}

// #[derive(Debug)]
// pub struct CliError {
//   error: ProcessError,
// }

impl CliError {
  /// Write this error to the given writer
  pub fn write_to(&self, writer: &mut impl std::io::Write) {
    output::write_error(self, writer);
  }
}

impl From<std::io::Error> for CliError {
  fn from(err: std::io::Error) -> Self {
    match err.kind() {
      std::io::ErrorKind::NotFound => CliError::FileNotFound(err.to_string()),
      std::io::ErrorKind::PermissionDenied => CliError::PermissionDenied(err.to_string()),
      std::io::ErrorKind::AlreadyExists => CliError::FileAlreadyExists,
      _ => CliError::Other(format!("IO error: {}", err)),
    }
  }
}

impl From<serde_json::Error> for CliError {
  fn from(err: serde_json::Error) -> Self {
    CliError::InvalidJson(err.to_string())
  }
}

pub fn invalid_subcommand_error(cmd: &str) -> CliError {
  CliError::ValidationError(ValidationErrorKind::InvalidSubcommand {
    subcommand: cmd.to_string(),
  })
}
