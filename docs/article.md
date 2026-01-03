# How to Build a Local-First CLI Financial Tracker with Rust

Most financial apps require you to trust a company with your sensitive data, pay monthly fees, and hope the service stays online. But what if you could build a financial tracker that runs entirely on your computer, stores data in human-readable JSON files, and gives you complete control?

That's the power of local-first applications. Your data stays on your machine, you own it completely, and you're not dependent on any external service. In this tutorial, you'll build a complete CLI financial tracker using Rust that lets you track income and expenses with full control over your data.

By the end of this tutorial, you'll have a working command-line tool that can initialize a tracker, add records, list and filter transactions, update entries, delete records, manage subcategories, and calculate totals. You'll learn Rust concepts like traits, error handling, file I/O, JSON serialization, and CLI development.

## Prerequisites

Before you start, make sure you have:

- Rust installed (version 1.70 or later). If you don't have Rust installed, follow the [official installation guide](https://rust-book.cs.brown.edu/ch01-01-installation.html). You can verify your installation by running `rustc --version` in your terminal.
- Basic understanding of Rust syntax (variables, functions, structs, enums).
- Familiarity with command-line tools and terminal usage.
- Basic knowledge of JSON format.

## Commands We'll Build

This tutorial will guide you on how to implement these commands step-by-step:

- `init` - Initialize a new tracker
- `add` - Add income or expense records
- `list` - View and filter records
- `update` - Modify existing records
- `delete` - Remove records
- `subcategory` - Manage subcategories (list, add, delete, rename)
- `total` - Calculate financial totals

## Step 1: Set Up the Project

Start by creating a new Rust project. Open your terminal and run:

```bash
cargo new fintrack
cd fintrack
```

This creates a new directory called `fintrack` with a basic Rust project structure. Cargo is Rust's package manager and build tool. It handles dependencies, compilation, and project management.

Now, open `Cargo.toml` in your editor. This file defines your project's metadata and dependencies. You'll add several dependencies that the application needs:

```toml
[package]
name = "fintrack"
version = "1.0.0"
edition = "2021"

[dependencies]
chrono = "0.4.42"
clap = { version = "4.5.53", features = ["derive"] }
dirs = "6.0.0"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.148"
strum = { version = "0.26", features = ["derive"] }
```

Let's understand what each dependency does:

- **`chrono`**: Handles dates and times. You'll use it to parse dates from user input and format them for display.
- **`clap`**: A powerful library for building command-line interfaces. It handles argument parsing and validation of command line arguments.
- **`dirs`**: Provides a cross-platform way to find the user's home directory, where you'll store the tracker data.
- **`serde`** and **`serde_json`**: `serde` is Rust's serialization framework. Combined with `serde_json`, it lets you convert Rust structs to JSON and back. This is how you'll save and load your tracker data.
- **`strum`**: Provides macros to automatically generate useful code for enums, like converting them to strings and parsing strings into enums.

The `features = ["derive"]` for `clap` and `serde` enables their derive macros, which will let you use attributes like `#[derive(...)]` to automatically generate code.

## Step 2: Design the Data Model

Before writing any command logic, you need to define what data your tracker will store. You'll create structs and enums to represent this data.

### Understanding Structs and Enums

In Rust, **structs** group related data together, like a record in a database. **Enums** represent a value that can be one of several variants. You'll use both to model your financial data.

Create a new file `src/models.rs` and add this code:

```rust
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: usize,
    pub category: usize,
    pub amount: f64,
    pub subcategory: usize,
    pub description: String,
    pub date: String,
}
```

This defines a `Record` struct. Each record will represent one transaction (income or expense). The `#[derive(...)]` attribute automatically generates code for common traits:

- `Debug`: Lets you print the struct for debugging
- `Clone`: Lets you create copies of the struct
- `Serialize` and `Deserialize`: Lets you convert the struct to/from JSON

The `pub` keyword makes the struct and its fields accessible from other modules. In Rust, everything is private by default, so you use `pub` to expose things.

Now add the main data structure:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerData {
    pub currency: String,
    pub opening_balance: f64,
    pub categories: HashMap<String, usize>,
    pub subcategories_by_name: HashMap<String, usize>,
    pub subcategories_by_id: HashMap<usize, String>,
    pub records: Vec<Record>,
    pub next_record_id: usize,
    pub next_subcategory_id: usize,
    pub last_modified: String,
}
```

This struct holds all your tracker's data. Let's break down each field:

- `currency`: The currency code (like "NGN" or "USD")
- `opening_balance`: The starting balance when the tracker was initialized
- `categories`: Maps category names (like "income") to their IDs
- `subcategories_by_name`: Maps subcategory names to their IDs
- `subcategories_by_id`: Maps subcategory IDs back to their names (for reverse lookup)
- `records`: A vector (growable array) of all transactions
- `next_record_id` and `next_subcategory_id`: Counters to generate unique IDs
- `last_modified`: Timestamp of the last change

### Understanding HashMaps

`HashMap` is Rust's key-value store, like a dictionary in Python or an object in JavaScript. You use it here because you need fast lookups: "What's the ID for 'groceries' subcategory?" or "What's the name for ID 5?"

### Understanding Vectors

`Vec` is Rust's growable array. Unlike arrays, vectors can change size at runtime. You'll use it for `records` because you'll add transactions over time.

Now add enums for categories and currencies:

```rust
#[derive(clap::ValueEnum, Clone, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Category {
    Income,
    Expenses,
}

