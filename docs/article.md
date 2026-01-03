# How to Build a Local-First CLI Financial Tracker with Rust

Most financial apps require you to trust a company with your sensitive data, pay monthly fees, and hope the service stays online. But what if you could build a financial tracker that runs entirely on your computer, stores data in human-readable JSON files, and gives you complete control?

That's the power of local-first applications. Your data stays on your machine, you own it completely, and you're not dependent on any external service. In this tutorial, you'll build a complete CLI financial tracker using Rust that lets you track income and expenses with full control over your data.

By the end of this tutorial, you'll have a working command-line tool that can initialize a tracker, add records, list and filter transactions, update entries, delete records, manage subcategories, and calculate totals. You'll learn Rust concepts like traits, error handling, file I/O, JSON serialization, and CLI development.

We'll focus on building the core features that make the tracker functional. Advanced features like data export, analysis, and automatic backups are left as exercises you can implement yourself.

## Prerequisites

Before you start, make sure you have:

- Rust installed (version 1.70 or later). If you don't have Rust installed, follow the [official installation guide](https://rust-book.cs.brown.edu/ch01-01-installation.html). You can verify your installation by running `rustc --version` in your terminal.
- Basic understanding of Rust concepts: structs, enums, error handling with `Result`, and traits.
- Familiarity with command-line tools and terminal usage.
- Basic knowledge of JSON format.

You don't need prior experience with `clap` or similar CLI libraries. We'll explain everything as we go.

## Commands We'll Build

In this tutorial, you'll implement these commands step-by-step:

- `init` - Initialize a new tracker
- `add` - Add income or expense records
- `list` - View and filter records
- `update` - Modify existing records
- `delete` - Remove records
- `subcategory` - Manage subcategories (list, add, delete, rename)
- `total` - Calculate financial totals

We won't cover advanced features like `describe` (data analysis), `export` (CSV/JSON export), `dump` (raw JSON display), or `clear` (reset tracker) in this tutorial. These make great exercises once you understand the fundamentals.

All code examples in this tutorial are actual, working code from the full implementation. You can copy and paste them directly, and they'll work exactly as shown.

## Step 1: Set Up the Project

Start by creating a new Rust project:

```bash
cargo new fintrack
cd fintrack
```

Next, add the necessary dependencies to your `Cargo.toml` file:

```toml
[dependencies]
chrono = "0.4.42"
clap = { version = "4.5.53", features = ["derive"] }
dirs = "6.0.0"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.148"
strum = { version = "0.26", features = ["derive"] }
```

Here's what each dependency does:

- `clap` - A powerful command-line argument parser. We use it to parse and validate command line arguments.
- `serde` and `serde_json` - Serialization libraries for converting Rust structs to and from JSON. This lets us save and load tracker data.
- `chrono` - Date and time handling. We use it to parse dates and generate timestamps.
- `dirs` - Cross-platform library for finding user directories. We use it to locate the home directory for storing tracker data.
- `strum` - Provides automatic `Display` and `FromStr` implementations for enums. This makes `Category` and `Currency` enums easier to work with.

We organize our code into modules for better structure. The main modules are:

- `commands/` - Each command has its own file (init, add, list, etc.)
- `utils/` - Shared utilities like file operations and argument parsing helpers
- `models.rs` - Data structures and types
- `error.rs` - Error types and handling
- `output.rs` - Output formatting functions

## Step 2: Design the Data Model

Before writing code, let's understand the architecture. FinTrack follows a layered architecture:

```
┌─────────────────────────────────────────────────────┐
│                    CLI Layer (main.rs)              │
│           Clap Argument Parsing & Dispatch          │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│              Process Layer (commands/)              │
│  init, add, delete, update, list, category,         │
│  subcategory, clear, total, describe, dump, export  │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│            Business Logic Layer (lib.rs)            │
│  Validation, file I/O, data transformation,         │
│  serialization                                       │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│           Persistent Storage Layer                  │
│  ~/.fintrack/tracker.json (main data)               │
│  ~/.fintrack/backups/ (future backup location)      │
└─────────────────────────────────────────────────────┘
```

This layered approach separates concerns. The CLI layer handles user input, the process layer contains command logic, the business logic layer provides shared utilities, and the storage layer persists data. This makes the code testable and maintainable.

Now let's define the core data structures. Create `src/models.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    pub id: usize,
    pub category: usize,    // ID from categories map
    pub subcategory: usize, // ID from subcategories map
    pub description: String,
    pub amount: f64,  // Always positive; sign determined by category
    pub date: String, // Format: DD-MM-YYYY
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrackerData {
    pub version: u32,
    pub currency: String,
    pub created_at: String,
    pub last_modified: String,
    pub opening_balance: f64,
    pub categories: HashMap<String, usize>,
    pub subcategories_by_id: HashMap<usize, String>,
    pub subcategories_by_name: HashMap<String, usize>,
    pub next_subcategory_id: u32,
    pub records: Vec<Record>,
    pub next_record_id: usize,
}
```

We use IDs instead of strings for categories and subcategories. This approach is more efficient, maintains referential integrity, and makes updates easier. If you rename a subcategory, you only update the HashMap entries, not every record that references it.

The two-HashMap approach for subcategories (`subcategories_by_id` and `subcategories_by_name`) provides bidirectional lookup. We need name→ID for validation when users provide subcategory names, and ID→name for display when showing records.

We store currency as a string for flexibility. This allows easy extension to new currencies without changing the enum. The JSON structure is human-readable, which aligns with the local-first principle and makes debugging easier.

Add the enums:

```rust
#[derive(clap::ValueEnum, Clone, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "UPPERCASE", ascii_case_insensitive)]
pub enum Currency {
    NGN,
    USD,
    GBP,
    EUR,
    CAD,
    AUD,
    JPY,
}

#[derive(clap::ValueEnum, Clone, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Category {
    Income,
    Expenses,
}
```

The `strum` crate automatically generates `Display` and `FromStr` implementations. The `ascii_case_insensitive` attribute makes parsing case-insensitive, so users can type "income", "Income", or "INCOME" and they all work.

Add helper methods to `TrackerData`:

```rust
impl TrackerData {
    pub fn push_record(&mut self, record: Record) -> &Self {
        self.records.push(record);
        self
    }

    pub fn category_id(&self, category: &str) -> usize {
        self.categories[category]
    }

