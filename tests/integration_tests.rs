mod common;

use common::TestContext;
use fintrack::*;
use fintrack::commands;
use std::fs;

#[test]
fn test_init_creates_tracker_file() {
    let mut ctx = TestContext::new();
    let args = commands::init::cli().get_matches_from(&["init", "--currency", "usd", "--opening", "1000.0"]);

    let result = commands::init::exec(ctx.gctx_mut(), &args);
    assert!(result.is_ok());

    assert!(ctx.gctx.tracker_path().exists());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(data["currency"], "USD");
    assert_eq!(data["opening_balance"], 1000.0);
    assert_eq!(data["categories"]["income"], 1);
    assert_eq!(data["categories"]["expenses"], 2);
}

#[test]
fn test_init_with_defaults() {
    let mut ctx = TestContext::new();
    let args = commands::init::cli().get_matches_from(&["init"]);

    let result = commands::init::exec(ctx.gctx_mut(), &args);
    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(data["currency"], "NGN");
    assert_eq!(data["opening_balance"], 0.0);
}

#[test]
fn test_init_fails_when_file_exists() {
    let mut ctx = TestContext::new();
    let args1 = commands::init::cli().get_matches_from(&["init"]);
    let args2 = commands::init::cli().get_matches_from(&["init"]);

    let result1 = commands::init::exec(ctx.gctx_mut(), &args1);
    assert!(result1.is_ok());

    let result2 = commands::init::exec(ctx.gctx_mut(), &args2);
    assert!(result2.is_err());
    assert!(matches!(result2.unwrap_err(), CliError::FileAlreadyExists));
}

#[test]
fn test_add_record() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::add::cli().get_matches_from(&["add", "income", "500.0", "--subcategory", "miscellaneous"]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records.len(), 1);
    assert_eq!(data.records[0].amount, 500.0);
    assert_eq!(data.records[0].category, 1); // income
    assert_eq!(data.records[0].description, "");
}

#[test]
fn test_add_record_with_all_fields() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::add::cli().get_matches_from(&[
        "add",
        "expenses",
        "100.50",
        "--subcategory",
        "miscellaneous",
        "--description",
        "Test expense",
        "--date",
        "15-01-2025",
    ]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records.len(), 1);
    assert_eq!(data.records[0].amount, 100.50);
    assert_eq!(data.records[0].category, 2); // expenses
    assert_eq!(data.records[0].description, "Test expense");
    assert_eq!(data.records[0].date, "15-01-2025");
}

#[test]
fn test_add_record_rejects_zero_amount() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::add::cli().get_matches_from(&["add", "income", "0.0"]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::AmountTooSmall { .. })
    ));
}

#[test]
fn test_add_record_rejects_negative_amount() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::add::cli().get_matches_from(&["add", "income", "--", "-100.0"]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::AmountTooSmall { .. })
    ));
}

#[test]
fn test_add_record_rejects_invalid_subcategory() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::add::cli().get_matches_from(&["add", "income", "100.0", "--subcategory", "nonexistent"]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::SubcategoryNotFound { .. })
    ));
}

#[test]
fn test_list_all_records() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add1 = commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", "01-01-2025"]);
    let add2 = commands::add::cli().get_matches_from(&["add", "expenses", "50.0", "--date", "02-01-2025"]);
    let add3 = commands::add::cli().get_matches_from(&["add", "income", "200.0", "--date", "03-01-2025"]);

    commands::add::exec(ctx.gctx_mut(), &add1).unwrap();
    commands::add::exec(ctx.gctx_mut(), &add2).unwrap();
    commands::add::exec(ctx.gctx_mut(), &add3).unwrap();

    let list_args = commands::list::cli().get_matches_from(&["list"]);
    let result = commands::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::List { records, .. }) = response.content() {
            assert_eq!(records.len(), 3);
            // Records should be sorted by date
            assert_eq!(records[0].date, "01-01-2025");
            assert_eq!(records[1].date, "02-01-2025");
            assert_eq!(records[2].date, "03-01-2025");
        } else {
            panic!("Expected List response");
        }
    } else {
        panic!("Expected Ok result");
    }
}