#[derive(clap::ValueEnum, Clone, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "uppercase")]
pub enum Currency {
    NGN,
    USD,
    GBP,
    EUR,
    CAD,
    AUD,
    JPY,
}
```

These enums represent fixed sets of values. `Category` can only be `Income` or `Expenses`. `Currency` can only be one of the listed currencies.

The `#[derive(strum::Display)]` automatically generates code to convert the enum to a string. `#[derive(strum::EnumString)]` generates code to parse a string into the enum. The `ascii_case_insensitive` attribute means users can type "income", "Income", or "INCOME" and they'll all work.

`clap::ValueEnum` lets `clap` use these enums directly as command-line argument values.

### Understanding Option

Sometimes a value might not exist. Rust uses `Option<T>` for this. `Option` can be:

- `Some(value)` - the value exists
- `None` - the value doesn't exist

You'll see `Option` used throughout the code. For example, when looking up a subcategory by name, it might not exist, so the function returns `Option<usize>`.

Now add helper methods to `TrackerData`. In Rust, you add methods to types using `impl` blocks:

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

Let's understand these methods:

- `&mut self`: This means the method borrows `self` mutably. You need `mut` because you're modifying the struct (like pushing a record).
- `&self`: This means the method borrows `self` immutably. You can read but not modify.
- `.get()`: HashMap's `get` method returns `Option<&V>` because the key might not exist. That's why `subcategory_id` returns `Option<usize>`.
- `.copied()`: Converts `Option<&usize>` to `Option<usize>` by copying the value.
- `.iter()`: Creates an iterator over the collection. Iterators let you process each item.
- `.find()`: Searches for the first item matching a condition. Returns `Option<Item>`.
- `.map()`: Transforms an `Option` value. If it's `Some(x)`, applies the function to `x`. If it's `None`, stays `None`.
- `.fold()`: Reduces a collection to a single value. Starts with an initial value `(0.0, 0.0)` (a tuple representing income and expenses), then applies a function to each item and accumulates the result.

The `totals()` method returns a tuple `(f64, f64)` representing total income and total expenses. Tuples group multiple values together.

Don't worry if some of this seems complex. You'll see these patterns repeatedly, and they'll become familiar.

Now add this to `src/lib.rs` to make the module available:

```rust
pub mod models;
```

## Step 3: Handle Errors Properly

Rust has a powerful error handling system using `Result<T, E>`. A `Result` can be:

- `Ok(value)` - the operation succeeded
- `Err(error)` - the operation failed

This forces you to handle errors explicitly, preventing crashes from unexpected failures.

Create `src/error.rs`:

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
    CategoryImmutable { category: usize },
    InvalidCategoryName { name: String, reason: String },
    InvalidName { name: String, reason: String },
    InvalidAmount { reason: String },
    TrackerAlreadyInitialized,
    InvalidSubcommand { subcommand: String },
}

#[derive(Debug)]
pub enum CliError {
    FileNotFound(String),
    InvalidJson(String),
    ValidationError(ValidationErrorKind),
    PermissionDenied(String),
    CorruptedData { backup_restored: bool, timestamp: String },
    FileAlreadyExists,
    Other(String),
}
```

You use enums for errors because they provide type safety. The compiler ensures you handle all error cases.

### Understanding the From Trait

Rust's `From` trait lets you convert one type to another. When you implement `From<A> for B`, you can convert `A` to `B`. This is useful for error handling because you can convert low-level errors (like file I/O errors) into your custom error type.

Add these implementations:

```rust
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

### Understanding Match

`match` is Rust's pattern matching. It's like a super-powered switch statement. You match on a value and handle each possible case. The compiler ensures you handle all cases (exhaustive matching).

Now add a method to write errors to the terminal. You'll implement this later in the output module, but for now, add the method signature:

```rust
impl CliError {
    pub fn write_to(&self, writer: &mut impl std::io::Write) -> io::Result<()> {
        crate::output::write_error(self, writer)
    }
}
```

The `&mut impl std::io::Write` means "any type that implements the `Write` trait". This lets you write to stdout, stderr, or a file using the same code.

Add `error` to `src/lib.rs`:

```rust
pub mod models;
pub mod error;
```

## Step 4: Create File Operations

You need to read and write JSON files. Instead of repeating file operations everywhere, you'll create a trait that adds methods to `Path` and `PathBuf`.

### Understanding Traits

Traits define shared behavior. When you implement a trait for a type, that type can use the trait's methods. Think of it like an interface in other languages.

Create `src/utils/file.rs`:

```rust
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

pub trait FilePath: AsRef<Path> {
    fn create_file_if_not_exists(&self) -> io::Result<File> {
        if let Some(parent) = self.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        File::options().write(true).create_new(true).open(self.as_ref())
    }

    fn read_file(&self) -> io::Result<File> {
        File::options().read(true).open(self.as_ref())
    }

    fn open_read_write(&self) -> io::Result<File> {
        File::options().read(true).write(true).open(self.as_ref())
    }

    fn open_read(&self) -> io::Result<File> {
        File::options().read(true).open(self.as_ref())
    }

    fn delete_if_exists(&self) -> io::Result<()> {
        let path = self.as_ref();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

impl<P: AsRef<Path>> FilePath for P {}
```

### Understanding the ? Operator

The `?` operator is Rust's error propagation shorthand. When you write `result?`, it means:

- If `result` is `Ok(value)`, use the value and continue
- If `result` is `Err(error)`, return the error immediately from the function

This is much cleaner than writing `match` statements everywhere.

The `impl<P: AsRef<Path>> FilePath for P {}` is a blanket implementation. It says "any type `P` that implements `AsRef<Path>` also implements `FilePath`". This means `Path`, `PathBuf`, `&Path`, `&PathBuf`, and even `String` (since `String` implements `AsRef<Path>`) all get these methods automatically.