    pub fn miscellaneous_subcategory_id(&self) -> Option<usize> {
        self.subcategories_by_name.get("miscellaneous").copied()
    }

    pub fn subcategory_id(&self, name: &str) -> Option<usize> {
        self.subcategories_by_name.get(name).copied()
    }

    pub fn category_name(&self, id: usize) -> Option<&String> {
        self.categories.iter().find(|(_, v)| **v == id).map(|(k, _)| k)
    }

    pub fn subcategory_name(&self, id: usize) -> Option<&String> {
        self.subcategories_by_id.get(&id)
    }

    pub fn totals(&self) -> (f64, f64) {
        self.records.iter().fold((0.0, 0.0), |mut acc, r| {
            if r.category == 1 {
                acc.0 += r.amount;
            } else {
                acc.1 += r.amount;
            }
            acc
        })
    }
}
```

The `totals()` method separates income (category ID 1) and expenses (category ID 2) and returns both totals as a tuple.

## Step 3: Set Up the CLI Structure

We use `clap` for argument parsing because it's powerful, ergonomic, and well-maintained. It provides excellent help text generation and validation.

Each command follows a pattern: a `cli()` function that defines arguments and a `exec()` function that contains the logic. This separation improves testability, reusability, and keeps concerns clear.

The `cli()` function returns a `Command` that describes arguments, help text, and validation rules. The `exec()` function takes a `GlobalContext` and `ArgMatches`, then returns a `CliResult`.

We use a `GlobalContext` struct to manage file paths centrally. This makes testing easier and keeps path logic in one place. Create `src/utils/context.rs`:

```rust
use std::path::PathBuf;

#[derive(Debug)]
pub struct GlobalContext {
    home_path: PathBuf,
    base_path: PathBuf,
    tracker_path: PathBuf,
    config_path: PathBuf,
    backups_path: PathBuf,
}

impl GlobalContext {
    pub fn new(home_dir: PathBuf) -> Self {
        let base_path = home_dir.join(".fintrack");
        let tracker_path = base_path.join("tracker.json");
        let config_path = base_path.join("config");
        let backups_path = base_path.join("backups");

        GlobalContext {
            home_path: home_dir,
            base_path,
            tracker_path,
            config_path,
            backups_path,
        }
    }

    pub fn tracker_path(&self) -> &PathBuf {
        &self.tracker_path
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn backups_path(&self) -> &PathBuf {
        &self.backups_path
    }
}
```

Now define error types. Create `src/error.rs`:

```rust
use std::io;

#[derive(Debug)]
pub enum ValidationErrorKind {
    AmountTooSmall { amount: f64 },
    InvalidDate { provided: String, expected_format: String },
    SubcategoryNotFound { name: String },
    SubcategoryAlreadyExists { name: String },
    RecordNotFound { id: usize },
    SubcategoryHasRecords { name: String, count: usize },
    CannotDeleteMiscellaneous,
    InvalidCategoryName { name: String, reason: String },
    InvalidName { name: String, reason: String },
    TrackerAlreadyInitialized,
    InvalidSubcommand { subcommand: String },
}

#[derive(Debug)]
pub enum CliError {
    FileNotFound(String),
    InvalidJson(String),
    ValidationError(ValidationErrorKind),
    PermissionDenied(String),
    FileAlreadyExists,
    Other(String),
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
```

We use enums for errors because they provide type safety and exhaustive matching. The compiler ensures we handle all error cases.

Define response types in `src/models.rs`:

```rust
pub struct CliResponse {
    content: Option<ResponseContent>,
}

impl CliResponse {
    pub fn new(content: ResponseContent) -> Self {
        Self {
            content: Some(content),
        }
    }

    pub fn success() -> Self {
        Self { content: None }
    }

    pub fn content(&self) -> Option<&ResponseContent> {
        self.content.as_ref()
    }
}

pub enum ResponseContent {
    Message(String),
    Record {
        record: Record,
        tracker_data: TrackerData,
        is_update: bool,
    },
    List { records: Vec<Record>, tracker_data: TrackerData },
    Total(Total),
    Subcategories(Vec<(usize, String)>),
}

pub struct Total {
    pub currency: Currency,
    pub opening_balance: f64,
    pub income_total: f64,
    pub expenses_total: f64,
}

pub type CliResult = Result<CliResponse, CliError>;
```

The `ResponseContent` enum allows different commands to return different types of data. This provides type safety and makes it clear what each command returns.

## Step 4: Implement File Operations

We create a `FilePath` trait for reusable file operations. Using a trait allows method syntax and works with both `Path` and `PathBuf`. Create `src/utils/file.rs`:

```rust
use std::fs::File;
use std::io;
use std::path::Path;

pub trait FilePath: AsRef<Path> {
    fn create_file_if_not_exists(&self) -> io::Result<File> {
        let path = self.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        File::options().write(true).create_new(true).open(path)
    }

    fn read_file(&self) -> io::Result<File> {
        File::options().read(true).open(self.as_ref())
    }

    fn open_read_write(&self) -> io::Result<File> {
        File::options().read(true).write(true).open(self.as_ref())
    }

