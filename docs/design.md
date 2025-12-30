# FinTrack: CLI Financial Tracker - Design Document

**Status:** In Progress (MVP Phase)
**Version:** 1.0
**Language:** Rust
**Last Updated:** December 2025

---

## 1. Overview

FinTrack is a command-line financial tracking tool that enables users to manage their income and expenses locally, without any remote data storage. All financial data is persisted to the user's filesystem, giving them complete ownership and control. The tool is designed for simplicity and reliability, with a focus on data integrity and recovery.

**Core Philosophy:** Local-first, user-owned data. No cloud. No dependencies on external servers.

---

## 2. Architecture

### 2.1 High-Level Design

FinTrack follows a modular, layered architecture:

```
┌─────────────────────────────────────────────────────┐
│                    CLI Layer (main.rs)              │
│           Clap Argument Parsing & Dispatch          │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│              Process Layer (modules/)               │
│  init, add, delete, update, list, category,         │
│  subcategory, clear, total, describe, dump, export  │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│            Business Logic Layer (lib.rs)            │
│  Validation, file I/O, data transformation,         │
│  backup/recovery, serialization                     │
└────────────────────┬────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────┐
│           Persistent Storage Layer                  │
│  ~/.fintrack/tracker.json (main data)               │
│  ~/.fintrack/backups/tracker.backup.*.json (backup) │
└─────────────────────────────────────────────────────┘
```

### 2.2 Module Structure

```
fintrack/
├── src/
│   ├── main.rs                 # CLI entry point, argument dispatch
│   ├── lib.rs                  # Shared functions, validation, I/O
│   ├── error.rs                # Error types and handling
│   ├── models.rs               # Data structures (TrackerData, Record, etc.)
│   ├── storage.rs              # File I/O, backup/recovery logic
│   ├── modules/
│   │   ├── init.rs             # Initialize tracker
│   │   ├── add.rs              # Add record
│   │   ├── delete.rs           # Delete records (by ID, category, subcategory)
│   │   ├── update.rs           # Update records
│   │   ├── list.rs             # List records with filters
│   │   ├── category.rs         # Category operations (view only for now)
│   │   ├── subcategory.rs      # Subcategory CRUD
│   │   ├── clear.rs            # Clear all data
│   │   ├── total.rs            # Compute and display totals
│   │   ├── describe.rs         # EDA (post-MVP)
│   │   ├── dump.rs             # Dump JSON to console
│   │   └── export.rs           # Export to CSV (post-MVP)
│   └── utils.rs                # Formatting, helpers
├── Cargo.toml
└── README.md
```

---

## 3. Data Model

### 3.1 File Structure

**Primary File:** `~/.fintrack/tracker.json`

```json
{
  "version": 1,
  "currency": "NGN",
  "created_at": "2025-12-30T10:30:00Z",
  "last_modified": "2025-12-30T14:45:30Z",
  "categories": {
    "Income": 1,
    "Expenses": 2
  },
  "subcategories_by_id": {
    "1": "Miscellaneous",
    "2": "Groceries",
    "3": "Wages"
  },
  "subcategories_by_name": {
    "Miscellaneous": 1,
    "Groceries": 2,
    "Wages": 3
  },
  "next_subcategory_id": 4,
  "records": [
    {
      "id": 1,
      "category": 1,
      "subcategory": 1,
      "description": "Monthly salary",
      "amount": 4000.0,
      "date": "2025-12-30"
    }
  ],
  "next_record_id": 2
}
```

**Backup File:** `~/.fintrack/backups/tracker.backup.2025-12-30T14-45-30Z.json`

Backups use the same structure as the primary file, with ISO 8601 timestamps in the filename for easy sorting and identification.