Now add a helper function to write JSON to a file:

```rust
use serde_json::Value;

pub fn write_json_to_file(json: &Value, file: &mut File) -> io::Result<()> {
    file.set_len(0)?;
    file.rewind()?;
    serde_json::to_writer_pretty(file, json)?;
    Ok(())
}
```

`set_len(0)` truncates the file to zero bytes, and `rewind()` moves the file pointer back to the start. This ensures you overwrite the entire file, not append to it.

Add this to `src/lib.rs`:

```rust
pub mod models;
pub mod error;
pub mod utils;

pub use error::{CliError, ValidationErrorKind};
pub use models::{Category, Currency, Record, TrackerData};
```

Create `src/utils/mod.rs`:

```rust
pub mod file;
```

## Step 5: Set Up the CLI Structure

You'll use `clap` for argument parsing. `clap` provides excellent help text generation and validation.

Each command follows a pattern: a `cli()` function that defines arguments and an `exec()` function that contains the logic. This separation improves testability and keeps concerns clear.

Create `src/commands/mod.rs` (you'll create this file structure):

```rust
pub mod init;
pub mod add;
pub mod list;
pub mod update;
pub mod delete;
pub mod subcategory;
pub mod total;
```

Create `src/commands.rs`:

```rust
use crate::{CliResult, GlobalContext};
use clap::{ArgMatches, Command};

pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn cli() -> Vec<Command> {
    vec![
        init::cli(),
        add::cli(),
        list::cli(),
        update::cli(),
        delete::cli(),
        subcategory::cli(),
        total::cli(),
    ]
}

pub fn build_exec(cmd: &str) -> Option<Exec> {
    match cmd {
        "init" => Some(init::exec),
        "add" => Some(add::exec),
        "list" => Some(list::exec),
        "update" => Some(update::exec),
        "delete" => Some(delete::exec),
        "subcategory" => Some(subcategory::exec),
        "total" => Some(total::exec),
        _ => None,
    }
}

pub mod init;
pub mod add;
pub mod list;
pub mod update;
pub mod delete;
pub mod subcategory;
pub mod total;
```

Now create `src/utils/context.rs` for managing file paths:

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

    pub fn home_path(&self) -> &PathBuf {
        &self.home_path
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    pub fn backups_path(&self) -> &PathBuf {
        &self.backups_path
    }
}
```

This struct centralizes path management. The `join()` method safely combines path components.

Update `src/utils/mod.rs`:

```rust
pub mod file;
pub mod context;
```

Update `src/lib.rs`:

```rust
pub mod models;
pub mod error;
pub mod utils;
pub mod commands;

pub use error::{CliError, ValidationErrorKind};
pub use models::{Category, Currency, Record, TrackerData};
pub use utils::context::GlobalContext;

pub type CliResult = Result<CliResponse, CliError>;
```

You'll define `CliResponse` in the next step.

## Step 6: Create Response Types

Commands need to return results. Create `src/models.rs` additions (or a separate `src/output.rs` - we'll use `models.rs` for now):

Add to `src/models.rs`:

```rust
#[derive(Debug)]
pub enum ResponseContent {
    Message(String),
    Record {
        record: Record,
        tracker_data: TrackerData,
        is_update: bool,
    },
    List {
        records: Vec<Record>,
        tracker_data: TrackerData,
    },
    TrackerData(TrackerData),
    Total(Total),
    Categories(Vec<(usize, String)>),
    Subcategories(Vec<(usize, String)>),
}

#[derive(Debug, Clone)]
pub struct Total {
    pub opening_balance: f64,
    pub total_income: f64,
    pub total_expenses: f64,
    pub net_balance: f64,
    pub currency: Currency,
}

#[derive(Debug)]
pub struct CliResponse {
    content: Option<ResponseContent>,
}

impl CliResponse {
    pub fn new(content: ResponseContent) -> Self {
        CliResponse {
            content: Some(content),
        }
    }

    pub fn success() -> Self {
        CliResponse { content: None }
    }

    pub fn content(&self) -> Option<&ResponseContent> {
        self.content.as_ref()
    }

    pub fn write_to(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        crate::output::write_response(self, writer)
    }
}
```

You'll implement `write_response` in the output module. For now, create a simple `src/output.rs`:

```rust
use crate::{CliError, CliResponse, ResponseContent};

pub fn write_response(res: &CliResponse, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    let Some(content) = res.content() else {
        writeln!(writer, "✓ Success")?;
        return Ok(());
    };

    match content {
        ResponseContent::Message(msg) => {
            writeln!(writer, "✓ {}", msg)?;
        }
        ResponseContent::Record { record, .. } => {
            writeln!(writer, "✓ Record created:")?;
            writeln!(writer, "  ID: {}", record.id)?;
            writeln!(writer, "  Amount: {}", record.amount)?;
            // More formatting later
        }
        ResponseContent::List { records, .. } => {
            for record in records {
                writeln!(writer, "{:?}", record)?;
            }
        }
        ResponseContent::Total(total) => {
            writeln!(writer, "Opening Balance: {}", total.opening_balance)?;
            writeln!(writer, "Total Income: {}", total.total_income)?;
            writeln!(writer, "Total Expenses: {}", total.total_expenses)?;
            writeln!(writer, "Net Balance: {}", total.net_balance)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn write_error(err: &CliError, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    match err {
        CliError::FileNotFound(msg) => writeln!(writer, "Error: File not found: {}", msg),
        CliError::InvalidJson(msg) => writeln!(writer, "Error: Invalid JSON: {}", msg),
        CliError::ValidationError(kind) => {
            match kind {
                crate::ValidationErrorKind::AmountTooSmall { amount } => {
                    writeln!(writer, "Error: Amount must be greater than 0, got {}", amount)
                }
                crate::ValidationErrorKind::SubcategoryNotFound { name } => {
                    writeln!(writer, "Error: Subcategory '{}' not found", name)
                }
                crate::ValidationErrorKind::RecordNotFound { id } => {
                    writeln!(writer, "Error: Record with ID {} not found", id)
                }
                _ => writeln!(writer, "Error: Validation failed"),
            }
        }
        CliError::FileAlreadyExists => {
            writeln!(writer, "Error: Tracker already initialized. Use 'fintrack clear' to start fresh.")
        }
        _ => writeln!(writer, "Error: {}", err),
    }
}
```

Add to `src/lib.rs`:

```rust
pub mod models;
pub mod error;
pub mod utils;
pub mod commands;
pub mod output;

pub use error::{CliError, ValidationErrorKind};
pub use models::{Category, Currency, Record, TrackerData, CliResponse, ResponseContent, Total};
pub use utils::context::GlobalContext;

pub type CliResult = Result<CliResponse, CliError>;
```

## Step 7: Create Argument Parsing Helpers

You'll frequently extract values from command-line arguments. Create a trait to make this easier and consistent.

Create `src/utils/cli.rs`:

```rust
use chrono::NaiveDate;
use clap::ArgMatches;
use crate::{Category, CliError, Currency};

const DEFAULT_F64: f64 = 0.0;
const DEFAULT_USIZE: usize = 0;
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
    fn contains_id(&self, id: &str) -> bool;
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
        self.get_one::<usize>(id).copied().unwrap_or(DEFAULT_USIZE)
    }

    fn get_string_or_default(&self, id: &str) -> String {
        self.get_one::<String>(id).cloned().unwrap_or_default()
    }

    fn get_subcategory_or_default(&self, id: &str) -> String {
        self.get_one::<String>(id)
            .cloned()
            .unwrap_or_else(|| DEFAULT_SUBCATEGORY.to_string())
    }

    fn get_currency_or_default(&self, id: &str) -> &Currency {
        self.get_one::<Currency>(id).unwrap_or(&Currency::NGN)
    }

    fn get_vec<T: Clone + Send + Sync + 'static>(&self, id: &str) -> Vec<T> {
        self.get_many::<T>(id)
            .map(|iter| iter.cloned().collect())
            .unwrap_or_default()
    }

    fn contains_id(&self, id: &str) -> bool {
        self.contains_id(id)
    }
}
```

### Understanding ok_or_else

`ok_or_else` converts an `Option` to a `Result`. If the `Option` is `Some(value)`, it becomes `Ok(value)`. If it's `None`, it calls the closure to create an error. This is useful when a missing value should be an error.

### Understanding unwrap_or and unwrap_or_else

- `unwrap_or(default)`: If `Option` is `None`, use `default`
- `unwrap_or_else(|| ...)`: If `Option` is `None`, call the closure to compute a default

You use `unwrap_or_else` when the default value is expensive to compute or when you need a closure (like for `DEFAULT_SUBCATEGORY.to_string()`).

Update `src/utils/mod.rs`:

```rust
pub mod file;
pub mod context;
pub mod cli;
```

Create `src/utils/parsers.rs` for custom parsers:

```rust
use chrono::NaiveDate;
use crate::Category;

pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%d-%m-%Y")
        .map_err(|_| format!("'{}' is not in the format DD-MM-YYYY", s))
}