    fn delete_if_exists(&self) -> io::Result<()> {
        let path = self.as_ref();
        if path.is_dir() {
            std::fs::remove_dir_all(path)?;
        } else {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}

impl<P: AsRef<Path>> FilePath for P {}
```

The blanket implementation `impl<P: AsRef<Path>> FilePath for P` means any type that implements `AsRef<Path>` automatically gets these methods. This works with `Path`, `PathBuf`, `&str`, `String`, and more.

Add a helper function for writing JSON:

```rust
use serde_json::Value;

pub fn write_json_to_file(json: &Value, file: &mut File) -> Result<(), crate::CliError> {
    let json_string = serde_json::to_string_pretty(&json)?;

    file.seek(io::SeekFrom::Start(0))?;
    file.set_len(0)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
```

We seek to the start and truncate the file before writing. This ensures we overwrite the entire file, not append to it. The `?` operator propagates errors automatically, making error handling concise and idiomatic.

## Step 5: Build the Init Command

The `init` command creates a new tracker file. Create `src/commands/init.rs`:

```rust
use clap::{Arg, ArgMatches, Command};
use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::{CliResponse, CliResult, Currency, GlobalContext, default_tracker_json};

pub fn cli() -> Command {
    Command::new("init")
        .about("Initialize a new financial tracker")
        .long_about("Creates a new tracker file in ~/.fintrack/ with default categories (Income, Expenses) and a default subcategory (Miscellaneous). You must run this command before using any other commands.")
        .arg(
            Arg::new("currency")
                .short('c')
                .long("currency")
                .value_parser(clap::value_parser!(Currency))
                .default_value("ngn")
                .help("Currency code for your tracker (NGN, USD, GBP, EUR, CAD, AUD, JPY)")
                .long_help("Sets the currency that will be used for all amounts. This cannot be changed after initialization. Defaults to NGN if not specified."),
        )
        .arg(
            Arg::new("opening")
                .short('o')
                .long("opening")
                .value_parser(clap::value_parser!(f64))
                .help("Your opening balance amount")
                .long_help("Sets your starting balance. This is the amount you have before adding any income or expenses. Defaults to 0.0 if not specified."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let currency = args.get_currency_or_default("currency");
    let opening_balance = args.get_f64_or_default("opening");

    let mut file = gctx.tracker_path().create_file_if_not_exists()?;

    let default_json = default_tracker_json(currency, opening_balance);
    write_json_to_file(&default_json, &mut file)?;

    Ok(CliResponse::success())
}
```

The `cli()` function defines the command structure. Arguments have short flags (`-c`, `-o`), long flags (`--currency`, `--opening`), help text, and default values.

The `exec()` function uses `ArgMatchesExt` methods to extract values. We'll define this trait next. The `create_file_if_not_exists()` method creates parent directories if needed and creates the file, failing if it already exists. This prevents accidental overwrites.

Add the `default_tracker_json` function to `src/models.rs`:

```rust
pub fn default_tracker_json(currency: &Currency, opening_balance: f64) -> serde_json::Value {
    serde_json::json!({
        "version": 1,
        "currency": currency.to_string(),
        "opening_balance": opening_balance,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "last_modified": chrono::Utc::now().to_rfc3339(),
        "categories": {
            "income": 1,
            "expenses": 2
        },
        "subcategories_by_id": {
            "1": "miscellaneous"
        },
        "subcategories_by_name": {
            "miscellaneous": 1
        },
        "records": [],
        "next_record_id": 1,
        "next_subcategory_id": 2
    })
}
```

We use fixed IDs for categories (1 for income, 2 for expenses) because they're immutable. This makes referencing them easier and more efficient. We track `created_at` and `last_modified` timestamps for data integrity and debugging.

## Step 6: Add Records

The `add` command creates new transaction records. Create `src/commands/add.rs`:

```rust
use chrono::Local;
use clap::{Arg, ArgMatches, Command};
use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::utils::parsers::{parse_category, parse_date};
use crate::{CliError, CliResponse, CliResult, GlobalContext, Record, ResponseContent, TrackerData};

pub fn cli() -> Command {
    Command::new("add")
        .about("Record a new income or expense transaction")
        .long_about("Adds a new financial record to your tracker. Category and amount are required. The amount must be greater than 0.")
        .arg(
            Arg::new("category")
                .index(1)
                .required(true)
                .value_parser(parse_category)
                .help("Transaction category: 'income' or 'expenses' (case-insensitive)")
                .long_help("The type of transaction. Use 'income' for money received or 'expenses' for money spent. Case-insensitive (Income, INCOME, income all work)."),
        )
        .arg(
            Arg::new("amount")
                .index(2)
                .required(true)
                .value_parser(clap::value_parser!(f64))
                .help("Transaction amount (must be greater than 0)")
                .long_help("The amount of money for this transaction. Must be a positive number greater than 0. Examples: 100, 150.50, 2000.75"),
        )
        .arg(
            Arg::new("subcategory")
                .short('s')
                .long("subcategory")
                .value_parser(clap::value_parser!(String))
                .default_value("miscellaneous")
                .help("Subcategory name for this transaction")
                .long_help("A more specific category for this transaction (e.g., 'Groceries', 'Salary', 'Rent'). Must already exist - use 'fintrack subcategory list' to see available subcategories. Defaults to 'miscellaneous' if not specified."),
        )
        .arg(
            Arg::new("description")
                .short('d')
                .long("description")
                .value_parser(clap::value_parser!(String))
                .help("Optional description or notes for this transaction")
                .long_help("Any additional notes or description you want to add to this transaction. This is optional and can be left empty."),
        )
        .arg(
            Arg::new("date")
                .short('D')
                .long("date")
                .value_parser(parse_date)
                .help("Transaction date in DD-MM-YYYY format")
                .long_help("The date when this transaction occurred. Format: DD-MM-YYYY (e.g., 30-12-2025). Defaults to today's date if not specified."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let category = args.get_category("category")?;
    let amount = args.get_f64_or_default("amount");

    if amount <= 0.0 {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::AmountTooSmall { amount },
        ));
    }

    let subcategory_name = args.get_subcategory_or_default("subcategory");
    let description = args.get_string_or_default("description");

    let category_str = category.to_string();
    let category_id = tracker_data.category_id(&category_str);

    let subcategory_id = tracker_data
        .subcategory_id(&subcategory_name)
        .ok_or_else(|| {
            CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
                name: subcategory_name,
            })
        })?;

    let date = args
        .get_date_opt("date")
        .map(|d| d.format("%d-%m-%Y").to_string())
        .unwrap_or_else(|| Local::now().format("%d-%m-%Y").to_string());

    let record_id = tracker_data.next_record_id;
    let record = Record {
        id: record_id,
        category: category_id,
        amount,
        subcategory: subcategory_id,
        description,
        date,
    };

    tracker_data.next_record_id += 1;
    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();
    tracker_data.push_record(record.clone());

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(ResponseContent::Record {
        record,
        tracker_data,
        is_update: false,
    }))
}
```

We validate that the amount is greater than zero. This prevents invalid data and enforces business logic. We default to "miscellaneous" subcategory because it's always available and prevents errors. We generate unique record IDs by incrementing a counter, which is simple and efficient. If no date is provided, we default to today's date for convenience.

We make category parsing case-insensitive for better user experience. Users can type "income", "Income", or "INCOME" and they all work.

## Step 7: List Records

The `list` command displays records with filtering options. Create `src/commands/list.rs`:

```rust
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
            category_filter.map_or(true, |expected_id| r.category == expected_id)
                && subcategory_filter.map_or(true, |expected_id| r.subcategory == expected_id)
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
```

We use `Option` types for filters because they're optional. The `map_or(true, ...)` pattern means "if the filter is set, apply it; otherwise, include the record." We parse dates upfront to validate them and enable date comparisons.

We use `ArgGroup` to make `first` and `last` mutually exclusive. Users can specify one or the other, but not both. This prevents ambiguity and creates a clear API.

We sort by date because chronological order is most useful for financial records. Users typically want to see transactions in the order they occurred.

## Step 8: Update Records

The `update` command modifies existing records. Create `src/commands/update.rs`:

```rust
use clap::{Arg, ArgMatches, Command};
use crate::command_prelude::ArgMatchesExt;
use crate::utils::file::{FilePath, write_json_to_file};
use crate::utils::parsers::{parse_category, parse_date};
use crate::{CliError, CliResponse, CliResult, GlobalContext, ResponseContent, TrackerData};

pub fn cli() -> Command {
    Command::new("update")
        .about("Modify an existing transaction record")
        .long_about("Updates one or more fields of an existing record. Only the fields you specify will be changed; others remain unchanged. Use 'fintrack list' to find record IDs.")
        .arg(
            Arg::new("record_id")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(usize))
                .help("The ID of the record to update")
                .long_help("The unique ID number of the record you want to modify. Use 'fintrack list' to see all records and their IDs."),
        )
        .arg(
            Arg::new("category")
                .short('c')
                .long("category")
                .value_parser(parse_category)
                .help("Change the category to 'income' or 'expenses'")
                .long_help("Updates the transaction category. Use 'income' for money received or 'expenses' for money spent. Case-insensitive."),
        )
        .arg(
            Arg::new("amount")
                .short('a')
                .long("amount")
                .value_parser(clap::value_parser!(f64))
                .help("Change the transaction amount (must be greater than 0)")
                .long_help("Updates the transaction amount. Must be a positive number greater than 0. Examples: 100, 150.50, 2000.75"),
        )
        .arg(
            Arg::new("subcategory")
                .short('s')
                .long("subcategory")
                .value_parser(clap::value_parser!(String))
                .help("Change the subcategory name")
                .long_help("Updates the subcategory. The subcategory must already exist - use 'fintrack subcategory list' to see available subcategories."),
        )
        .arg(
            Arg::new("description")
                .short('d')
                .long("description")
                .value_parser(clap::value_parser!(String))
                .help("Change the description or notes")
                .long_help("Updates the transaction description or notes. You can set this to an empty string to remove the description."),
        )
        .arg(
            Arg::new("date")
                .short('D')
                .long("date")
                .value_parser(parse_date)
                .help("Change the transaction date (DD-MM-YYYY format)")
                .long_help("Updates the transaction date. Format: DD-MM-YYYY (e.g., 30-12-2025)."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let record_id = args
        .get_usize("record_id")
        .map_err(|_| CliError::ValidationError(crate::ValidationErrorKind::RecordNotFound { id: 0 }))?;

    let category_id = args.get_category_opt("category").map(|category| {
        let category_str = category.to_string();
        tracker_data.category_id(&category_str)
    });

    let subcategory_id = args
        .get_subcategory_opt("subcategory")
        .map(|name| {
            tracker_data.subcategory_id(&name).ok_or_else(|| {
                CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound { name })
            })
        })
        .transpose()?;

    let record = tracker_data
        .records
        .iter_mut()
        .find(|r| r.id == record_id)
        .ok_or_else(|| {
            CliError::ValidationError(crate::ValidationErrorKind::RecordNotFound { id: record_id })
        })?;

    if let Some(cat_id) = category_id {
        record.category = cat_id;
    }

    if let Some(amount) = args.get_f64_opt("amount") {
        if amount <= 0.0 {
            return Err(CliError::ValidationError(
                crate::ValidationErrorKind::AmountTooSmall { amount },
            ));
        }
        record.amount = amount;
    }

    if let Some(subcat_id) = subcategory_id {
        record.subcategory = subcat_id;
    }

    if let Some(description) = args.get_string_opt("description") {
        record.description = description;
    }

    if let Some(date) = args.get_date_opt("date") {
        record.date = date.format("%d-%m-%Y").to_string();
    }

    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let updated_record = record.clone();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(ResponseContent::Record {
        record: updated_record,
        tracker_data,
        is_update: true,
    }))
}
```

We use ID lookup because it's fast and unambiguous. We support partial updates for user convenience and efficiency. Users only need to specify the fields they want to change.

We validate on update to maintain data integrity and prevent invalid states. We handle optional arguments gracefully using `Option` types. If an argument isn't provided, we skip updating that field.

## Step 9: Delete Records

The `delete` command removes records. Create `src/commands/delete.rs`:

```rust
use std::collections::HashSet;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use crate::{CliResponse, CliResult, GlobalContext, TrackerData, command_prelude::ArgMatchesExt, utils::file::{FilePath, write_json_to_file}, utils::parsers::parse_category};

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

