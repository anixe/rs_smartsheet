// These are tests running on the real service
//
// CAUTION!
// DATA FROM THE TEST SHEET WILL BE LOST!
//
// Requirements:
// - internet connection
// - Smartsheet account with generated API token
// - test table with at least 2 text/number columns named 'JIRA' and 'AWP' and 2 existing rows
//
// Running:
// REAL_TEST_TOKEN=<API token> REAL_TEST_SHEET=<test table name> cargo test --features "run_real_service_tests"

#![cfg(feature = "run_real_service_tests")]

#[macro_use]
extern crate lazy_static;
extern crate smartsheet;

use smartsheet::{CellValue, Client, ColumnId, RowId, Smartsheet};
use std::sync::{Mutex, MutexGuard, PoisonError};

const TOKEN: &str = env!("REAL_TEST_TOKEN");
const SHEET: &str = env!("REAL_TEST_SHEET");

lazy_static! {
    static ref REAL_SERVICE_USAGE_MUTEX: Mutex<()> = Mutex::new(());
}

fn acquire_unique_service_access() -> MutexGuard<'static, ()> {
    REAL_SERVICE_USAGE_MUTEX.lock()
        .unwrap_or_else(PoisonError::into_inner)
}

fn fetch_smartsheet() -> Smartsheet {
    let client = Client::new(TOKEN);
    Smartsheet::fetch(&client, SHEET)
        .expect(&format!("Failed to fetch sheet {}", SHEET))
}

fn init_smartsheet() {
    let mut smartsheet = fetch_smartsheet();
    let jira_column_id = &smartsheet.get_column_id("JIRA").unwrap();
    let awp_column_id = &smartsheet.get_column_id("AWP").unwrap();
    let row_1_id = &smartsheet.find_row_id(|_| true).unwrap();
    let row_2_id = &smartsheet.find_row_id(|row| &row.get_row_id() != row_1_id).unwrap();
    smartsheet.push_cell_value(jira_column_id,  row_1_id, "JIRA_1").unwrap();
    smartsheet.push_cell_value(awp_column_id,   row_1_id, "AWP_1").unwrap();
    smartsheet.push_cell_value(jira_column_id,  row_2_id, "JIRA_2").unwrap();
    smartsheet.push_cell_value(awp_column_id,   row_2_id, "AWP_2").unwrap();
}

fn assert_cell(expected_str: &str, smartsheet: &Smartsheet, column_id: &ColumnId, row_id: &RowId) {
    let expected = &CellValue::from(expected_str);
    let actual = smartsheet.get_cell_value(column_id, row_id).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn find_and_update_row() {
    let _service_guard = acquire_unique_service_access();
    init_smartsheet();

    let mut smartsheet = fetch_smartsheet();
    let jira_column_id = &smartsheet.get_column_id("JIRA").unwrap();
    let awp_column_id = &smartsheet.get_column_id("AWP").unwrap();
    let row_1_id = &smartsheet.find_row_id(|_| true).unwrap();
    let row_2_id = &smartsheet.find_row_id(|row| &row.get_row_id() != row_1_id).unwrap();


    let searched_value = CellValue::from("JIRA_2");
    let row_id = &smartsheet.find_row_id(|row| row.get_cell_value(jira_column_id) == Some(&searched_value)).unwrap();
    smartsheet.push_cell_value(awp_column_id, row_id, "AWP_NEW").unwrap();


    //Assert local values
    assert_cell("JIRA_1",   &smartsheet, jira_column_id,    row_1_id);
    assert_cell("AWP_1",    &smartsheet, awp_column_id,     row_1_id);
    assert_cell("JIRA_2",   &smartsheet, jira_column_id,    row_2_id);
    assert_cell("AWP_NEW",  &smartsheet, awp_column_id,     row_2_id);

    //Assert values on server
    let smartsheet_remote = fetch_smartsheet();
    assert_cell("JIRA_1",   &smartsheet_remote, jira_column_id, row_1_id);
    assert_cell("AWP_1",    &smartsheet_remote, awp_column_id,  row_1_id);
    assert_cell("JIRA_2",   &smartsheet_remote, jira_column_id, row_2_id);
    assert_cell("AWP_NEW",  &smartsheet_remote, awp_column_id,  row_2_id);
}