### 3.2 Data Structures (Rust Models)

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
struct TrackerData {
    version: u32,
    currency: String,
    created_at: String,        // ISO 8601
    last_modified: String,     // ISO 8601
    categories: HashMap<String, u32>,
    subcategories_by_id: HashMap<u32, String>,
    subcategories_by_name: HashMap<String, u32>,
    next_subcategory_id: u32,
    records: Vec<Record>,
    next_record_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Record {
    id: u64,
    category: u32,             // ID from categories map
    subcategory: u32,          // ID from subcategories map
    description: String,
    amount: f64,               // Always positive; sign determined by category
    date: String,              // Format: DD-MM-YYYY
}

#[derive(Debug)]
struct ProcessResponse {
    success: bool,
    stdout: String,
}

#[derive(Debug)]
enum ProcessError {
    FileNotFound(String),
    InvalidJson(String),
    ValidationError(ValidationErrorKind),
    PermissionDenied(String),
    CorruptedData { backup_restored: bool, timestamp: String },
    Other(String),
}

enum ValidationErrorKind {
  AmountTooSmall {
    amount: f64
  },
  InvalidDate {
    provided: String,
    expected_format: String
  },
  SubcategoryNotFound {
    name: String
  },
  SubcategoryAlreadyExists {
    name: String
  },
  RecordNotFound {
    id: usize
  },
  SubcategoryHasRecords {
    name: String,
    count: usize
  },
  CannotDeleteMiscellaneous,
  CategoryImmutable {
    category: usize
  },
  InvalidCategoryName {
    name: String,
    reason: String
  },
  InvalidName {
    name: String,
    reason: String
  },
  InvalidAmount {
    reason: String
  }
}
```

### 3.3 Validation Rules

**Category:**

- Immutable: `Income`, `Expenses` (cannot be deleted or renamed)
- Used as integers (1 = Income, 2 = Expenses) in records
- Case-insensitive in CLI but stored as exact case shown above

**Subcategory:**

- Case-insensitive in CLI but stored in Title Case (e.g., "Wages", "Groceries")
- Must be alphanumeric and start with a letter
- Cannot be named "Miscellaneous" more than once
- Unique per tracker
- Cannot be deleted if records reference it; user must delete records first or use `--by-subcat` flag
- Cannot rename to an existing subcategory name

**Record:**

- `amount`: Must be positive (>0), stored as float
- `date`: Format DD-MM-YYYY; must be a valid calendar date (leap years, etc.)
- `description`: Optional; can contain escape sequences like `\n`
- `category`: Must exist in categories map (Income or Expenses)
- `subcategory`: Must exist in subcategories map; defaults to "Miscellaneous" if not provided

**Currency:**

- Accepted: NGN, USD, GBP, EUR, CAD, AUD, JPY, INR (extensible list)
- Set during `fintrack init` and cannot be changed (immutable per tracker)

---

## 4. Commands

### 4.1 Initialization

```bash
fintrack init [--currency <CURRENCY>]
```

**Behavior:**

- Creates `~/.fintrack/` directory if it doesn't exist
- Creates `~/.fintrack/backups/` directory
- Initializes `~/.fintrack/tracker.json` with default structure
- Sets currency (defaults to NGN if not specified)
- Creates default categories (Income, Expenses) with IDs 1 and 2
- Creates default subcategory "Miscellaneous" with ID 1
- Sets `created_at` and `last_modified` timestamps
- If tracker already exists, displays error: `"Tracker already initialized. Use 'fintrack clear' to start over."`
- On success, displays: `"✓ Tracker initialized with currency: NGN"`

---

### 4.2 Adding Records

```bash
fintrack add \
  --category <CATEGORY> \
  --amount <AMOUNT> \
  [--subcategory <SUBCATEGORY>] \
  [--description <DESCRIPTION>] \
  [--date <DATE>]
```

**Short flags available:**

```bash
fintrack add -c Income -a 4000
fintrack add -c Expenses -a 150.50 -s Groceries -d "Weekly shop" --date 28-12-2025
```

| Long Flag       | Short  | Required | Default       |
| --------------- | ------ | -------- | ------------- |
| `--category`    | `-c`   | Yes      | —             |
| `--amount`      | `-a`   | Yes      | —             |
| `--subcategory` | `-s`   | No       | Miscellaneous |
| `--description` | `-d`   | No       | (empty)       |
| `--date`        | (none) | No       | Today         |

**Behavior:**

- Flags can appear in any order
- `--category` and `--amount` are required
- `--subcategory` defaults to "Miscellaneous" if omitted
- `--date` defaults to today (current date in DD-MM-YYYY format) if omitted
- `--description` is optional; empty string allowed
- Validates category exists, amount > 0, date is valid, subcategory exists
- Auto-generates record ID (incremental)
- Updates `last_modified` timestamp
- Creates backup before mutation
- On success, displays the full created record with generated ID and formatted date
- Example output:
  ```
  ✓ Record added:
    ID: 1 | Income | Wages | 4000.00 NGN | 30-12-2025 | Monthly salary
  ```

---

### 4.3 Deleting Records

#### Delete by ID(s):

```bash
fintrack delete <ID1>,<ID2>,<ID3>
fintrack delete 1,5,10
```

**Behavior:**

- Accepts comma-separated list of record IDs
- Validates all IDs exist before deletion
- If any ID doesn't exist, displays error and cancels entire operation (atomic)
- Updates `last_modified` timestamp
- Creates backup before mutation
- On success, displays deleted records in strikethrough or red formatting
- Example output:
  ```
  ✓ Deleted 2 records:
    ~~ID: 1 | Income | Wages | 4000.00 NGN | 30-12-2025~~
    ~~ID: 5 | Expenses | Groceries | 150.00 NGN | 28-12-2025~~
  ```

#### Delete by Category:

```bash
fintrack delete --by-cat Income
fintrack delete -C Income
```

**Short flag:** `-C, --by-cat`

**Behavior:**

- Deletes all records with the specified category
- Displays confirmation prompt: `"Delete all X records in category '<CATEGORY>'? This cannot be undone. (yes/no)"`
- If user confirms, performs deletion and displays all deleted records
- If user cancels, displays: `"Deletion cancelled."`
- Updates `last_modified` timestamp, creates backup before mutation

#### Delete by Subcategory:

```bash
fintrack delete --by-subcat Groceries
fintrack delete -S Groceries
```

**Short flag:** `-S, --by-subcat`

**Behavior:**

- Same as `--by-cat` but for subcategories
- Confirmation prompt and deletion logic identical

---

### 4.4 Updating Records

```bash
fintrack update <ID> \
  [--category <CATEGORY>] \
  [--amount <AMOUNT>] \
  [--subcategory <SUBCATEGORY>] \
  [--description <DESCRIPTION>] \
  [--date <DATE>]
```

**Short flags available:**

```bash
fintrack update 5 -c Income -a 5000
fintrack update 5 -s Wages -d "Updated description"
```

| Long Flag       | Short  | Optional |
| --------------- | ------ | -------- |
| `--category`    | `-c`   | Yes      |
| `--amount`      | `-a`   | Yes      |
| `--subcategory` | `-s`   | Yes      |
| `--description` | `-d`   | Yes      |
| `--date`        | (none) | Yes      |

**Behavior:**

- At least one flag (beyond ID) must be provided
- Validates record ID exists
- Validates each provided field against rules
- Updates only specified fields; others remain unchanged
- Updates `last_modified` timestamp
- Creates backup before mutation
- On success, displays the full updated record
- Example output:
  ```
  ✓ Record updated:
    ID: 1 | Income | Wages | 5000.00 NGN | 30-12-2025 | Monthly salary (increased)
  ```

---

### 4.5 Listing Records

#### List all:

```bash
fintrack list
```

#### List with limits:

```bash
fintrack list -f 5      # First 5 records
fintrack list -l 10     # Last 10 records
```

**Short flags available:**

```bash
fintrack list -c Income
fintrack list -s Groceries
fintrack list --start 01-12-2025 --end 31-12-2025
```

| Long Flag       | Short  | Optional |
| --------------- | ------ | -------- |
| `--first`       | `-f`   | Yes      |
| `--last`        | `-l`   | Yes      |
| `--category`    | `-c`   | Yes      |
| `--subcategory` | `-s`   | Yes      |
| `--start`       | (none) | Yes      |
| `--end`         | (none) | Yes      |

**Behavior:**

- `--first N`: Display the first N records (oldest N)
- `--last N`: Display the last N records (most recent N)
- `-c <CATEGORY>`: Displays all records in the specified category
- `-s <SUBCATEGORY>`: Displays all records in the specified subcategory
- `--start` and `--end`: Both optional, defaults to -Infinity and today respectively
- Date format: DD-MM-YYYY
- Inclusive on both ends
- Sorted by date
- All variants sorted by date (oldest first)
- Table format with columns: ID, Category, Subcategory, Amount, Currency, Date, Description
- If no records exist: `"No records found."`

---

### 4.6 Category Operations

```bash
fintrack category list
```

**Behavior:**

- Displays list of all categories with their IDs
- Output:
  ```
  Categories:
    1 - Income
    2 - Expenses
  ```

**Note:** Categories cannot be created, deleted, or renamed. They are immutable.

---

### 4.7 Subcategory Operations

#### List subcategories:

```bash
fintrack subcategory list
```

**Behavior:**

- Displays all subcategories with IDs and creation dates
- Output:
  ```
  Subcategories:
    1 - Miscellaneous (created: 2025-12-30T10:30:00Z)
    2 - Groceries (created: 2025-12-30T10:35:15Z)
    3 - Wages (created: 2025-12-30T10:40:22Z)
  ```

#### Add subcategory:

```bash
fintrack subcategory add <NAME>
```

**Behavior:**

- `<NAME>` must be alphanumeric, start with a letter, and be unique
- Case-insensitive; stored in Title Case (first letter uppercase, rest lowercase)
- Validates name doesn't already exist
- Auto-generates subcategory ID
- Updates `last_modified` timestamp
- Creates backup before mutation
- On success: `"✓ Subcategory 'Groceries' added (ID: 2)"`

#### Delete subcategory:

```bash
fintrack subcategory delete <NAME>
```

**Behavior:**

- Validates subcategory exists
- Checks if any records reference this subcategory
- If records exist, displays error: `"Cannot delete 'Groceries'—it has 5 records. Delete those first using 'fintrack delete --by-subcat Groceries', or manually delete individual records."`
- If no records, prompts for confirmation: `"Delete subcategory 'Groceries'? (yes/no)"`
- On confirmation, deletes and displays: `"✓ Subcategory 'Groceries' deleted."`
- Cannot delete "Miscellaneous" (system subcategory)
- Updates `last_modified` timestamp, creates backup before mutation

#### Rename subcategory:

```bash
fintrack subcategory update --old Groceries --new Food
fintrack subcategory update -o Groceries -n "Food & Groceries"
```

**Short flags:** `-o, --old` | `-n, --new`

**Behavior:**

- Validates old subcategory exists
- Validates new name doesn't already exist
- Updates the name in `subcategories_by_id` and `subcategories_by_name`
- Records continue to reference the ID (no record updates needed)
- On success: `"✓ Subcategory renamed: 'Groceries' → 'Food & Groceries'"`
- Updates `last_modified` timestamp, creates backup before mutation

---

### 4.8 Totals

```bash
fintrack total
```

**Behavior:**

- Computes total income, total expenses, and net (income - expenses)
- Displays with currency symbol
- Output:
  ```
  Financial Summary:
    Total Income:    125,500.00 NGN
    Total Expenses:   45,230.50 NGN
    ──────────────────────────────
    Net Balance:      80,269.50 NGN
  ```

---

### 4.9 Clear All Data

```bash
fintrack clear
```

**Behavior:**

- Displays confirmation prompt: `"Delete ALL data? This cannot be undone. (yes/no)"`
- If user confirms, deletes `tracker.json` and all backups
- Resets state to uninitialized
- If user cancels: `"Clear cancelled."`
- On success: `"✓ All data cleared. Run 'fintrack init' to start over."`

---

### 4.10 Dump JSON

```bash
fintrack dump
```

**Behavior:**

- Pretty-prints the entire `tracker.json` file to stdout
- Useful for inspection and debugging
- Output is formatted JSON with proper indentation
- No filtering or transformation

---

### 4.11 Describe (Post-MVP)

```bash
fintrack describe
```

**Behavior:**

- Provides basic exploratory data analysis (EDA)
- Shows:
  - Total records
  - Record count by category
  - Record count by subcategory
  - Total by category
  - Date range (earliest and latest record)
  - Average transaction amount
- Format:

  ```
  Financial Overview:
    Total Records: 45
    Date Range: 01-01-2025 to 30-12-2025