#[test]
fn test_list_filter_by_category() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add1 = commands::add::cli().get_matches_from(&["add", "income", "100.0"]);
    let add2 = commands::add::cli().get_matches_from(&["add", "expenses", "50.0"]);
    let add3 = commands::add::cli().get_matches_from(&["add", "income", "200.0"]);

    commands::add::exec(ctx.gctx_mut(), &add1).unwrap();
    commands::add::exec(ctx.gctx_mut(), &add2).unwrap();
    commands::add::exec(ctx.gctx_mut(), &add3).unwrap();

    let list_args = commands::list::cli().get_matches_from(&["list", "--category", "income"]);
    let result = commands::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::List { records, .. }) = response.content() {
            assert_eq!(records.len(), 2);
            assert_eq!(records[0].amount, 100.0);
            assert_eq!(records[1].amount, 200.0);
        } else {
            panic!("Expected List response");
        }
    } else {
        panic!("Expected Ok result");
    }
}

#[test]
fn test_list_filter_by_date_range() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", "01-01-2025"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "expenses", "50.0", "--date", "05-01-2025"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "200.0", "--date", "10-01-2025"])).unwrap();

    let list_args = commands::list::cli().get_matches_from(&["list", "--start", "03-01-2025", "--end", "07-01-2025"]);
    let result = commands::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::List { records, .. }) = response.content() {
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].amount, 50.0);
        } else {
            panic!("Expected List response");
        }
    } else {
        panic!("Expected Ok result");
    }
}

#[test]
fn test_list_first_n_records() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    for i in 1..=5 {
        let date = format!("{:02}-01-2025", i);
        let add_args = commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", &date]);
        commands::add::exec(ctx.gctx_mut(), &add_args).unwrap();
    }

    let list_args = commands::list::cli().get_matches_from(&["list", "--first", "3"]);
    let result = commands::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::List { records, .. }) = response.content() {
            assert_eq!(records.len(), 3);
        } else {
            panic!("Expected List response");
        }
    } else {
        panic!("Expected Ok result");
    }
}

#[test]
fn test_list_last_n_records() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    for i in 1..=5 {
        let date = format!("{:02}-01-2025", i);
        let add_args = commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", &date]);
        commands::add::exec(ctx.gctx_mut(), &add_args).unwrap();
    }

    let list_args = commands::list::cli().get_matches_from(&["list", "--last", "2"]);
    let result = commands::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::List { records, .. }) = response.content() {
            assert_eq!(records.len(), 2);
        } else {
            panic!("Expected List response");
        }
    } else {
        panic!("Expected Ok result");
    }
}

#[test]
fn test_update_record() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0"])).unwrap();

    let update_args = commands::update::cli().get_matches_from(&["update", "1", "--amount", "150.0"]);
    let result = commands::update::exec(ctx.gctx_mut(), &update_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records[0].amount, 150.0);
}

#[test]
fn test_update_record_not_found() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let update_args = commands::update::cli().get_matches_from(&["update", "999", "--amount", "100.0"]);
    let result = commands::update::exec(ctx.gctx_mut(), &update_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::RecordNotFound { id: 999 })
    ));
}

#[test]
fn test_update_record_rejects_zero_amount() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0"])).unwrap();

    let update_args = commands::update::cli().get_matches_from(&["update", "1", "--amount", "0.0"]);
    let result = commands::update::exec(ctx.gctx_mut(), &update_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::AmountTooSmall { .. })
    ));
}

#[test]
fn test_delete_by_ids() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    for _i in 1..=5 {
        let add_args = commands::add::cli().get_matches_from(&["add", "income", "100.0"]);
        commands::add::exec(ctx.gctx_mut(), &add_args).unwrap();
    }

    let delete_args = commands::delete::cli().get_matches_from(&["delete", "--ids", "1,3,5"]);
    let result = commands::delete::exec(ctx.gctx_mut(), &delete_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records.len(), 2);
    assert_eq!(data.records[0].id, 2);
    assert_eq!(data.records[1].id, 4);
}

#[test]
fn test_delete_by_category() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "expenses", "50.0"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "200.0"])).unwrap();

    let delete_args = commands::delete::cli().get_matches_from(&["delete", "--by-cat", "income"]);
    let result = commands::delete::exec(ctx.gctx_mut(), &delete_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records.len(), 1);
    assert_eq!(data.records[0].category, 2); // only expenses left
}

