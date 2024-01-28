#![cfg(test)]

const TEST_SELECT_STATEMENT: &str = include_str!("test_select_statement.sparql");
const TEST_UPDATE_STATEMENT_WITH_ENCODE_FOR_URI_ISSUE: &str =
    include_str!("test_update_statement_with_encode_for_uri_issue.sparql");
const TEST_UPDATE_STATEMENT: &str = include_str!("test_update_statement.sparql");

#[test_log::test]
fn test_parse_select_statement() {
    let prefixes = crate::Prefixes::try_default().unwrap();
    let statement = crate::Statement::new(&prefixes, TEST_SELECT_STATEMENT.into()).unwrap();
    crate::ParsedStatement::parse(&statement, None).expect("TODO: panic message");
}

#[test_log::test]
fn test_parse_select_statement_with_encode_for_uri() {
    let prefixes = crate::Prefixes::try_default().unwrap();
    let statement = crate::Statement::new(
        &prefixes,
        TEST_UPDATE_STATEMENT_WITH_ENCODE_FOR_URI_ISSUE.into(),
    )
    .unwrap();
    crate::ParsedStatement::parse(&statement, None).expect("TODO: panic message");
}

#[test_log::test]
fn test_parse_update_statement() {
    let prefixes = crate::Prefixes::try_default().unwrap();
    let statement = crate::Statement::new(&prefixes, TEST_UPDATE_STATEMENT.into()).unwrap();
    crate::ParsedStatement::parse(&statement, None).expect("TODO: panic message");
}
