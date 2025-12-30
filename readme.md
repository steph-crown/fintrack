# FinTrack

> _described below is what i plan for this tool to be able to do. i'll update with what's done as i progress_

A local-first CLI financial tracker written in Rust. Track your income and expenses on your own machine, with zero cloud dependencies and complete data ownership.

## Why FinTrack?

- **Your data stays yours.** Everything is stored locally in `~/.fintrack/`. No remote servers, no accounts, no privacy concerns.
- **Simple and fast.** Lightweight CLI tool that gets out of your way.
- **Reliable.** Automatic backups and corruption recovery ensure your data is never lost.
- **Transparent.** Open-source and easy to inspect. All your financial data is in human-readable JSON.

## Installation

### From Source (Recommended)

> _eventually, i want this to be a distributed binary that won't require you to install Rust_

Requires Rust 1.70+. [Install Rust here](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/yourusername/fintrack.git
cd fintrack
cargo install --path .
```

Then verify installation:

```bash
fintrack --version
```

## Quick Start

### 1. Initialize Your Tracker

```bash
fintrack init --currency NGN
```

This creates `~/.fintrack/tracker.json` and sets your currency. Supported currencies: NGN, USD, GBP, EUR, CAD, AUD, JPY, INR.

### 2. Add Your First Record

```bash
fintrack add -c Income -a 4000 -s Wages
```

Or using long flags:

```bash
fintrack add --category Income --amount 4000 --subcategory Wages
```

Optionally add a description and specific date:

```bash
fintrack add -c Expenses -a 150.50 -s Groceries -d "Weekly shop" --date 28-12-2025
```

**Available flags:**

- `-c, --category` (required) – Income or Expenses
- `-a, --amount` (required) – Positive number
- `-s, --subcategory` (optional) – Defaults to Miscellaneous
- `-d, --description` (optional) – Any text
- `--date` (optional) – Format: DD-MM-YYYY, defaults to today

### 3. View Your Data

```bash
fintrack list
```

See totals:

```bash
fintrack total
```

Filter by date range:

```bash
fintrack list --start 01-12-2025 --end 31-12-2025
```

Filter by category or subcategory:

```bash
fintrack list -c Income
fintrack list -s Groceries
```

View last 10 records:

```bash
fintrack list -l 10
```

### 4. Manage Subcategories

View all subcategories:

```bash
fintrack subcategory list
```

Add a new subcategory:

```bash
fintrack subcategory add Utilities
```

Delete a subcategory (only if it has no records):

```bash
fintrack subcategory delete Utilities
```

Rename a subcategory:

```bash
fintrack subcategory update -o Groceries -n Food
```

### 5. Update or Delete Records

Update a record by ID:

```bash
fintrack update 5 -a 200 -d "Revised amount"
```

Delete a record by ID:

```bash
fintrack delete 5
```

Delete all records in a category or subcategory:

```bash
fintrack delete -C Expenses
fintrack delete -S Groceries
```

(Both require confirmation.)

## Common Commands

| Task                          | Command                                                            |
| ----------------------------- | ------------------------------------------------------------------ |
| Initialize tracker            | `fintrack init --currency NGN`                                     |
| Add record (short)            | `fintrack add -c Income -a 4000 -s Wages`                          |
| Add record (long)             | `fintrack add --category Income --amount 4000 --subcategory Wages` |
| Update record (short)         | `fintrack update 5 -a 200 -d "Updated"`                            |
| Update record (long)          | `fintrack update 5 --amount 200 --description "Updated"`           |
| List all records              | `fintrack list`                                                    |
| List last 10 (short)          | `fintrack list -l 10`                                              |
| List last 10 (long)           | `fintrack list --last 10`                                          |
| Filter by category (short)    | `fintrack list -c Income`                                          |
| Filter by category (long)     | `fintrack list --category Income`                                  |
| Filter by date                | `fintrack list --start 01-12-2025 --end 31-12-2025`                |
| View totals                   | `fintrack total`                                                   |
| Delete record by ID           | `fintrack delete 5`                                                |
| Delete by category (short)    | `fintrack delete -C Expenses`                                      |
| Delete by category (long)     | `fintrack delete --by-cat Expenses`                                |
| Delete by subcategory (short) | `fintrack delete -S Groceries`                                     |
| View categories               | `fintrack category list`                                           |
| View subcategories            | `fintrack subcategory list`                                        |
| Add subcategory               | `fintrack subcategory add Shopping`                                |
| Rename subcategory (short)    | `fintrack subcategory update -o Old -n New`                        |
| Rename subcategory (long)     | `fintrack subcategory update --old Old --new New`                  |
| Export data (future)          | `fintrack export --path ~/Downloads --type csv`                    |
| View raw JSON                 | `fintrack dump`                                                    |
| Clear all data                | `fintrack clear`                                                   |
| Get help                      | `fintrack help`                                                    |

## Data Formats

**Dates:** DD-MM-YYYY (e.g., `30-12-2025`)

**Amounts:** Positive numbers only (e.g., `4000` or `150.50`)

**Names:** Alphanumeric, start with a letter (e.g., "Groceries", "Utilities_Bill")

## Data Storage

All your data is stored locally:

```
~/.fintrack/
├── tracker.json           # Your financial data
├── config                 # (Future) Configuration
└── backups/
    └── tracker.backup.*.json  # Automatic backups for recovery
```

You can safely back up the entire `~/.fintrack/` directory to protect your data.

## Automatic Backups & Recovery

FinTrack automatically creates timestamped backups before any changes. If your data becomes corrupted, FinTrack will automatically restore from the latest backup and notify you.

You can view your current data anytime:

```bash
fintrack dump
```

This pretty-prints your `tracker.json` to the terminal.

## Examples

### Track Monthly Income and Expenses

```bash
# Add monthly salary
fintrack add --category Income --amount 50000 --subcategory Wages --date 01-12-2025

# Add rent
fintrack add --category Expenses --amount 20000 --subcategory Housing --date 01-12-2025

# Add groceries
fintrack add --category Expenses --amount 5000 --subcategory Groceries --date 10-12-2025
fintrack add --category Expenses --amount 4500 --subcategory Groceries --date 20-12-2025

# View summary
fintrack total

# See expenses by category
fintrack list --category Expenses
```

### Review Last Week's Spending

```bash
fintrack list --last 7
```

### See Income for the Year

```bash
fintrack list --category Income --start 01-01-2025 --end 31-12-2025
```

### Correct a Mistake

```bash
fintrack list --last 5        # Find the wrong record
fintrack update 42 --amount 300  # Correct it
```

## Keyboard Shortcuts & Tips

- Use `fintrack help` to see all available commands
- Flag order doesn't matter: `--category Income --amount 4000` is the same as `--amount 4000 --category Income`
- Category and subcategory names are case-insensitive (use "wages", "Wages", or "WAGES"—all work)
- Dates default to today if not specified
- Descriptions are optional but helpful for future reference

## Troubleshooting

### "Tracker already initialized"

You've already run `fintrack init` once. If you want to start fresh, run:

```bash
fintrack clear
```

Then `fintrack init` again.

### "Subcategory does not exist"

View all available subcategories:

```bash
fintrack subcategory list
```

Then use the exact name from the list.

### "Cannot delete subcategory—it has X records"

You must delete all records in that subcategory first, or delete the subcategory and all its records at once:

```bash
fintrack delete --by-subcat Groceries
```

### Data seems corrupted or missing

FinTrack automatically detected corruption and restored from the latest backup. Run:

```bash
fintrack dump
```

to inspect your data. If something is still wrong, contact support or check GitHub issues.

## Future Features

Coming soon:

- **Describe command:** Exploratory data analysis (EDA) of your spending
- **CSV Export:** Export your data to CSV for use in Excel or other tools
- **Shell Autocompletion:** Tab-complete commands and category names
- **Configuration file:** Customize defaults and display preferences

## Contributing

Found a bug? Want a feature? Open an issue or pull request on [GitHub](https://github.com/yourusername/fintrack).

## License

MIT License. See LICENSE file for details.

---

## Want to Know More?

Interested in the technical design and architecture decisions behind FinTrack? Check out the **[Design Document](./docs/design.md)** for a comprehensive deep-dive into how I plan to build tool, including data structures, error handling, backup strategies, and the reasoning behind each decision.

---

**Get started now:** `fintrack init --currency NGN`