    By Category:
      Income:      15 records | 125,500.00 NGN
      Expenses:    30 records |  45,230.50 NGN

    By Subcategory (Top 5):
      Groceries:   12 records |  24,500.00 NGN
      Wages:        8 records |  80,000.00 NGN
      ...

    Average Transaction: 3,812.30 NGN
  ```

**Note:** Deferred to post-MVP to focus on core functionality.

---

### 4.12 Export (Post-MVP)

```bash
fintrack export --path <FOLDER_PATH> --type <FILE_TYPE>
```

**Behavior:**

- Exports tracker data to a file in `<FOLDER_PATH>`
- Supported types: `csv` (initially), `json` (future)
- For CSV: columns are ID, Category, Subcategory, Description, Amount, Currency, Date
- Filename format: `fintrack_export_2025-12-30T14-45-30Z.csv`
- On success: `"✓ Data exported to: /path/to/fintrack_export_2025-12-30T14-45-30Z.csv"`
- Validates folder exists and is writable

**Note:** Deferred to post-MVP.

---

### 4.13 Help

```bash
fintrack help
fintrack --help
fintrack -h
```

**Behavior:**

- Displays comprehensive help text with all commands, flags, and examples

---

## 5. File System Layout

```
~/.fintrack/
├── tracker.json                              # Primary data file
├── config                                    # (Future) Config file
└── backups/
    ├── tracker.backup.2025-12-30T10-30-00Z.json
    ├── tracker.backup.2025-12-30T12-15-45Z.json
    └── tracker.backup.2025-12-30T14-45-30Z.json
```

**Backup Policy:**

- One backup is maintained at a time
- Before any mutation (add/update/delete), the current `tracker.json` is copied to `backups/` with a timestamped filename
- After successful command completion, the old backup is deleted
- If corruption is detected on startup, the latest backup is restored and user is notified
- User can manually inspect backups in the `backups/` directory

---

## 6. Error Handling and Recovery

### 6.2 Error Types and Suggestions

Each error type maps to a specific suggestion, generated by a `format_error()` function:

```rust
fn format_error(error: &ProcessError) -> String {
    match error {
        ProcessError::ValidationError(kind) => {
            match kind {
                ValidationErrorKind::AmountTooSmall { amount } => {
                    format!(
                        "✗ ValidationError: Amount must be greater than 0 (got {})\n\
                         Suggestion: Re-run the command with a positive amount (e.g., --amount 500)",
                        amount
                    )
                }

                ValidationErrorKind::InvalidDate { provided, expected_format } => {
                    format!(
                        "✗ ValidationError: Invalid date format '{}'\n\
                         Suggestion: Use format '{}' (e.g., 30-12-2025)",
                        provided, expected_format
                    )
                }
                ValidationErrorKind::SubcategoryNotFound { name } => {
                    format!(
                        "✗ ValidationError: Subcategory '{}' does not exist\n\
                         Suggestion: View all subcategories with 'fintrack subcategory list'",
                        name
                    )
                }
                ValidationErrorKind::SubcategoryAlreadyExists { name } => {
                    format!(
                        "✗ ValidationError: Subcategory '{}' already exists\n\
                         Suggestion: Use a different name, or view all subcategories with 'fintrack subcategory list'",
                        name
                    )
                }
                ValidationErrorKind::RecordNotFound { id } => {
                    format!(
                        "✗ ValidationError: Record with ID {} does not exist\n\
                         Suggestion: Use 'fintrack list' to view all records",
                        id
                    )
                }
                ValidationErrorKind::SubcategoryHasRecords { name, count } => {
                    format!(
                        "✗ ValidationError: Cannot delete '{}' — it has {} records\n\
                         Suggestion: Delete those records first, or use 'fintrack delete -S {}' to delete the subcategory and all its records",
                        name, count, name
                    )
                }
                ValidationErrorKind::CannotDeleteMiscellaneous => {
                    "✗ ValidationError: Cannot delete 'Miscellaneous' — it is a system subcategory\n\
                     Suggestion: Choose a different subcategory to delete".to_string()
                }
                ValidationErrorKind::CategoryImmutable { category } => {
                    format!(
                        "✗ ValidationError: Category '{}' is immutable and cannot be modified\n\
                         Suggestion: Use 'fintrack category list' to view available categories",
                        category
                    )
                }
                ValidationErrorKind::InvalidCategoryName { name, reason } => {
                    format!(
                        "✗ ValidationError: Invalid category name '{}' — {}\n\
                         Suggestion: Use only letters and numbers, starting with a letter",
                        name, reason
                    )
                }
                ValidationErrorKind::InvalidName { name, reason } => {
                    format!(
                        "✗ ValidationError: Invalid name '{}' — {}\n\
                         Suggestion: Names must be alphanumeric and start with a letter",
                        name, reason
                    )
                }
                ValidationErrorKind::InvalidAmount { reason } => {
                    format!(
                        "✗ ValidationError: Invalid amount — {}\n\
                         Suggestion: Enter a positive number (e.g., 500 or 150.50)",
                        reason
                    )
                }
            }
        }
        ProcessError::FileNotFound(msg) => {
            format!(
                "✗ FileNotFound: {}\n\
                 Suggestion: Run 'fintrack init' to create a tracker",
                msg
            )
        }
        ProcessError::InvalidJson(msg) => {
            format!(
                "✗ InvalidJson: {}\n\
                 Suggestion: Your tracker.json may be corrupted. Run 'fintrack dump' to inspect, or 'fintrack clear' to reset",
                msg
            )
        }
        ProcessError::PermissionDenied(msg) => {
            format!(
                "✗ PermissionDenied: {}\n\
                 Suggestion: Check file permissions in ~/.fintrack/",
                msg
            )
        }
        ProcessError::CorruptedData { backup_restored, timestamp } => {
            if *backup_restored {
                format!(
                    "⚠ CorruptedData: Your data was corrupted and has been recovered\n\
                     Details: Restored from backup created at {}\n\
                     Suggestion: Verify your recent changes. Use 'fintrack dump' for inspection",
                    timestamp
                )
            } else {
                format!(
                    "✗ CorruptedData: Both main file and backup are corrupted\n\
                     Details: Unable to recover automatically\n\
                     Suggestion: Run 'fintrack dump' to inspect remaining data, or 'fintrack clear' to reset and start over",
                )
            }
        }
        ProcessError::Other(msg) => {
            format!("✗ Error: {}", msg)
        }
    }
}
```

### 6.1 Validation

All input is validated before mutation:

1. **File existence:** Check if `tracker.json` exists and is readable
2. **JSON structure:** Validate JSON is well-formed and matches expected schema
3. **Referential integrity:** Ensure all category/subcategory IDs in records exist in the maps
4. **Data consistency:** Check for duplicate record IDs, invalid amounts, malformed dates
5. **Business logic:** Validate category/subcategory names, amounts > 0, dates are real

### 6.3 Corruption Detection and Recovery

**On startup (before any operation):**

```rust
fn load_tracker(tracker_path: &Path, backup_path: &Path) -> Result<TrackerData, ProcessError> {
    // 1. Try to load tracker.json
    match std::fs::read_to_string(tracker_path) {
        Ok(content) => {
            // 2. Try to deserialize and validate
            match serde_json::from_str(&content) {
                Ok(data) => validate(&data),
                Err(_) => {
                    // 3. If corrupt, try backup
                    restore_from_backup(backup_path)
                }
            }
        }
        Err(_) => {
            // File doesn't exist, try backup
            restore_from_backup(backup_path)
        }
    }
}
```

**On corruption:**

- Latest backup is restored to `tracker.json`
- User is notified: `"⚠ Your data was corrupted and restored from backup created at 2025-12-30T14:45:30Z. Please verify your recent changes."`
- Both files are now in sync; old backup is deleted after first successful command
- If both main and backup are corrupted: `"✗ Fatal: Both tracker.json and backup are corrupted. Unable to recover. Run 'fintrack dump' to inspect remaining data, or 'fintrack clear' to reset."`

### 6.3 Atomic Mutations with Backup

**Every mutation follows this pattern:**

1. Load and validate current `tracker.json`
2. Create timestamped backup in `~/.fintrack/backups/`
3. Apply mutation to in-memory copy
4. Serialize and write to temporary file (e.g., `tracker.json.tmp`)
5. Atomic move (rename) `tracker.json.tmp` → `tracker.json`
6. On success, delete old backup
7. On failure, restore from backup and surface error to user

This ensures data is never in a partially-written state.

---

## 7. Input Handling and Validation

### 7.1 Naming Conventions

**Category/Subcategory Names (CLI input):**

- Case-insensitive (user can type "wages", "Wages", or "WAGES")
- Internally stored in Title Case (e.g., "Wages")
- Alphanumeric only; must start with a letter
- Validated on input

**Dates:**

- Format: DD-MM-YYYY (e.g., "30-12-2025")
- Validated against actual calendar (leap years, valid month/day combinations)
- Stored internally as DD-MM-YYYY strings for consistency

**Amounts:**

- Positive floats only (>0)
- No currency symbol in input (e.g., enter `4000`, not `4000 NGN`)
- Stored with 2 decimal places for display

**Descriptions:**

- Optional; can be empty string
- Supports escape sequences like `\n` (newline), `\t` (tab)

### 7.2 Clap Configuration

- Use Clap v4+ for parsing
- Define subcommands for each operation (`init`, `add`, `delete`, etc.)
- Enforce required flags within subcommands
- Support flexible flag ordering
- Provide default values where applicable
- Generate helpful error messages for invalid input

---

## 8. Testing Strategy

### 8.1 Unit Tests

- **Validation functions:** Test date parsing, amount validation, name validation
- **Business logic:** Test record creation, deletion, filtering (without file I/O)
- **Data transformation:** Test record conversion, sorting, aggregation
- **Error cases:** Test invalid input, missing fields, duplicate names

### 8.2 Integration Tests

- **File I/O:** Create test tracker files, perform operations, verify state
- **Backup/recovery:** Corrupt a test file, verify recovery logic
- **Atomic mutations:** Verify that failed operations don't partially write state
- **End-to-end:** Full command flows (init → add → list → delete → total)

### 8.3 Test Structure

```rust
#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_tracker() -> (TempDir, PathBuf) {
        // Create temporary directory
        // Initialize tracker
        // Return temp dir and tracker path
    }

    #[test]
    fn test_add_record_valid() { }

    #[test]
    fn test_add_record_invalid_amount() { }

    #[test]
    fn test_backup_created_before_mutation() { }

    #[test]
    fn test_corruption_recovery() { }
    // ... many more
}
```

**Test Utilities:**

- Helper functions to create test trackers quickly
- Mock/fake implementations if needed for isolation
- Use `tempfile` crate for temporary directories in tests

---

## 9. CLI Features

### 9.1 Autocompletion

**Future feature (post-MVP):** Generate shell completions for bash, zsh, fish.

- Auto-complete command names
- Auto-complete category/subcategory names
- Auto-complete flags

Using `clap_complete` crate or similar.

### 9.2 Output Formatting

**Colors and Styling:**

- Use `colored` or `termcolor` for terminal output
- Success messages in green (`✓`)
- Errors in red (`✗`)
- Warnings in yellow (`⚠`)
- Deleted records in strikethrough or red

**Tables:**

- Use `prettytable-rs` or `tabled` for tabular output
- Right-align numbers for readability
- Clear column headers

**Example Table:**

```
┌────┬──────────┬──────────────┬─────────┬────────────────┬──────────────┐
│ ID │ Category │ Subcategory  │ Amount  │ Date           │ Description  │
├────┼──────────┼──────────────┼─────────┼────────────────┼──────────────┤
│ 1  │ Income   │ Wages        │ 4000.00 │ 30-12-2025     │ Monthly      │
│ 2  │ Expenses │ Groceries    │  150.50 │ 28-12-2025     │ Weekly shop  │
└────┴──────────┴──────────────┴─────────┴────────────────┴──────────────┘
```

---

## 10. Error Messages

All error messages follow a consistent pattern:

```
✗ <ErrorType>: <Specific detail>

