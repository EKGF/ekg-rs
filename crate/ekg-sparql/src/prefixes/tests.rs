#![cfg(test)]

#[test_log::test]
fn test_default_namespaces() -> Result<(), ekg_error::Error> {
    let prefixes = crate::prefixes::Prefixes::builder()
        .default_namespaces()
        .build()?;
    assert_eq!(prefixes.len(), 4);
    Ok(())
}
