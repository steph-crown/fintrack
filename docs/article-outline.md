# Article Outline: How to Build a Local-First CLI Financial Tracker with Rust

## Proposed Title

**"How to Build a Local-First CLI Financial Tracker with Rust"**

**Review:** ✅ This title follows freeCodeCamp's recommended "How to build..." format. It's clear, descriptive, and includes key search terms (CLI, Financial Tracker, Rust). Consider adding "Step-by-Step" if you want to emphasize the tutorial nature, but the current title is strong.

---

## Article Structure

### Introduction

**What this section covers:**

- Hook: Why build a financial tracker? The value of local-first applications
- What readers will build: A complete CLI tool for tracking income and expenses
- What they'll learn: Rust CLI development, file I/O, JSON serialization, error handling, argument parsing
- Brief overview of the final product's capabilities
- Set expectations: We'll build core features, mention advanced features at the end

**Estimated length:** 2-3 paragraphs

---

### Prerequisites

**What this section covers:**

- Rust installed (version 1.70+) (include link to install instructions https://rust-book.cs.brown.edu/ch01-01-installation.html, and command to confirm they have it installed)
- Basic understanding of Rust (structs, enums, error handling, traits)
- Familiarity with command-line tools
- Basic knowledge of JSON
- Optional: Experience with `clap` or similar CLI libraries (we'll explain as we go. emphasize that it's not compulsory they have used it before since we'll explain in the article)

**Estimated length:** 1 paragraph with bullet points

---

### Commands We'll Build

**What this section covers:**

- Commands we'll implement step-by-step: `init`, `add`, `list`, `update`, `delete`, `subcategory` (with subcommands: `list`, `add`, `delete`, `rename`), `total`
- Why we focus on these commands (core functionality, learning fundamentals)
- Commands we won't cover but exist in the full implementation: `describe` (data analysis), `export` (CSV/JSON export), `dump` (raw JSON display), `clear` (reset tracker), `category list` (view categories)
- Why we leave these out (advanced features, can be implemented as exercises)
- Note: All code will be actual, working code that matches the full implementation

**Estimated length:** 1 paragraph

---

### Step 1: Set Up the Project

**What this section covers:**

- Creating a new Rust project with `cargo new fintrack`
- Adding dependencies to `Cargo.toml`: `clap`, `serde`, `serde_json`, `chrono`, `dirs`
- Brief explanation of what each dependency does and why we need it
- Project structure overview
- Why we organize code into modules

**Estimated length:** 1-2 paragraphs + code block

---

### Step 2: Design the Data Model

**What this section covers:**

- High-level architecture overview (from design.md 2.1): CLI Layer → Process Layer → Business Logic Layer → Persistent Storage Layer
- Why we chose a layered architecture (separation of concerns, testability, maintainability)
- The `TrackerData` struct: currency, opening balance, categories, subcategories, records
- The `Record` struct: id, category, subcategory, amount, description, date
- Why we use IDs instead of strings (efficiency, referential integrity, easier updates)
- The two-HashMap approach for subcategories (bidirectional lookup: name→ID and ID→name)
- Why we need bidirectional lookup (validation and display)
- JSON structure and why it's human-readable (local-first principle, easy debugging)
- Why we store currency as a string (flexibility, extensibility)

**Estimated length:** 3-4 paragraphs + code examples + architecture diagram

---

### Step 3: Set Up the CLI Structure

**What this section covers:**

- Using `clap` for argument parsing (why clap: powerful, ergonomic, well-maintained)
- Command structure: subcommands for each operation
- The `cli()` and `exec()` pattern: why we separate command definition from execution
  - `cli()` function: defines arguments, help text, validation rules (returns `Command`)
  - `exec()` function: contains the actual logic, takes `GlobalContext` and `ArgMatches` (returns `CliResult`)
  - Why this separation: testability, reusability, clear separation of concerns
- Global context: managing file paths (`~/.fintrack/tracker.json`)
- Why we use a `GlobalContext` struct (centralized path management, easier to test)
- Error types: `CliError` enum with variants
- Response types: `CliResponse` and `ResponseContent` enum
- Why we use enums for responses (type safety, exhaustive matching)

**Estimated length:** 3-4 paragraphs + code examples

---

### Step 4: Implement File Operations

**What this section covers:**

- Creating the `FilePath` trait for reusable file operations
- Why we use a trait (method syntax, works with `Path` and `PathBuf`, extensibility)
- Methods: `create_file_if_not_exists()`, `open_read_write()`, `read_file()`
- Blanket implementation using `AsRef<Path>` (why: works with any path-like type)
- Writing JSON to files safely (atomic writes, error handling)
- Error handling for file operations (why we propagate errors with `?`)

**Estimated length:** 2-3 paragraphs + code examples

---

### Step 5: Build the Init Command

**What this section covers:**

- The `cli()` function: defining arguments (currency, opening balance)
- The `exec()` function: implementation logic
- Creating the tracker file and directory structure
- Why we create `~/.fintrack/backups/` directory upfront (future-proofing)
- Setting up default categories (Income, Expenses) with IDs 1 and 2
- Why we use fixed IDs for categories (they're immutable, easier to reference)
- Creating the default "Miscellaneous" subcategory with ID 1
- Parsing currency and opening balance arguments
- Initializing the JSON structure with timestamps
- Why we track `created_at` and `last_modified` (data integrity, debugging)
- Error handling: what if tracker already exists? (prevent accidental overwrite)

**Estimated length:** 4-5 paragraphs + code examples (both `cli()` and `exec()`)

---

### Step 6: Add Records

**What this section covers:**

- The `cli()` function: defining arguments (category, amount, subcategory, description, date)
- The `exec()` function: implementation logic
- Parsing category (case-insensitive) and amount arguments
- Why we make category case-insensitive (better user experience)
- Validating amount > 0 (why: prevents invalid data, business logic requirement)
- Resolving subcategory (defaulting to "miscellaneous")
- Why we default to "miscellaneous" (always available, prevents errors)
- Generating unique record IDs (incrementing counter, why this approach)
- Setting default date to today if not provided (convenience, common use case)
- Adding the record to the tracker
- Updating `last_modified` timestamp
- Displaying the created record (basic formatting without external libraries)

**Estimated length:** 5-6 paragraphs + code examples (both `cli()` and `exec()`)

---

### Step 7: List Records

**What this section covers:**

- The `cli()` function: defining filter arguments (category, subcategory, date range, first/last)
- The `exec()` function: implementation logic
- Reading all records from the tracker
- Filtering by category (optional) - why we use `Option` types
- Filtering by subcategory (optional)
- Filtering by date range (start and end dates) - why we parse dates upfront
- Limiting results (first N or last N records) - why we use `ArgGroup` for mutual exclusivity
- Sorting by date (why: chronological order is most useful)
- Displaying records in a formatted table (basic formatting: column alignment, headers)
- Why we format output (readability, professional appearance)
- Handling empty results gracefully (user-friendly messages)

**Estimated length:** 5-6 paragraphs + code examples (both `cli()` and `exec()`)

---

### Step 8: Update Records

**What this section covers:**

- The `cli()` function: defining optional update arguments
- The `exec()` function: implementation logic
- Finding a record by ID (why ID lookup: fast, unambiguous)
- Partial updates: only updating provided fields (why: user convenience, efficiency)
- Validating each field (amount > 0, subcategory exists, etc.)
- Why we validate on update (data integrity, prevent invalid states)
- Handling optional arguments gracefully (using `Option` types)
- Updating `last_modified` timestamp
- Displaying the updated record

**Estimated length:** 4-5 paragraphs + code examples (both `cli()` and `exec()`)

---

### Step 9: Delete Records

**What this section covers:**

- The `cli()` function: defining deletion modes with `ArgGroup`
- The `exec()` function: implementation logic
- Three deletion modes: by ID(s), by category, by subcategory
- Why we support multiple deletion modes (flexibility, different use cases)
- Using `ArgGroup` to enforce one method (why: prevent ambiguity, clear API)
- Implementing each deletion method
- Using `Vec::retain()` for efficient filtering (why: in-place, efficient)
- Updating `last_modified` timestamp

**Estimated length:** 3-4 paragraphs + code examples (both `cli()` and `exec()`)

---

### Step 10: Manage Subcategories

**What this section covers:**

- The `cli()` function: defining subcategory subcommands (list, add, delete, rename)
- The `exec()` functions: implementation for each subcommand
- Listing subcategories: reading from the tracker and displaying
- Adding subcategories: validating name format (why: consistency, prevent errors)
- Checking duplicates (case-insensitive, why: prevent confusion)
- Generating IDs (incrementing counter, why: simple, efficient)
- Deleting subcategories: checking if records exist (why: referential integrity)
- Preventing deletion of "Miscellaneous" (why: system subcategory, always needed)
- Renaming subcategories: updating both HashMaps (why: maintain bidirectional lookup)
- Checking for conflicts (why: prevent duplicate names)
- Why records don't need updating (they reference by ID, not name)

**Estimated length:** 6-7 paragraphs + code examples (all subcommands)

---

### Step 11: Calculate Totals

**What this section covers:**

- The `cli()` function: no arguments needed
- The `exec()` function: implementation logic
- Reading all records
- Separating income and expenses (why: different calculations)
- Calculating totals for each category
- Computing net balance (opening + income - expenses)
- Why we show net balance (most important metric for users)
- Formatting currency display (basic formatting without external libraries)
- Displaying a summary with proper formatting

**Estimated length:** 3-4 paragraphs + code examples (both `cli()` and `exec()`)

---

### Step 12: Format Output

**What this section covers:**

- Why we separate output formatting (separation of concerns, testability)
- Basic formatting without external libraries (using standard library)
- Formatting records as tables (column alignment, headers)
- Formatting totals (clear labels, currency display)
- Formatting errors (clear messages, suggestions)
- Formatting success messages (consistent style)
- Why we keep formatting simple (no dependencies, works everywhere)
- Note: readers can enhance with libraries like `colored` and `tabled` if desired

**Estimated length:** 2-3 paragraphs + code examples

---

### Step 13: Handle Errors and Provide User Feedback

**What this section covers:**

- Custom error types (`CliError`, `ValidationErrorKind`)
- Why we use custom errors (better error messages, type safety)
- Meaningful error messages with suggestions (why: help users fix issues)
- Using `Result` types throughout (why: explicit error handling, no panics)
- The `?` operator for error propagation (why: concise, idiomatic)
- Formatting errors for terminal output (basic formatting)
- Success messages and formatted output

**Estimated length:** 3-4 paragraphs + code examples

---

### Step 14: Test Your Implementation

**What this section covers:**

- Manual testing workflow: init → add → list → update → delete
- Testing edge cases: invalid amounts, missing subcategories, etc.
- Verifying data persistence
- Testing filtering and sorting
- Common issues and how to debug them

**Estimated length:** 2-3 paragraphs

---

### What's Next and Advanced Features

**What this section covers:**

- What we built: summary of core features (init, add, list, update, delete, subcategories, totals)
- What we didn't cover: export to CSV/JSON, data analysis (`describe`), raw JSON dump (`dump`), clear command
- Why we focused on core features (learning fundamentals, building blocks)
- Challenge: Try implementing export functionality (CSV/JSON serialization)
- Challenge: Add data validation and recovery (corrupted JSON handling)
- Challenge: Implement backup functionality (automatic snapshots)
- Challenge: Add colored output using `colored` crate
- Challenge: Add tabular formatting using `tabled` crate
- Full implementation: Link to GitHub repo (with all features including export, describe, dump)
- Using the tool: Link to crates.io and binary releases
- Encouragement to extend and customize

**Estimated length:** 4-5 paragraphs

---

### Conclusion

**What this section covers:**

- Recap what readers built (a complete CLI financial tracker)
- Key Rust concepts they learned (traits, error handling, file I/O, JSON serialization, CLI development)
- The value of local-first applications (data ownership, privacy, no dependencies)
- Encouragement to continue building (extend features, customize, contribute)
- Final call-to-action: check out the full implementation on GitHub

**Estimated length:** 2-3 paragraphs

---

## Estimated Total Word Count

**Target:** 5,000-8,000 words (comprehensive tutorial range)

**Breakdown:**

- Introduction: ~200 words
- Prerequisites: ~100 words
- Commands We'll Build: ~100 words
- Step 1: Set Up the Project: ~200 words
- Step 2: Design the Data Model: ~500 words (includes architecture)
- Step 3: Set Up the CLI Structure: ~400 words (includes cli/exec pattern)
- Step 4: Implement File Operations: ~300 words
- Step 5: Build the Init Command: ~600 words
- Step 6: Add Records: ~700 words
- Step 7: List Records: ~800 words
- Step 8: Update Records: ~600 words
- Step 9: Delete Records: ~500 words
- Step 10: Manage Subcategories: ~900 words
- Step 11: Calculate Totals: ~400 words
- Step 12: Format Output: ~300 words
- Step 13: Handle Errors and Provide User Feedback: ~400 words
- Step 14: Test Your Implementation: ~300 words
- What's Next and Advanced Features: ~500 words
- Conclusion: ~200 words

**Total:** ~7,200 words (within target range)

---

## Code Examples to Include (All Actual Working Code)

**Important:** All code examples must be actual, copy-pasteable code from the implementation. Readers should be able to copy each snippet, follow the instructions, and have everything work exactly as described.

1. **Cargo.toml** - Complete dependencies section (actual working dependencies)
2. **Data structures** - Complete `TrackerData`, `Record`, `Category`, `Currency` enums (actual structs)
3. **High-level architecture** - ASCII diagram from design.md 2.1
4. **FilePath trait** - Complete trait definition with blanket implementation (actual code)
5. **Init command** - Both `cli()` and `exec()` functions (complete, working code)
6. **Add command** - Both `cli()` and `exec()` functions with validation (complete, working code)
7. **List command** - Both `cli()` and `exec()` functions with filtering logic (complete, working code)
8. **Update command** - Both `cli()` and `exec()` functions with partial updates (complete, working code)
9. **Delete command** - Both `cli()` and `exec()` functions with multiple deletion modes (complete, working code)
10. **Subcategory commands** - All four subcommands (`list`, `add`, `delete`, `rename`) with both `cli()` and `exec()` (complete, working code)
11. **Total command** - Both `cli()` and `exec()` functions (complete, working code)
12. **Error handling** - Custom error types (`CliError`, `ValidationErrorKind`) (actual enums)
13. **Output formatting** - Basic table formatting functions (actual formatting code, no external libraries)
14. **Main.rs** - Command dispatch logic (how commands are registered and executed)

---

## Notes for Writing

- Use active voice throughout ("You create..." not "Creating...")
- **NO GERUNDS in headings** - Use "Step X:" format for implementation sections, imperative mood or descriptive nouns for supporting sections, never "Doing X" or "Building X"
- Use "Step X:" format only for actual implementation/building steps (Steps 1-14), keep supporting sections (Introduction, Prerequisites, etc.) without step numbers
- Keep paragraphs short (1-2 sentences)
- Use code examples liberally - every code example must be actual, working code from the implementation
- Explain the "why" behind decisions, not just the "how" (why we use IDs, why we separate cli/exec, why we use traits, etc.)
- Include common pitfalls and how to avoid them
- Use subheadings (H3, H4) to break up long sections
- Add inline code formatting for function names, types, etc.
- Include terminal output examples where helpful
- Link to relevant Rust documentation when introducing concepts
- For each command, show both `cli()` and `exec()` functions
- Explain architectural decisions (layered architecture, trait usage, etc.)
- Mention that output formatting is basic (no external libraries) but can be enhanced

---

## Style Guide Compliance Checklist

- ✅ Title follows "How to build..." format
- ✅ **NO GERUNDS in headings** - All headings use imperative mood or descriptive nouns (no -ing forms)
- ✅ Active voice throughout ("You create..." not "Creating...")
- ✅ Short sentences and paragraphs (1-2 sentences per paragraph)
- ✅ Clear subheadings structure (H2 for main sections, H3/H4 for subsections)
- ✅ Prerequisites section included
- ✅ Step-by-step walkthrough
- ✅ Code examples with syntax highlighting (all actual, working code)
- ✅ Conclusion that recaps learning
- ✅ No excessive formatting (bold/italics used sparingly)
- ✅ Proper capitalization (Rust, JSON, CLI, etc.)
- ✅ No abbreviations (spell out "for example" not "e.g.")
- ✅ Use "you" instead of "we" when possible
- ✅ Explain "why" behind decisions, not just "how"
- ✅ All code examples are copy-pasteable and work exactly as shown