pub fn parse_category(s: &str) -> Result<Category, String> {
    s.parse::<Category>().map_err(|_| {
        format!("'{}' is not a valid category. Use 'income' or 'expenses'", s)
    })
}
```

Update `src/utils/mod.rs`:

```rust
pub mod file;
pub mod context;
pub mod cli;
pub mod parsers;
```

## Step 8: Implement the Init Command

The `init` command creates a new tracker. Create `src/commands/init.rs`:

```rust
use clap::{Arg, Command};
use crate::{CliError, CliResult, CliResponse, GlobalContext, Currency};
use crate::utils::file::write_json_to_file;
use crate::utils::cli::ArgMatchesExt;
use clap::ArgMatches;
use std::fs;

fn default_tracker_json(currency: Currency, opening_balance: f64) -> serde_json::Value {
    serde_json::json!({
        "currency": currency.to_string(),
        "opening_balance": opening_balance,
        "categories": {
            "income": 1,
            "expenses": 2
        },
        "subcategories_by_name": {
            "miscellaneous": 1
        },
        "subcategories_by_id": {
            "1": "miscellaneous"
        },
        "records": [],
        "next_record_id": 1,
        "next_subcategory_id": 2,
        "last_modified": chrono::Utc::now().to_rfc3339()
    })
}