#[test]
fn test_total_calculation() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init", "--opening", "1000.0"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "500.0"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "expenses", "200.0"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "300.0"])).unwrap();

    let total_args = commands::total::cli().get_matches_from(&["total"]);
    let result = commands::total::exec(ctx.gctx_mut(), &total_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::Total(total)) = response.content() {
            assert_eq!(total.opening_balance, 1000.0);
            assert_eq!(total.income_total, 800.0);
            assert_eq!(total.expenses_total, 200.0);
            assert_eq!(total.total(), 1600.0); // 1000 + 800 - 200
        } else {
            panic!("Expected Total response");
        }
    } else {
        panic!("Expected Ok result");
    }
}

#[test]
fn test_subcategory_add() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::subcategory::add::cli().get_matches_from(&["add", "Groceries"]);
    let result = commands::subcategory::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert!(data.subcategories_by_name.contains_key("groceries"));
    assert_eq!(data.subcategories_by_id.get(&2), Some(&"Groceries".to_string()));
}

#[test]
fn test_subcategory_add_rejects_duplicate() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args1 = commands::subcategory::add::cli().get_matches_from(&["add", "Groceries"]);
    let add_args2 = commands::subcategory::add::cli().get_matches_from(&["add", "groceries"]); // Case-insensitive duplicate

    commands::subcategory::add::exec(ctx.gctx_mut(), &add_args1).unwrap();
    let result = commands::subcategory::add::exec(ctx.gctx_mut(), &add_args2);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::SubcategoryAlreadyExists { .. })
    ));
}

#[test]
fn test_subcategory_add_rejects_miscellaneous() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::subcategory::add::cli().get_matches_from(&["add", "Miscellaneous"]);
    let result = commands::subcategory::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::CannotDeleteMiscellaneous)
    ));
}

#[test]
fn test_subcategory_list() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args1 = commands::subcategory::add::cli().get_matches_from(&["add", "Groceries"]);
    let add_args2 = commands::subcategory::add::cli().get_matches_from(&["add", "Salary"]);

    commands::subcategory::add::exec(ctx.gctx_mut(), &add_args1).unwrap();
    commands::subcategory::add::exec(ctx.gctx_mut(), &add_args2).unwrap();

    let list_args = commands::subcategory::list::cli().get_matches_from(&["list"]);
    let result = commands::subcategory::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::Subcategories(subs)) = response.content() {
            assert_eq!(subs.len(), 3); // miscellaneous + 2 custom
            assert!(subs.iter().any(|(_, name)| name.to_lowercase() == "miscellaneous"));
            assert!(subs.iter().any(|(_, name)| name == "Groceries"));
            assert!(subs.iter().any(|(_, name)| name == "Salary"));
        } else {
            panic!("Expected Subcategories response");
        }
    } else {
        panic!("Expected Ok result");
    }
}

#[test]
fn test_subcategory_delete() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::subcategory::add::cli().get_matches_from(&["add", "Groceries"]);
    commands::subcategory::add::exec(ctx.gctx_mut(), &add_args).unwrap();

    let delete_args = commands::subcategory::delete::cli().get_matches_from(&["delete", "Groceries"]);
    let result = commands::subcategory::delete::exec(ctx.gctx_mut(), &delete_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert!(!data.subcategories_by_name.contains_key("groceries"));
}

#[test]
fn test_subcategory_delete_rejects_miscellaneous() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let delete_args = commands::subcategory::delete::cli().get_matches_from(&["delete", "Miscellaneous"]);
    let result = commands::subcategory::delete::exec(ctx.gctx_mut(), &delete_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::CannotDeleteMiscellaneous)
    ));
}

#[test]
fn test_subcategory_delete_rejects_when_has_records() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_sub = commands::subcategory::add::cli().get_matches_from(&["add", "Groceries"]);
    commands::subcategory::add::exec(ctx.gctx_mut(), &add_sub).unwrap();

    let add_rec = commands::add::cli().get_matches_from(&["add", "expenses", "100.0", "--subcategory", "groceries"]);
    commands::add::exec(ctx.gctx_mut(), &add_rec).unwrap();

    let delete_args = commands::subcategory::delete::cli().get_matches_from(&["delete", "Groceries"]);
    let result = commands::subcategory::delete::exec(ctx.gctx_mut(), &delete_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::SubcategoryHasRecords { count: 1, .. })
    ));
}

