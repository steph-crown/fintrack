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