pub fn cli() -> Command {
    Command::new("init")
        .about("Initialize a new financial tracker")
        .long_about(
            "Sets up your new financial tracker. This creates the necessary data file \
            (~/.fintrack/tracker.json) and a directory for future backups (~/.fintrack/backups/). \
            You can specify a currency and an optional opening balance. If a tracker already exists, \
            this command will return an error to prevent accidental data overwrite.",
        )
        .arg(
            Arg::new("currency")
                .short('c')
                .long("currency")
                .value_parser(clap::value_parser!(Currency))
                .default_value("ngn")
                .help("The currency to use for your tracker (e.g., NGN, USD, EUR)."),
        )
        .arg(
            Arg::new("opening")
                .short('o')
                .long("opening")
                .value_parser(clap::value_parser!(f64))
                .help("Your initial balance when starting the tracker."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let currency = args.get_currency_or_default("currency");
    let opening_balance = args.get_f64_or_default("opening");

    std::fs::create_dir_all(gctx.backups_path())?;

    let mut file = gctx.tracker_path().create_file_if_not_exists()
        .map_err(|_| CliError::FileAlreadyExists)?;

    let default_json = default_tracker_json(*currency, opening_balance);
    write_json_to_file(&default_json, &mut file)?;

    Ok(CliResponse::success())
}
```

### Understanding map_err

`map_err` transforms the error type in a `Result`. Here, you convert a generic file error into `FileAlreadyExists` when the file creation fails (because the file already exists).

The `create_file_if_not_exists` method uses `create_new(true)`, which fails if the file exists. This prevents accidentally overwriting existing data.

## Step 9: Implement the Add Command

The `add` command creates a new transaction record. Create `src/commands/add.rs`:

```rust
use clap::{Arg, Command};
use chrono::Local;
use crate::{
    CliError, CliResult, CliResponse, GlobalContext, Record, ResponseContent, TrackerData,
};
use crate::utils::file::write_json_to_file;
use crate::utils::cli::ArgMatchesExt;
use crate::utils::parsers::parse_category;
use clap::ArgMatches;

pub fn cli() -> Command {
    Command::new("add")
        .about("Record a new income or expense transaction")
        .long_about(
            "Adds a new financial record to your tracker. Category and amount are required. \
            The amount must be greater than 0. You can optionally specify a subcategory, \
            a description, and a date for the transaction. If no date is provided, \
            it defaults to today's date.",
        )
        .arg(
            Arg::new("category")
                .index(1)
                .required(true)
                .value_parser(parse_category)
                .help("The type of transaction. Use 'income' for money received or 'expenses' for money spent."),
        )
        .arg(
            Arg::new("amount")
                .index(2)
                .required(true)
                .value_parser(clap::value_parser!(f64))
                .help("The amount of money for this transaction. Must be a positive number greater than 0."),
        )
        .arg(
            Arg::new("subcategory")
                .short('s')
                .long("subcategory")
                .value_parser(clap::value_parser!(String))
                .default_value("miscellaneous")
                .help("A more specific category for this transaction (e.g., 'Groceries', 'Salary')."),
        )
        .arg(
            Arg::new("description")
                .short('d')
                .long("description")
                .value_parser(clap::value_parser!(String))
                .help("Any additional notes or description you want to add to this transaction."),
        )
        .arg(
            Arg::new("date")
                .short('D')
                .long("date")
                .value_parser(crate::utils::parsers::parse_date)
                .help("The date when this transaction occurred. Format: DD-MM-YYYY."),
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

### Understanding ok_or_else with ?

When `subcategory_id` returns `Option<usize>`, you use `ok_or_else` to convert it to `Result<usize, CliError>`. Then the `?` operator handles the error: if the subcategory doesn't exist, return the error immediately.

### Understanding map with Option

The `date` handling shows a common pattern:

1. `get_date_opt("date")` returns `Option<NaiveDate>`
2. `.map(|d| ...)` transforms the date if it exists
3. `.unwrap_or_else(|| ...)` provides a default if it doesn't

This is cleaner than an `if let` statement.

## Step 10: Implement the List Command

The `list` command displays records with optional filtering. Create `src/commands/list.rs`:

```rust
use clap::{Arg, ArgGroup, Command};
use chrono::NaiveDate;
use crate::{CliResult, CliResponse, GlobalContext, ResponseContent, TrackerData, Record};
use crate::utils::cli::ArgMatchesExt;
use crate::utils::parsers::parse_category;
use clap::ArgMatches;

pub fn cli() -> Command {
    Command::new("list")
        .about("View and filter your transaction records")
        .long_about(
            "Displays a list of your financial transaction records. You can filter records \
            by date range, category, or subcategory. You can also limit the output to the \
            first or last N records. Records are always sorted by date.",
        )
        .arg(
            Arg::new("first")
                .short('f')
                .long("first")
                .value_parser(clap::value_parser!(usize))
                .help("Display only the first N (oldest) records."),
        )
        .arg(
            Arg::new("last")
                .short('l')
                .long("last")
                .value_parser(clap::value_parser!(usize))
                .help("Display only the last N (most recent) records."),
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
                .value_parser(crate::utils::parsers::parse_date)
                .help("Filter records starting from this date (inclusive). Format: DD-MM-YYYY."),
        )
        .arg(
            Arg::new("end")
                .short('E')
                .long("end")
                .value_parser(crate::utils::parsers::parse_date)
                .help("Filter records up to this date (inclusive). Format: DD-MM-YYYY."),
        )
        .arg(
            Arg::new("category")
                .short('c')
                .long("category")
                .value_parser(parse_category)
                .help("Filter records by category (e.g., 'income', 'expenses')."),
        )
        .arg(
            Arg::new("subcategory")
                .short('s')
                .long("subcategory")
                .value_parser(clap::value_parser!(String))
                .help("Filter records by subcategory name."),
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
            let matches_category = category_filter
                .map(|expected_id| r.category == expected_id)
                .unwrap_or(true);

            let matches_subcategory = subcategory_filter
                .map(|expected_id| r.subcategory == expected_id)
                .unwrap_or(true);

            let matches_date = NaiveDate::parse_from_str(&r.date, "%d-%m-%Y")
                .map(|record_date| {
                    let after_start = start_date.map_or(true, |start| record_date >= start);
                    let before_end = end_date.map_or(true, |end| record_date <= end);
                    after_start && before_end
                })
                .unwrap_or(false);

            matches_category && matches_subcategory && matches_date
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

### Understanding and_then

`and_then` is like `map`, but the closure returns an `Option`. If the original `Option` is `None`, it stays `None`. If it's `Some(value)`, it calls the closure. If the closure returns `None`, the result is `None`. This is useful for chaining operations that might fail.

Here, `get_subcategory_opt` returns `Option<String>`, and `subcategory_id` returns `Option<usize>`. `and_then` chains them: if the name exists, look up the ID; otherwise, return `None`.

### Understanding map_or

`map_or` is a shorthand for a common pattern:

```rust
option.map_or(default, |value| transform(value))
```

This means: if the `Option` is `Some(value)`, transform it; otherwise, use `default`. In the filter, `map_or(true, ...)` means "if the filter is set, apply it; otherwise, include the record (true means include)".

### Understanding sort_by

`sort_by` takes a closure that compares two items. It returns `Ordering` (`Less`, `Equal`, or `Greater`). The `cmp` method on types that implement `Ord` returns an `Ordering`.

## Step 11: Implement the Update Command

The `update` command modifies existing records. Create `src/commands/update.rs`:

```rust
use clap::{Arg, Command};
use crate::{CliError, CliResult, CliResponse, GlobalContext, ResponseContent, TrackerData};
use crate::utils::file::write_json_to_file;
use crate::utils::cli::ArgMatchesExt;
use crate::utils::parsers::{parse_category, parse_date};
use clap::ArgMatches;

pub fn cli() -> Command {
    Command::new("update")
        .about("Modify an existing transaction record")
        .long_about(
            "Updates one or more fields of an existing financial record identified by its ID. \
            You only need to provide the fields you wish to change. The record ID is required. \
            Amount must be greater than 0. Use 'fintrack list' to find record IDs.",
        )
        .arg(
            Arg::new("record_id")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(usize))
                .help("The ID of the record you want to update."),
        )
        .arg(
            Arg::new("category")
                .short('c')
                .long("category")
                .value_parser(parse_category)
                .help("New category for the record."),
        )
        .arg(
            Arg::new("amount")
                .short('a')
                .long("amount")
                .value_parser(clap::value_parser!(f64))
                .help("New amount for the record. Must be greater than 0."),
        )
        .arg(
            Arg::new("subcategory")
                .short('s')
                .long("subcategory")
                .value_parser(clap::value_parser!(String))
                .help("New subcategory for the record."),
        )
        .arg(
            Arg::new("description")
                .short('d')
                .long("description")
                .value_parser(clap::value_parser!(String))
                .help("New description for the record."),
        )
        .arg(
            Arg::new("date")
                .short('D')
                .long("date")
                .value_parser(parse_date)
                .help("New date for the record. Format: DD-MM-YYYY."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let record_id = args.get_usize("record_id")?;

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

### Understanding transpose

`transpose` converts `Option<Result<T, E>>` to `Result<Option<T>, E>`. This is useful when you have an optional operation that might fail. Here, `subcategory_id` lookup is optional, but if you try to look it up and it fails, you want to return the error.

### Understanding iter_mut and find

`iter_mut()` creates an iterator that yields mutable references. `find()` searches for the first matching item and returns `Option<&mut Item>`. This lets you modify the record in place.

### Understanding if let

`if let Some(value) = option` is a shorthand for:

```rust
match option {
    Some(value) => { /* use value */ }
    None => {}
}
```

It's useful when you only care about one variant of an `Option` or `Result`.

## Step 12: Implement the Delete Command

The `delete` command removes records. Create `src/commands/delete.rs`:

```rust
use clap::{Arg, ArgGroup, Command};
use crate::{CliError, CliResult, CliResponse, GlobalContext, ResponseContent, TrackerData};
use crate::utils::file::write_json_to_file;
use crate::utils::cli::ArgMatchesExt;
use crate::utils::parsers::parse_category;
use clap::ArgMatches;
use std::collections::HashSet;

pub fn cli() -> Command {
    Command::new("delete")
        .about("Delete transaction records")
        .long_about(
            "Deletes one or more financial records from your tracker. You must specify \
            exactly one deletion method: by individual record IDs, by category, or by subcategory. \
            Be cautious, as this action cannot be undone.",
        )
        .arg(
            Arg::new("ids")
                .help("Comma-separated list of record IDs to delete.")
                .short('i')
                .long("ids")
                .value_parser(clap::value_parser!(usize))
                .action(clap::ArgAction::Append)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("by-cat")
                .help("Delete all records belonging to a specific category.")
                .short('c')
                .long("by-cat")
                .value_parser(parse_category),
        )
        .arg(
            Arg::new("by-subcat")
                .help("Delete all records belonging to a specific subcategory.")
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

    let ids_to_delete: HashSet<usize> = if args.contains_id("ids") {
        args.get_vec::<usize>("ids").into_iter().collect()
    } else if args.contains_id("by-cat") {
        let category = args.get_category("by-cat")?;
        let category_str = category.to_string();
        let category_id = tracker_data.category_id(&category_str);
        tracker_data
            .records
            .iter()
            .filter(|r| r.category == category_id)
            .map(|r| r.id)
            .collect()
    } else if args.contains_id("by-subcat") {
        let subcategory_name = args.get_subcategory_opt("by-subcat").unwrap();
        let subcategory_id = tracker_data
            .subcategory_id(&subcategory_name)
            .ok_or_else(|| {
                CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
                    name: subcategory_name,
                })
            })?;
        tracker_data
            .records
            .iter()
            .filter(|r| r.subcategory == subcategory_id)
            .map(|r| r.id)
            .collect()
    } else {
        return Err(CliError::Other("No deletion method specified".to_string()));
    };

    let initial_count = tracker_data.records.len();
    tracker_data.records.retain(|r| !ids_to_delete.contains(&r.id));
    let deleted_count = initial_count - tracker_data.records.len();

    if deleted_count == 0 {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::RecordNotFound { id: 0 },
        ));
    }

    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(ResponseContent::Message(format!(
        "Deleted {} record(s)",
        deleted_count
    ))))
}
```

### Understanding HashSet

`HashSet` is like `HashMap` but only stores keys (no values). It's useful for fast membership testing: "Is this ID in the set?" You use it here to track which record IDs to delete.

### Understanding retain

`retain` removes items from a vector based on a predicate. It keeps items where the predicate returns `true` and removes items where it returns `false`. This is more efficient than creating a new vector with `filter` and `collect`.

## Step 13: Implement Subcategory Commands

Subcategories let users organize transactions. Create `src/commands/subcategory.rs`:

```rust
use clap::{ArgMatches, Command};
use crate::{CliResult, GlobalContext};

pub fn cli() -> Command {
    Command::new("subcategory")
        .about("Manage your subcategories")
        .subcommand_required(true)
        .subcommands([
            list::cli(),
            add::cli(),
            delete::cli(),
            rename::cli(),
        ])
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

Create `src/commands/subcategory/list.rs`:

```rust
use clap::Command;
use crate::{CliResult, CliResponse, GlobalContext, ResponseContent, TrackerData};
use clap::ArgMatches;

pub fn cli() -> Command {
    Command::new("list")
        .about("View all available subcategories")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
    let file = gctx.tracker_path().open_read()?;
    let tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let subcategories: Vec<(usize, String)> = tracker_data
        .subcategories_by_id
        .iter()
        .map(|(id, name)| (*id, name.clone()))
        .collect();

    Ok(CliResponse::new(ResponseContent::Subcategories(subcategories)))
}
```

Create `src/commands/subcategory/add.rs`:

```rust
use clap::{Arg, Command};
use crate::{CliError, CliResult, CliResponse, GlobalContext, TrackerData};
use crate::utils::file::write_json_to_file;
use crate::utils::cli::ArgMatchesExt;
use clap::ArgMatches;

fn normalize_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

pub fn cli() -> Command {
    Command::new("add")
        .about("Create a new subcategory")
        .arg(
            Arg::new("name")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("The name of the new subcategory to add."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let name = args.get_string_opt("name").unwrap();
    let normalized = normalize_name(&name);

    if normalized.to_lowercase() == "miscellaneous" {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::CannotDeleteMiscellaneous,
        ));
    }

    if tracker_data.subcategories_by_name.contains_key(&normalized.to_lowercase()) {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::SubcategoryAlreadyExists {
                name: normalized,
            },
        ));
    }

    let id = tracker_data.next_subcategory_id;
    tracker_data.next_subcategory_id += 1;

    tracker_data
        .subcategories_by_name
        .insert(normalized.to_lowercase(), id);
    tracker_data.subcategories_by_id.insert(id, normalized.clone());

    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(ResponseContent::Message(format!(
        "Subcategory '{}' added with ID {}",
        normalized, id
    ))))
}
```

Create `src/commands/subcategory/delete.rs`:

```rust
use clap::{Arg, Command};
use crate::{CliError, CliResult, CliResponse, GlobalContext, TrackerData};
use crate::utils::file::write_json_to_file;
use crate::utils::cli::ArgMatchesExt;
use clap::ArgMatches;

pub fn cli() -> Command {
    Command::new("delete")
        .about("Delete a subcategory")
        .arg(
            Arg::new("name")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("The name of the subcategory to delete."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let name = args.get_string_opt("name").unwrap();
    let normalized = name.to_lowercase();

    if normalized == "miscellaneous" {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::CannotDeleteMiscellaneous,
        ));
    }

    let id = tracker_data
        .subcategories_by_name
        .get(&normalized)
        .ok_or_else(|| {
            CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
                name: name.clone(),
            })
        })?;

    let record_count = tracker_data
        .records
        .iter()
        .filter(|r| r.subcategory == *id)
        .count();

    if record_count > 0 {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::SubcategoryHasRecords {
                name: name.clone(),
                count: record_count,
            },
        ));
    }

    tracker_data.subcategories_by_name.remove(&normalized);
    tracker_data.subcategories_by_id.remove(id);

    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(ResponseContent::Message(format!(
        "Subcategory '{}' deleted",
        name
    ))))
}
```

Create `src/commands/subcategory/rename.rs`:

```rust
use clap::{Arg, Command};
use crate::{CliError, CliResult, CliResponse, GlobalContext, TrackerData};
use crate::utils::file::write_json_to_file;
use crate::utils::cli::ArgMatchesExt;
use clap::ArgMatches;