Suggestion: <Action user can take>
```

**Examples:**

```
✗ ValidationError: Amount must be greater than 0
Suggestion: Re-run the command with a positive amount (e.g., --amount 500)

✗ NotFound: Subcategory 'Groceries' does not exist
Suggestion: View all subcategories with 'fintrack subcategory list'

✗ CorruptedData: Your data was corrupted and has been recovered
Details: Restored from backup created at 2025-12-30T14:45:30Z
Suggestion: Verify your recent changes. Manual inspection available with 'fintrack dump'
```

---

## 11. Implementation Phases

### Phase 1 (MVP)

- `fintrack init`
- `fintrack add`
- `fintrack delete` (by ID, by category, by subcategory)
- `fintrack update`
- `fintrack list` (all variants)
- `fintrack category list`
- `fintrack subcategory` (list, add, delete, update/rename)
- `fintrack clear`
- `fintrack total`
- `fintrack dump`
- Full backup/recovery system
- Comprehensive validation
- Table-based output formatting
- Help/documentation

### Phase 2 (Post-MVP)

- `fintrack describe` (EDA)
- `fintrack export` (CSV, later JSON)
- Autocompletion (bash, zsh, fish)
- Configuration file (`~/.fintrack/config`)
- Performance optimizations
- Extended currency support

### Phase 3 (Future)

- History/undo/redo (requires history directory)
- Budget tracking and alerts
- Multi-tracker support
- Encrypted backups
- Cloud sync (optional, user-controlled)

---

## 12. Technical Decisions

### 12.1 Two-HashMap Approach for Subcategories

**Why:** Enables O(1) lookups in both directions (name → ID and ID → name).

```rust
subcategories_by_id: HashMap<u32, String>,
subcategories_by_name: HashMap<String, u32>,
```

**Trade-off:** Slight memory overhead and must keep in sync. Worth it for fast operations.

### 12.2 Date Format: DD-MM-YYYY

**Why:** Human-readable, unambiguous in many locales, easy to parse/validate.

**Note:** Dates are stored as strings, not Unix timestamps, for readability in the JSON file and user-friendliness.

### 12.3 Amounts as Floats

**Why:** Simplicity for MVP; acceptable precision for financial tracking at personal scale.

**Future:** Consider `Decimal` crate for exact decimal arithmetic if needed post-MVP.

### 12.4 Process-Based Error Handling

**Why:** Allows clean separation of concerns, easy testing, and consistent error reporting across all commands.

### 12.5 Single Backup Policy

**Why:** Simplicity for MVP. Sufficient for recovery from corruption or accidental deletion. History/undo deferred to future.

---

## 13. Dependencies

**Core:**

- `serde` and `serde_json` – Serialization
- `clap` v4+ – CLI argument parsing
- `chrono` – Date/time handling (validation, formatting)
- `tempfile` – Testing

**UI/Output:**

- `colored` or `termcolor` – Terminal colors
- `tabled` or `prettytable-rs` – Table formatting

**Optional (post-MVP):**

- `clap_complete` – Autocompletion generation
- `rust_decimal` – Precise decimal arithmetic

---

## 14. Future Considerations

### 14.1 Config File (`~/.fintrack/config`)

```toml
[default]
currency = "NGN"
date_format = "DD-MM-YYYY"
default_subcategory = "Miscellaneous"

[display]
colorize_output = true
table_format = "fancy"

[backup]
auto_backup = true
max_backups = 1
```

### 14.2 History and Undo/Redo

Requires separate `~/.fintrack/history/` directory to store state snapshots. Deferred to Phase 3.

### 14.3 Multi-Tracker Support

Allow users to manage multiple independent trackers (e.g., personal and business). Deferred to Phase 3.

---

## 15. Success Criteria

A world-class financial tracker is:

- **Reliable:** Data is never lost; corruption is detected and recovered
- **Fast:** Instant operations even with thousands of records
- **Simple:** Easy to learn and use; no unnecessary complexity
- **Safe:** Atomic mutations; no partial writes; backups always available
- **Transparent:** User owns all data; no remote dependencies
- **Well-tested:** High coverage; edge cases handled
- **User-friendly:** Clear error messages; helpful feedback
- **Documented:** Comprehensive help and examples

This design achieves all of these for the MVP phase.

---

**End of Document**