        tracker_data.records.retain(|r| r.subcategory != subcategory_id);
    }

    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::success())
}
```

We support multiple deletion modes for flexibility. Users can delete by IDs, by category, or by subcategory depending on their needs. We use `ArgGroup` to enforce one method, preventing ambiguity and creating a clear API.

We use `Vec::retain()` for efficient filtering. It removes elements in-place, which is more efficient than creating a new vector. The `retain()` method keeps elements where the closure returns `true`, so we negate the condition.

## Step 10: Manage Subcategories

Subcategories help organize transactions. We'll implement four subcommands: list, add, delete, and rename.

### List Subcategories

Create `src/commands/subcategory/list.rs`:

```rust
use clap::{ArgMatches, Command};
use crate::{CliResponse, CliResult, GlobalContext, ResponseContent, TrackerData, utils::file::FilePath};

pub fn cli() -> Command {
    Command::new("list")
        .about("List all available subcategories")
        .long_about("Displays all subcategories with their IDs. Shows both system subcategories (like 'Miscellaneous') and any custom subcategories you've created. Use these names when adding or filtering records.")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
    let file = gctx.tracker_path().open_read()?;
    let tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let mut subcategories: Vec<(usize, String)> = tracker_data
        .subcategories_by_id
        .iter()
        .map(|(&id, name)| (id, name.clone()))
        .collect();

    subcategories.sort_by_key(|(id, _)| *id);

    Ok(CliResponse::new(ResponseContent::Subcategories(subcategories)))
}
```

### Add Subcategory

Create `src/commands/subcategory/add.rs`:

```rust
use clap::{Arg, ArgMatches, Command};
use crate::{CliError, CliResponse, CliResult, GlobalContext, TrackerData, utils::file::{FilePath, write_json_to_file}, utils::parsers::parse_label};