fn normalize_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

pub fn cli() -> Command {
    Command::new("rename")
        .about("Rename an existing subcategory")
        .arg(
            Arg::new("old")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("The current name of the subcategory."),
        )
        .arg(
            Arg::new("new")
                .index(2)
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("The new name for the subcategory."),
        )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let mut file = gctx.tracker_path().open_read_write()?;
    let mut tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let old_name = args.get_string_opt("old").unwrap();
    let new_name = args.get_string_opt("new").unwrap();
    let normalized_old = old_name.to_lowercase();
    let normalized_new = normalize_name(&new_name);

    if normalized_old == "miscellaneous" {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::CannotDeleteMiscellaneous,
        ));
    }

    let id = tracker_data
        .subcategories_by_name
        .get(&normalized_old)
        .ok_or_else(|| {
            CliError::ValidationError(crate::ValidationErrorKind::SubcategoryNotFound {
                name: old_name.clone(),
            })
        })?;

    if tracker_data.subcategories_by_name.contains_key(&normalized_new.to_lowercase()) {
        return Err(CliError::ValidationError(
            crate::ValidationErrorKind::SubcategoryAlreadyExists {
                name: normalized_new,
            },
        ));
    }

    tracker_data.subcategories_by_name.remove(&normalized_old);
    tracker_data
        .subcategories_by_name
        .insert(normalized_new.to_lowercase(), *id);
    tracker_data.subcategories_by_id.insert(*id, normalized_new.clone());

    tracker_data.last_modified = chrono::Utc::now().to_rfc3339();

    let tracker_json = serde_json::json!(tracker_data);
    write_json_to_file(&tracker_json, &mut file)?;

    Ok(CliResponse::new(ResponseContent::Message(format!(
        "Subcategory '{}' renamed to '{}'",
        old_name, normalized_new
    ))))
}
```

## Step 14: Implement the Total Command

The `total` command calculates financial summaries. Create `src/commands/total.rs`:

```rust
use clap::Command;
use crate::{CliResult, CliResponse, GlobalContext, ResponseContent, Total, Currency, TrackerData};
use clap::ArgMatches;