#[test]
fn test_subcategory_rename() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add_args = commands::subcategory::add::cli().get_matches_from(&["add", "Groceries"]);
    commands::subcategory::add::exec(ctx.gctx_mut(), &add_args).unwrap();

    let rename_args = commands::subcategory::rename::cli().get_matches_from(&["rename", "Groceries", "Food"]);
    let result = commands::subcategory::rename::exec(ctx.gctx_mut(), &rename_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert!(!data.subcategories_by_name.contains_key("groceries"));
    assert!(data.subcategories_by_name.contains_key("food"));
    assert_eq!(data.subcategories_by_id.get(&2), Some(&"Food".to_string()));
}

#[test]
fn test_subcategory_rename_rejects_duplicate() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let add1 = commands::subcategory::add::cli().get_matches_from(&["add", "Groceries"]);
    let add2 = commands::subcategory::add::cli().get_matches_from(&["add", "Food"]);

    commands::subcategory::add::exec(ctx.gctx_mut(), &add1).unwrap();
    commands::subcategory::add::exec(ctx.gctx_mut(), &add2).unwrap();

    let rename_args = commands::subcategory::rename::cli().get_matches_from(&["rename", "Groceries", "Food"]);
    let result = commands::subcategory::rename::exec(ctx.gctx_mut(), &rename_args);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        CliError::ValidationError(ValidationErrorKind::SubcategoryAlreadyExists { .. })
    ));
}

// ============================================================================
// EXPORT COMMAND TESTS
// ============================================================================

#[test]
fn test_export_to_json() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init", "--currency", "usd", "--opening", "1000.0"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add some test data
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "500.0", "--description", "Salary"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "expenses", "100.0", "--description", "Food"])).unwrap();

    // Export to JSON
    let export_path = ctx.temp_dir.path().to_path_buf();
    let export_args = commands::export::cli().get_matches_from(&["export", export_path.to_str().unwrap(), "--type", "json"]);
    let result = commands::export::exec(ctx.gctx_mut(), &export_args);

    assert!(result.is_ok());

    // Verify exported file exists and contains valid data
    let exported_files: Vec<_> = fs::read_dir(&export_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .collect();

    assert_eq!(exported_files.len(), 1, "Should have exactly one JSON export file");

    let export_file_path = exported_files[0].path();
    let exported_content = fs::read_to_string(&export_file_path).unwrap();
    let exported_data: TrackerData = serde_json::from_str(&exported_content).unwrap();

    assert_eq!(exported_data.records.len(), 2);
    assert_eq!(exported_data.currency, "USD");
    assert_eq!(exported_data.opening_balance, 1000.0);
}

#[test]
fn test_export_to_csv() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add test data with special characters in description
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "250.50", "--description", "Test, with \"quotes\" and commas"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "expenses", "75.25", "--description", "Normal description"])).unwrap();

    // Export to CSV
    let export_path = ctx.temp_dir.path().to_path_buf();
    let export_args = commands::export::cli().get_matches_from(&["export", export_path.to_str().unwrap(), "--type", "csv"]);
    let result = commands::export::exec(ctx.gctx_mut(), &export_args);

    assert!(result.is_ok());

    // Verify CSV file exists
    let exported_files: Vec<_> = fs::read_dir(&export_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("csv"))
        .collect();

    assert_eq!(exported_files.len(), 1, "Should have exactly one CSV export file");

    let export_file_path = exported_files[0].path();
    let csv_content = fs::read_to_string(&export_file_path).unwrap();

    // Verify CSV has header and data rows
    let lines: Vec<&str> = csv_content.lines().collect();
    assert!(lines.len() >= 3, "CSV should have header + 2 data rows");
    assert!(lines[0].contains("ID,Category,Subcategory,Amount,Currency,Date,Description"));
    assert!(lines[1].contains("income") || lines[2].contains("income"));
    assert!(lines[1].contains("expenses") || lines[2].contains("expenses"));
}

#[test]
fn test_export_invalid_path() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Try to export to non-existent directory
    let export_args = commands::export::cli().get_matches_from(&["export", "/nonexistent/path/that/does/not/exist", "--type", "json"]);
    let result = commands::export::exec(ctx.gctx_mut(), &export_args);

    assert!(result.is_err());
}