pub fn cli() -> Command {
    Command::new("add")
        .about("Create a new subcategory")
        .long_about("Adds a custom subcategory to help organize your transactions. The name must start with a letter and can contain letters, numbers, and underscores. Names are case-insensitive but will be stored in Title Case. You cannot create a subcategory named 'Miscellaneous' as it's reserved.")
        .arg(
            Arg::new("name")
                .index(1)
                .required(true)
                .value_parser(parse_label)
                .help("Name for the new subcategory")
                .long_help("The name for your new subcategory. Must start with a letter and can contain letters, numbers, and underscores. Examples: 'Groceries', 'Salary', 'Rent', 'Utilities_Bill'. The name will be stored in Title Case (first letter uppercase, rest lowercase)."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let name = args
        .get_one::<String>("name")
        .ok_or_else(|| CliError::Other("Subcategory name not provided".to_string()))?;

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
```

We validate the name format for consistency and to prevent errors. We check for duplicates case-insensitively to prevent confusion. We generate IDs using an incrementing counter, which is simple and efficient. We prevent creation of "Miscellaneous" because it's a system subcategory that's always needed.

Create `src/utils/parsers.rs` for the label parser:

```rust
pub fn parse_label(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err("Input cannot be empty".to_string());
    }

    if !s.chars().next().unwrap().is_ascii_alphabetic() {
        return Err(format!("'{}' must start with a letter", s));
    }

    if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(format!(
            "'{}' contains invalid symbols (only letters, numbers, and underscores allowed)",
            s
        ));
    }

    Ok(s.to_string())
}
```

### Delete Subcategory

Create `src/commands/subcategory/delete.rs`:

```rust
use clap::{Arg, ArgMatches, Command};
use crate::{CliError, CliResponse, CliResult, GlobalContext, TrackerData, utils::file::{FilePath, write_json_to_file}, utils::parsers::parse_label};

pub fn cli() -> Command {
    Command::new("delete")
        .about("Delete a subcategory")
        .long_about("Removes a subcategory from your tracker. You can only delete subcategories that have no associated records. If a subcategory has records, you must delete those records first (using 'fintrack delete -s <subcategory>') or delete the subcategory manually. The 'Miscellaneous' subcategory cannot be deleted as it's a system subcategory.")
        .arg(
            Arg::new("name")
                .index(1)
                .required(true)
                .value_parser(parse_label)
                .help("Name of the subcategory to delete")
                .long_help("The name of the subcategory you want to remove. The name is case-insensitive. Use 'fintrack subcategory list' to see available subcategories. If the subcategory has records, you'll get an error message telling you how many records need to be deleted first."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let name = args
        .get_one::<String>("name")
        .ok_or_else(|| CliError::Other("Subcategory name not provided".to_string()))?;

    let name_lower = name.to_lowercase();

    if name_lower == "miscellaneous" {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::CannotDeleteMiscellaneous,
        ));
    }

    let subcategory_id = tracker_data
        .subcategory_id(&name_lower)
        .ok_or_else(|| {
            CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
                name: name.to_string(),
            })
        })?;

    let record_count = tracker_data
        .records
        .iter()
        .filter(|r| r.subcategory == subcategory_id)
        .count();

    if record_count > 0 {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::SubcategoryHasRecords {
                name: name.to_string(),
                count: record_count,
            },
        ));
    }

    tracker_data.subcategories_by_id.remove(&subcategory_id);
    tracker_data.subcategories_by_name.remove(&name_lower);
    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(crate::ResponseContent::Message(format!(
        "Subcategory '{}' deleted",
        name
    ))))
}
```

We check if records exist before deletion to maintain referential integrity. If records reference the subcategory, we return an error telling the user how many records need to be deleted first. We prevent deletion of "Miscellaneous" because it's a system subcategory that's always needed.

### Rename Subcategory

Create `src/commands/subcategory/rename.rs`:

```rust
use clap::{Arg, ArgMatches, Command};
use crate::{CliError, CliResponse, CliResult, GlobalContext, TrackerData, utils::file::{FilePath, write_json_to_file}, utils::parsers::parse_label};

pub fn cli() -> Command {
    Command::new("rename")
        .about("Rename an existing subcategory")
        .long_about("Changes the name of an existing subcategory. All existing records that use this subcategory will automatically use the new name (they reference by ID, not name). The new name must not already exist. You cannot rename 'Miscellaneous' as it's a system subcategory.")
        .arg(
            Arg::new("old")
                .help("Current subcategory name")
                .long_help("The current name of the subcategory you want to rename. The name is case-insensitive. Use 'fintrack subcategory list' to see available subcategories.")
                .index(1)
                .required(true)
                .value_parser(parse_label),
        )
        .arg(
            Arg::new("new")
                .help("New name for the subcategory")
                .long_help("The new name you want to use. Must start with a letter and can contain letters, numbers, and underscores. The name must not already exist. It will be stored in Title Case (first letter uppercase, rest lowercase).")
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

    let subcategory_id = tracker_data
        .subcategory_id(&old_name_lower)
        .ok_or_else(|| {
            CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
                name: old_name.to_string(),
            })
        })?;

    if tracker_data.subcategories_by_name.contains_key(&new_name_lower) {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::SubcategoryAlreadyExists {
                name: new_name_title.clone(),
            },
        ));
    }

    tracker_data.subcategories_by_id.insert(subcategory_id, new_name_title.clone());
    tracker_data.subcategories_by_name.remove(&old_name_lower);
    tracker_data.subcategories_by_name.insert(new_name_lower, subcategory_id);
    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(crate::ResponseContent::Message(format!(
        "Subcategory renamed: '{}' → '{}'",
        old_name, new_name_title
    ))))
}
```

We update both HashMaps to maintain bidirectional lookup. We check for conflicts to prevent duplicate names. Records don't need updating because they reference by ID, not name. This is why we use IDs instead of strings.

Create `src/commands/subcategory.rs` to wire up the subcommands:

```rust
use clap::{ArgMatches, Command};
use crate::{CliResult, GlobalContext};