pub fn cli() -> Command {
    Command::new("total")
        .about("Display financial summary with totals")
}

pub fn exec(gctx: &mut GlobalContext, _args: &ArgMatches) -> CliResult {
    let file = gctx.tracker_path().open_read()?;
    let tracker_data: TrackerData = serde_json::from_reader(&file)?;

    let currency: Currency = tracker_data.currency.parse().unwrap_or(Currency::NGN);
    let (total_income, total_expenses) = tracker_data.totals();
    let net_balance = tracker_data.opening_balance + total_income - total_expenses;

    let total = Total {
        opening_balance: tracker_data.opening_balance,
        total_income,
        total_expenses,
        net_balance,
        currency,
    };

    Ok(CliResponse::new(ResponseContent::Total(total)))
}
```

## Step 15: Wire Up the Main Function

Finally, create `src/main.rs`:

```rust
use std::io;
use clap::Command;
use fintrack::{GlobalContext, commands};

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(e) => {
            e.write_to(&mut std::io::stderr()).expect("Failed to write error");
            1
        }
    };
    std::process::exit(exit_code);
}

fn run() -> Result<(), String> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Failed to determine home directory".to_string())?;

    let mut gctx = GlobalContext::new(home_dir);

    let matches = Command::new("fintrack")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A local-first CLI financial tracker")
        .long_about(
            "FinTrack is a powerful, local-first command-line financial tracker. \
            It allows you to manage your income and expenses directly on your machine, \
            ensuring complete data ownership and privacy.",
        )
        .bin_name("fintrack")
        .subcommand_required(true)
        .subcommands(commands::cli())
        .get_matches();

    let (cmd, args) = matches
        .subcommand()
        .expect("subcommand required but not found");

    let exec_fn = commands::build_exec(cmd)
        .ok_or_else(|| format!("Unknown command: {}", cmd))?;

    let exec_result = exec_fn(&mut gctx, args);
    process_result(&exec_result)?;

    Ok(())
}