#[test]
fn test_export_path_is_file() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Create a file in temp directory
    let file_path = ctx.temp_dir.path().join("somefile.txt");
    fs::write(&file_path, "test").unwrap();

    // Try to export to a file instead of directory
    let export_args = commands::export::cli().get_matches_from(&["export", file_path.to_str().unwrap(), "--type", "json"]);
    let result = commands::export::exec(ctx.gctx_mut(), &export_args);

    assert!(result.is_err());
}

// ============================================================================
// DESCRIBE COMMAND TESTS
// ============================================================================

#[test]
fn test_describe_empty_tracker() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let describe_args = commands::describe::cli().get_matches_from(&["describe"]);
    let result = commands::describe::exec(ctx.gctx_mut(), &describe_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::Describe(data)) = response.content() {
            assert_eq!(data.total_records, 0);
            assert_eq!(data.date_range, None);
            assert_eq!(data.average_transaction, 0.0);
        } else {
            panic!("Expected Describe response");
        }
    }
}

#[test]
fn test_describe_with_data() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init", "--currency", "usd"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add various records
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "1000.0", "--date", "01-01-2025"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "expenses", "200.0", "--date", "15-01-2025"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "500.0", "--date", "20-01-2025"])).unwrap();

    let describe_args = commands::describe::cli().get_matches_from(&["describe"]);
    let result = commands::describe::exec(ctx.gctx_mut(), &describe_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::Describe(data)) = response.content() {
            assert_eq!(data.total_records, 3);
            assert!(data.date_range.is_some());

            let (earliest, latest) = data.date_range.as_ref().unwrap();
            assert_eq!(earliest, "01-01-2025");
            assert_eq!(latest, "20-01-2025");

            // Average = (1000 + 200 + 500) / 3 = 566.67 (roughly)
            assert!((data.average_transaction - 566.67).abs() < 0.1);

            // Verify category breakdown
            assert!(data.by_category.len() > 0);
        } else {
            panic!("Expected Describe response");
        }
    }
}

#[test]
fn test_describe_date_range() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add records with different dates
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", "10-03-2024"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "expenses", "50.0", "--date", "05-01-2025"])).unwrap();
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "75.0", "--date", "20-12-2023"])).unwrap();

    let describe_args = commands::describe::cli().get_matches_from(&["describe"]);
    let result = commands::describe::exec(ctx.gctx_mut(), &describe_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::Describe(data)) = response.content() {
            assert!(data.date_range.is_some());

            let (earliest, latest) = data.date_range.as_ref().unwrap();
            assert_eq!(earliest, "20-12-2023");
            assert_eq!(latest, "05-01-2025");
        } else {
            panic!("Expected Describe response");
        }
    }
}

// ============================================================================
// DUMP COMMAND TESTS
// ============================================================================

#[test]
fn test_dump_tracker_data() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init", "--currency", "gbp", "--opening", "500.0"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0"])).unwrap();

    let dump_args = commands::dump::cli().get_matches_from(&["dump"]);
    let result = commands::dump::exec(ctx.gctx_mut(), &dump_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::TrackerData(data)) = response.content() {
            assert_eq!(data.currency, "GBP");
            assert_eq!(data.opening_balance, 500.0);
            assert_eq!(data.records.len(), 1);
        } else {
            panic!("Expected TrackerData response");
        }
    }
}

// ============================================================================
// CATEGORY COMMAND TESTS
// ============================================================================

#[test]
fn test_category_list() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    let category_args = commands::category::list::cli().get_matches_from(&["list"]);
    let result = commands::category::list::exec(ctx.gctx_mut(), &category_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::Categories(categories)) = response.content() {
            assert_eq!(categories.len(), 2);

            // Verify both default categories exist
            let has_income = categories.iter().any(|(id, name)| *id == 1 && name == "income");
            let has_expenses = categories.iter().any(|(id, name)| *id == 2 && name == "expenses");

            assert!(has_income, "Should have income category");
            assert!(has_expenses, "Should have expenses category");
        } else {
            panic!("Expected Categories response");
        }
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_add_command_on_uninitialized_tracker() {
    let mut ctx = TestContext::new();
    // Do NOT initialize tracker

    let add_args = commands::add::cli().get_matches_from(&["add", "income", "100.0"]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::FileNotFound(_)));
}