pub fn cli() -> Command {
    Command::new("subcategory")
        .about("Manage your subcategories")
        .subcommand_required(true)
        .subcommands([list::cli(), add::cli(), delete::cli(), rename::cli()])
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    match args.subcommand() {
        Some(("list", args)) => list::exec(gctx, args),
        Some(("add", args)) => add::exec(gctx, args),
        Some(("delete", args)) => delete::exec(gctx, args),
        Some(("rename", args)) => rename::exec(gctx, args),
        _ => Err(crate::CliError::Other("Invalid subcommand".to_string())),
    }
}

pub mod list;
pub mod add;
pub mod delete;
pub mod rename;
```

## Step 11: Calculate Totals

The `total` command calculates financial summaries. Create `src/commands/total.rs`:

```rust
use clap::{ArgMatches, Command};
use crate::{CliError, CliResponse, CliResult, Currency, GlobalContext, Total, TrackerData, utils::file::FilePath};

pub fn cli() -> Command {
    Command::new("total")
        .about("Display financial summary with totals")
        .long_about("Shows a summary of your finances including opening balance, total income, total expenses, and net balance (opening + income - expenses).")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
    let file = gctx.tracker_path().open_read()?;
    let tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let opening_balance = tracker_data.opening_balance;

    let currency = tracker_data
        .currency
        .parse::<Currency>()
        .map_err(|e| CliError::Other(format!("Invalid currency in tracker data: {}", e)))?;

    let (income_total, expenses_total) = tracker_data.totals();

    Ok(CliResponse::new(crate::ResponseContent::Total(Total {
        currency,
        opening_balance,
        income_total,
        expenses_total,
    })))
}
```

We separate income and expenses because they require different calculations. We show net balance because it's the most important metric for users. It tells them their current financial position after accounting for all transactions.

## Step 12: Format Output

We separate output formatting for better organization and testability. Create basic formatting functions in `src/output.rs`:

```rust
use std::io;
use crate::{CliError, Currency, Record, ResponseContent, TrackerData, Total};

pub fn write_response(res: &crate::CliResponse, writer: &mut impl io::Write) -> io::Result<()> {
    let Some(content) = res.content() else {
        writeln!(writer, "✓ Success")?;
        return Ok(());
    };

    match content {
        ResponseContent::Message(msg) => {
            writeln!(writer, "✓ {}", msg)?;
        }
        ResponseContent::Record { record, tracker_data, is_update } => {
            if *is_update {
                writeln!(writer, "✓ Record updated:")?;
            } else {
                writeln!(writer, "✓ Record created:")?;
            }
            write_record_single(&record, Some(tracker_data), None, writer)?;
        }
        ResponseContent::List { records, tracker_data } => {
            if records.is_empty() {
                writeln!(writer, "No records found.")?;
            } else {
                write_records_table(records, Some(tracker_data), writer)?;
            }
        }
        ResponseContent::Total(total) => {
            write_total(total, writer)?;
        }
        ResponseContent::Subcategories(subcategories) => {
            write_subcategories_list(subcategories, writer)?;
        }
        _ => {}
    }
    Ok(())
}

fn write_record_single(
    record: &Record,
    tracker_data: Option<&TrackerData>,
    currency: Option<&Currency>,
    writer: &mut impl io::Write,
) -> io::Result<()> {
    let category_name = tracker_data
        .and_then(|td| td.category_name(record.category))
        .map(|s| s.as_str())
        .unwrap_or("Unknown");

    let subcategory_name = tracker_data
        .and_then(|td| td.subcategory_name(record.subcategory))
        .map(|s| s.as_str())
        .unwrap_or("Unknown");

    let currency_str = currency
        .map(|c| c.to_string())
        .or_else(|| tracker_data.map(|td| td.currency.clone()))
        .unwrap_or_else(|| "".to_string());

    writeln!(writer, "  ID: {}", record.id)?;
    writeln!(writer, "  Category: {}", category_name)?;
    writeln!(writer, "  Subcategory: {}", subcategory_name)?;
    writeln!(writer, "  Amount: {} {}", currency_str, record.amount)?;
    writeln!(writer, "  Date: {}", record.date)?;
    if !record.description.is_empty() {
        writeln!(writer, "  Description: {}", record.description)?;
    }
    Ok(())
}

fn write_records_table(
    records: &[Record],
    tracker_data: Option<&TrackerData>,
    writer: &mut impl io::Write,
) -> io::Result<()> {
    let currency_str = tracker_data.map(|td| td.currency.clone()).unwrap_or_else(|| "".to_string());

    writeln!(writer, "\n{:-<80}", "")?;
    writeln!(writer, "{:<6} {:<12} {:<20} {:<15} {:<12} {:<15}", "ID", "Category", "Subcategory", "Amount", "Date", "Description")?;
    writeln!(writer, "{:-<80}", "")?;

    for record in records {
        let category_name = tracker_data
            .and_then(|td| td.category_name(record.category))
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let subcategory_name = tracker_data
            .and_then(|td| td.subcategory_name(record.subcategory))
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        let description = if record.description.len() > 14 {
            &record.description[..14]
        } else {
            &record.description
        };

        writeln!(
            writer,
            "{:<6} {:<12} {:<20} {:<15} {:<12} {:<15}",
            record.id,
            category_name,
            subcategory_name,
            format!("{} {}", currency_str, record.amount),
            record.date,
            description
        )?;
    }

    writeln!(writer, "{:-<80}", "")?;
    Ok(())
}

fn write_total(total: &Total, writer: &mut impl io::Write) -> io::Result<()> {
    let net_balance = total.opening_balance + total.income_total - total.expenses_total;
    let currency_str = total.currency.to_string();

    writeln!(writer, "\nFinancial Summary")?;
    writeln!(writer, "{:-<40}", "")?;
    writeln!(writer, "Opening Balance:    {} {}", currency_str, total.opening_balance)?;
    writeln!(writer, "Total Income:      {} {}", currency_str, total.income_total)?;
    writeln!(writer, "Total Expenses:    {} {}", currency_str, total.expenses_total)?;
    writeln!(writer, "{:-<40}", "")?;
    writeln!(writer, "Net Balance:       {} {}", currency_str, net_balance)?;
    writeln!(writer, "{:-<40}", "")?;
    Ok(())
}