fn process_result(result: &fintrack::CliResult) -> Result<(), String> {
    match result {
        Ok(res) => res.write_to(&mut std::io::stdout())
            .map_err(|e| format!("Failed to write response: {}", e))?,
        Err(err) => err.write_to(&mut std::io::stderr())
            .map_err(|e| format!("Failed to write error: {}", e))?,
    }
    Ok(())
}
```

Update `src/lib.rs`:

```rust
pub mod models;
pub mod error;
pub mod utils;
pub mod commands;
pub mod output;

pub use error::{CliError, ValidationErrorKind};
pub use models::{Category, Currency, Record, TrackerData, CliResponse, ResponseContent, Total};
pub use utils::context::GlobalContext;

pub type CliResult = Result<CliResponse, CliError>;
```

## Testing Your Application

Build and test your application:

```bash
cargo build
cargo run -- init --currency USD --opening 1000
cargo run -- add income 500 --subcategory salary
cargo run -- add expenses 50 --subcategory groceries
cargo run -- list
cargo run -- total
```

## What's Next and Advanced Features

Congratulations! You've built a complete local-first CLI financial tracker. The application you've created includes:

- Data persistence in JSON format
- Full CRUD operations for financial records
- Subcategory management
- Financial calculations
- Comprehensive error handling
- Type-safe command-line argument parsing

### Advanced Features to Explore

The full implementation includes additional features you can explore and implement:

- **Export**: Export data to CSV format for analysis in spreadsheet applications
- **Describe**: Generate visual charts and statistics about your spending patterns
- **Enhanced Output Formatting**: Use libraries like `colored` and `tabled` for beautiful terminal output with colors and tables

You can find the complete implementation with all features, including advanced output formatting, export functionality, and more, in the [full GitHub repository](https://github.com/yourusername/fintrack). The repository also includes installation instructions for downloading the binary or installing via Cargo.

## Conclusion

You've learned how to:

- Structure a Rust CLI application with proper error handling
- Use traits to extend functionality
- Work with JSON serialization
- Parse and validate command-line arguments
- Manage file I/O operations
- Implement a complete data model with relationships

The patterns you've learned here apply to many Rust applications. Traits, error handling with `Result`, and the ownership system are fundamental to writing idiomatic Rust code.

Keep building, and happy tracking!