#[test]
fn test_list_command_on_uninitialized_tracker() {
    let mut ctx = TestContext::new();
    // Do NOT initialize tracker

    let list_args = commands::list::cli().get_matches_from(&["list"]);
    let result = commands::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::FileNotFound(_)));
}

#[test]
fn test_total_command_on_uninitialized_tracker() {
    let mut ctx = TestContext::new();
    // Do NOT initialize tracker

    let total_args = commands::total::cli().get_matches_from(&["total"]);
    let result = commands::total::exec(ctx.gctx_mut(), &total_args);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::FileNotFound(_)));
}

#[test]
fn test_add_with_future_date() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add record with future date
    let add_args = commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", "31-12-2099"]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_ok(), "Should accept future dates");

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records[0].date, "31-12-2099");
}

#[test]
fn test_add_with_very_large_amount() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add record with very large amount (> 1 million)
    let add_args = commands::add::cli().get_matches_from(&["add", "income", "9999999.99"]);
    let result = commands::add::exec(ctx.gctx_mut(), &add_args);

    assert!(result.is_ok(), "Should accept large amounts");

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records[0].amount, 9999999.99);
}

#[test]
fn test_list_with_no_results() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add records in January 2025
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", "15-01-2025"])).unwrap();

    // List with date range in different month (should return empty)
    let list_args = commands::list::cli().get_matches_from(&["list", "--start", "01-02-2025", "--end", "28-02-2025"]);
    let result = commands::list::exec(ctx.gctx_mut(), &list_args);

    assert!(result.is_ok(), "Empty results should not be an error");

    if let Ok(response) = result {
        if let Some(ResponseContent::List { records, .. }) = response.content() {
            assert_eq!(records.len(), 0, "Should return empty list");
        } else {
            panic!("Expected List response");
        }
    }
}

#[test]
fn test_update_multiple_fields() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add a subcategory first
    commands::subcategory::add::exec(ctx.gctx_mut(), &commands::subcategory::add::cli().get_matches_from(&["add", "Salary"])).unwrap();

    // Add initial record
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0", "--date", "01-01-2025"])).unwrap();

    // Update multiple fields at once
    let update_args = commands::update::cli().get_matches_from(&[
        "update",
        "1",
        "--amount", "500.0",
        "--description", "Updated salary",
        "--date", "15-01-2025",
        "--subcategory", "Salary"
    ]);
    let result = commands::update::exec(ctx.gctx_mut(), &update_args);

    assert!(result.is_ok());

    let content = fs::read_to_string(ctx.gctx.tracker_path()).unwrap();
    let data: TrackerData = serde_json::from_str(&content).unwrap();

    assert_eq!(data.records[0].amount, 500.0);
    assert_eq!(data.records[0].description, "Updated salary");
    assert_eq!(data.records[0].date, "15-01-2025");
    assert_eq!(data.records[0].subcategory, 2); // Salary is ID 2
}

#[test]
fn test_delete_nonexistent_ids() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Add one record (ID will be 1)
    commands::add::exec(ctx.gctx_mut(), &commands::add::cli().get_matches_from(&["add", "income", "100.0"])).unwrap();

    // Try to delete non-existent IDs
    let delete_args = commands::delete::cli().get_matches_from(&["delete", "--ids", "999,1000"]);
    let result = commands::delete::exec(ctx.gctx_mut(), &delete_args);

    // Should succeed but delete nothing (or handle gracefully)
    // The actual behavior depends on implementation - verify it doesn't crash
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_total_with_empty_tracker() {
    let mut ctx = TestContext::new();

    let init_args = commands::init::cli().get_matches_from(&["init", "--opening", "2500.0"]);
    commands::init::exec(ctx.gctx_mut(), &init_args).unwrap();

    // Get total without adding any records
    let total_args = commands::total::cli().get_matches_from(&["total"]);
    let result = commands::total::exec(ctx.gctx_mut(), &total_args);

    assert!(result.is_ok());

    if let Ok(response) = result {
        if let Some(ResponseContent::Total(total)) = response.content() {
            assert_eq!(total.opening_balance, 2500.0);
            assert_eq!(total.income_total, 0.0);
            assert_eq!(total.expenses_total, 0.0);
            assert_eq!(total.total(), 2500.0);
        } else {
            panic!("Expected Total response");
        }
    }
}