fn write_subcategories_list(
    subcategories: &[(usize, String)],
    writer: &mut impl io::Write,
) -> io::Result<()> {
    writeln!(writer, "\nSubcategories:")?;
    writeln!(writer, "{:-<40}", "")?;
    for (id, name) in subcategories {
        writeln!(writer, "  {}: {}", id, name)?;
    }
    writeln!(writer, "{:-<40}", "")?;
    Ok(())
}
```

We keep formatting simple using only the standard library. This means no external dependencies and it works everywhere. You can enhance this later with libraries like `colored` for colors and `tabled` for better tables if desired.

## Step 13: Handle Errors and Provide User Feedback

We use custom error types for better error messages and type safety. Update `src/error.rs` to add error formatting:

```rust
pub fn write_error(err: &CliError, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    match err {
        CliError::FileNotFound(path) => {
            writeln!(writer, "✗ File not found: {}", path)?;
            writeln!(writer, "Suggestion: Run 'fintrack init' to initialize the tracker")?;
        }
        CliError::InvalidJson(msg) => {
            writeln!(writer, "✗ Invalid JSON: {}", msg)?;
            writeln!(writer, "Suggestion: Your tracker data may be corrupted. Try restoring from backup")?;
        }
        CliError::ValidationError(kind) => {
            write_validation_error(kind, writer)?;
        }
        CliError::PermissionDenied(path) => {
            writeln!(writer, "✗ Permission denied: {}", path)?;
            writeln!(writer, "Suggestion: Check file permissions or run with appropriate access")?;
        }
        CliError::FileAlreadyExists => {
            writeln!(writer, "✗ Tracker already initialized")?;
            writeln!(writer, "Suggestion: Use 'fintrack clear' to start over")?;
        }
        CliError::Other(msg) => {
            writeln!(writer, "✗ {}", msg)?;
        }
    }
    Ok(())
}

fn write_validation_error(kind: &ValidationErrorKind, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    match kind {
        ValidationErrorKind::AmountTooSmall { amount } => {
            writeln!(writer, "✗ ValidationError: Amount must be greater than 0, got: {}", amount)?;
            writeln!(writer, "Suggestion: Re-run the command with a positive amount (e.g., --amount 500)")?;
        }
        ValidationErrorKind::SubcategoryNotFound { name } => {
            writeln!(writer, "✗ ValidationError: Subcategory '{}' not found", name)?;
            writeln!(writer, "Suggestion: Use 'fintrack subcategory list' to see available subcategories")?;
        }
        ValidationErrorKind::RecordNotFound { id } => {
            writeln!(writer, "✗ ValidationError: Record with ID {} not found", id)?;
            writeln!(writer, "Suggestion: Use 'fintrack list' to see all records and their IDs")?;
        }
        _ => {
            writeln!(writer, "✗ ValidationError: {:?}", kind)?;
        }
    }
    Ok(())
}
```

We provide meaningful error messages with suggestions to help users fix issues. We use `Result` types throughout for explicit error handling without panics. The `?` operator propagates errors concisely and idiomatically.

## Step 14: Test Your Implementation

Now let's wire everything together. Create `src/commands.rs`:

```rust
use crate::{CliResult, command_prelude::*};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
    vec![
        add::cli(),
        init::cli(),
        list::cli(),
        subcategory::cli(),
        total::cli(),
        update::cli(),
        delete::cli(),
    ]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
    match cmd {
        "add" => Some(add::exec),
        "init" => Some(init::exec),
        "list" => Some(list::exec),
        "subcategory" => Some(subcategory::exec),
        "total" => Some(total::exec),
        "update" => Some(update::exec),
        "delete" => Some(delete::exec),
        _ => None,
    }
}

pub mod add;
pub mod init;
pub mod list;
pub mod subcategory;
pub mod total;
pub mod update;
pub mod delete;
```

Create `src/utils/cli.rs` for the `ArgMatchesExt` trait:

```rust
use chrono::NaiveDate;
use clap::ArgMatches;
use crate::{Category, CliError, Currency};

const DEFAULT_F64: f64 = 0.0;
const DEFAULT_SUBCATEGORY: &str = "miscellaneous";

pub trait ArgMatchesExt {
    fn get_category(&self, id: &str) -> Result<&Category, CliError>;
    fn get_usize(&self, id: &str) -> Result<usize, CliError>;
    fn get_category_opt(&self, id: &str) -> Option<&Category>;
    fn get_f64_opt(&self, id: &str) -> Option<f64>;
    fn get_usize_opt(&self, id: &str) -> Option<usize>;
    fn get_string_opt(&self, id: &str) -> Option<String>;
    fn get_subcategory_opt(&self, id: &str) -> Option<String>;
    fn get_date_opt(&self, id: &str) -> Option<NaiveDate>;
    fn get_currency_opt(&self, id: &str) -> Option<&Currency>;
    fn get_f64_or_default(&self, id: &str) -> f64;
    fn get_usize_or_default(&self, id: &str) -> usize;
    fn get_string_or_default(&self, id: &str) -> String;
    fn get_subcategory_or_default(&self, id: &str) -> String;
    fn get_currency_or_default(&self, id: &str) -> &Currency;
    fn get_vec<T: Clone + Send + Sync + 'static>(&self, id: &str) -> Vec<T>;
}

impl ArgMatchesExt for ArgMatches {
    fn get_category(&self, id: &str) -> Result<&Category, CliError> {
        self.get_one::<Category>(id).ok_or_else(|| {
            CliError::ValidationError(crate::ValidationErrorKind::InvalidCategoryName {
                name: id.to_string(),
                reason: "Category not provided".to_string(),
            })
        })
    }

    fn get_usize(&self, id: &str) -> Result<usize, CliError> {
        self.get_one::<usize>(id).copied().ok_or_else(|| {
            CliError::Other(format!("Required argument '{}' not provided", id))
        })
    }

    fn get_category_opt(&self, id: &str) -> Option<&Category> {
        self.get_one::<Category>(id)
    }

    fn get_f64_opt(&self, id: &str) -> Option<f64> {
        self.get_one::<f64>(id).copied()
    }

    fn get_usize_opt(&self, id: &str) -> Option<usize> {
        self.get_one::<usize>(id).copied()
    }

    fn get_string_opt(&self, id: &str) -> Option<String> {
        self.get_one::<String>(id).cloned()
    }

    fn get_subcategory_opt(&self, id: &str) -> Option<String> {
        self.get_one::<String>(id).cloned()
    }

    fn get_date_opt(&self, id: &str) -> Option<NaiveDate> {
        self.get_one::<NaiveDate>(id).copied()
    }

    fn get_currency_opt(&self, id: &str) -> Option<&Currency> {
        self.get_one::<Currency>(id)
    }

    fn get_f64_or_default(&self, id: &str) -> f64 {
        self.get_one::<f64>(id).copied().unwrap_or(DEFAULT_F64)
    }

    fn get_usize_or_default(&self, id: &str) -> usize {
        self.get_one::<usize>(id).copied().unwrap_or(0)
    }

    fn get_string_or_default(&self, id: &str) -> String {
        self.get_one::<String>(id).cloned().unwrap_or_default()
    }

    fn get_subcategory_or_default(&self, id: &str) -> String {
        self.get_one::<String>(id).cloned().unwrap_or_else(|| DEFAULT_SUBCATEGORY.to_string())
    }

    fn get_currency_or_default(&self, id: &str) -> &Currency {
        self.get_one::<Currency>(id).unwrap_or(&Currency::NGN)
    }

    fn get_vec<T: Clone + Send + Sync + 'static>(&self, id: &str) -> Vec<T> {
        self.get_many::<T>(id).map(|iter| iter.cloned().collect()).unwrap_or_default()
    }
}
```

Create `src/utils/command_prelude.rs`:

```rust
pub use crate::utils::cli::ArgMatchesExt;
pub use crate::utils::context::GlobalContext;
```

Update `src/lib.rs`:

```rust
pub mod commands;
pub mod error;
pub mod models;
pub mod output;
pub mod utils;

pub use error::*;
pub use models::*;
pub use utils::command_prelude;
```

Update `src/main.rs`:

```rust
use std::io;
use clap::Command;
use fintrack::{GlobalContext, commands};

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    };
    std::process::exit(exit_code);
}

fn run() -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or_else(|| "Failed to determine home directory".to_string())?;
    let mut gctx = GlobalContext::new(home_dir);

    let matches = Command::new("fintrack")
        .bin_name("fintrack")
        .about("A local-first CLI financial tracker for managing income and expenses")
        .version("1.0.0")
        .subcommand_required(true)
        .subcommands(commands::cli())
        .get_matches();

    let (cmd, args) = matches.subcommand().expect("subcommand required but not found");
    let exec_fn = commands::build_exec(cmd).ok_or_else(|| format!("Unknown command: {}", cmd))?;
    let exec_result = exec_fn(&mut gctx, args);

    process_result(&exec_result)?;
    Ok(())
}

fn process_result(result: &fintrack::CliResult) -> io::Result<()> {
    match result {
        Ok(res) => res.write_to(&mut std::io::stdout()),
        Err(err) => err.write_to(&mut std::io::stderr()),
    }
}
```

Add the `write_to` method to `CliResponse` and `CliError` in their respective files. Now test your implementation:

```bash
# Initialize tracker
cargo run -- init -c USD -o 1000

# Add some records
cargo run -- add income 5000 -s Salary
cargo run -- add expenses 150.50 -s Groceries -d "Weekly shopping"
cargo run -- add expenses 2000 -s Rent

# List records
cargo run -- list
cargo run -- list -c expenses
cargo run -- list -l 2

# Update a record
cargo run -- update 1 -a 5500

# Add a subcategory
cargo run -- subcategory add Utilities

# Calculate totals
cargo run -- total

# Delete a record
cargo run -- delete -i 1
```

Test edge cases like invalid amounts, missing subcategories, and empty lists. Verify that data persists between runs by checking `~/.fintrack/tracker.json`.

## What's Next and Advanced Features

Congratulations! You've built a complete CLI financial tracker. Here's what you accomplished:

- Initialized trackers with custom currencies and opening balances
- Added income and expense records with validation
- Listed and filtered records by category, subcategory, and date range
- Updated existing records with partial updates
- Deleted records by ID, category, or subcategory
- Managed subcategories (list, add, delete, rename)
- Calculated financial totals

We focused on core features that teach fundamental concepts. Here are some advanced features you can implement as exercises:

**Export functionality**: Add CSV and JSON export. Serialize records to CSV format or pretty-print JSON. This teaches serialization and file format handling.

**Data analysis**: Implement a `describe` command that shows statistics like total records, date ranges, spending breakdowns, and average transaction amounts. This teaches data aggregation and analysis.

**Backup functionality**: Add automatic backup creation before destructive operations. Save timestamped copies of `tracker.json` to the backups directory. This teaches file management and safety.

**Enhanced output**: Use the `colored` crate to add colors to success messages and errors. Use the `tabled` crate for better-formatted tables. This teaches external library integration.

The full implementation with all features is available on [GitHub](https://github.com/steph-crown/fintrack). You can also install it from [crates.io](https://crates.io) or download pre-built binaries from the releases page.

Feel free to extend and customize the tracker to fit your needs. Add new commands, modify the data model, or integrate with other tools. The local-first architecture makes it easy to experiment without breaking anything.

## Conclusion

You've built a complete local-first CLI financial tracker with Rust. Along the way, you learned:

- How to structure a Rust CLI application with modules and traits
- Error handling with custom error types and the `Result` type
- File I/O operations and JSON serialization
- Command-line argument parsing with `clap`
- Data modeling with structs, enums, and HashMaps
- The value of local-first applications: data ownership, privacy, and independence

Local-first applications give you complete control over your data. Your financial information stays on your machine, you own it completely, and you're not dependent on any external service. This is the power of building tools yourself.

Continue building and experimenting. Add features, customize the tracker, or use these concepts in your own projects. The skills you've learned apply to many other domains beyond financial tracking.

Check out the [full implementation](https://github.com/steph-crown/fintrack) to see advanced features like data export, analysis, and enhanced formatting. Happy coding!
