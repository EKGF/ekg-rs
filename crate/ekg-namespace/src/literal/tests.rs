#![cfg(test)]

#[cfg(feature = "serde")]
use serde_json::json;

use crate::literal::this::Literal;

#[test]
fn test_as_local_name_01() -> Result<(), ekg_error::Error> {
    let val = Literal::from_iri(&fluent_uri::Uri::from_static(
        "https://whatever.kg/id/abc",
    ));
    assert!(val.is_ok());
    let val = val.unwrap();
    let name = val.as_local_name();
    assert!(name.is_some());
    let name = name.unwrap();
    assert_eq!(name, "abc");
    Ok(())
}

#[test]
fn test_as_local_name_02() -> Result<(), ekg_error::Error> {
    let val = Literal::from_iri(&fluent_uri::Uri::from_static(
        "https://whatever.kg/id#abc",
    ));
    assert!(val.is_ok());
    let val = val.unwrap();
    let name = val.as_local_name();
    assert!(name.is_some());
    let name = name.unwrap();
    assert_eq!(name, "abc");
    Ok(())
}

#[test]
fn test_id_url_01() -> Result<(), ekg_error::Error> {
    let id_base_iri = fluent_uri::Uri::from_str("https://whatever.kg/id/")?;
    let literal = Literal::from_iri(&fluent_uri::Uri::from_static(
        "https://whatever.kg/id/abc",
    ));
    assert!(literal.is_ok());
    let literal = literal.unwrap();
    assert!(literal.is_id_iri(&id_base_iri));
    let id = literal.as_id(&id_base_iri)?;
    assert_eq!(id, "abc");
    Ok(())
}

#[test]
fn test_id_url_02() -> Result<(), ekg_error::Error> {
    let id_base_iri = fluent_uri::Uri::from_str("https://whatever.kg/id/")?;
    let literal = Literal::from_iri(&fluent_uri::Uri::from_static(
        "https://whatever.kg/id/abc",
    ));
    assert!(literal.is_ok());
    let literal = literal.unwrap();
    assert!(literal.is_id_iri(&id_base_iri));
    let id = format!("{:}", literal.as_id_url_display(&id_base_iri));
    assert_eq!(id, "abc");
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_01() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("http://abc.de/whatever"))?;
    assert_eq!(
        format!("{literal:?}"),
        "Literal(AnyUri,<http://abc.de/whatever>))"
    );
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_02() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("http://abc.de/whatever"))?;
    assert_eq!(
        format!("{literal:?}"),
        "Literal(AnyUri,<http://abc.de/whatever>))"
    );
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_03() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("123"))?;
    assert_eq!(format!("{literal:?}"), "Literal(Integer,123)");
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_04() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("123.45"))?;
    assert_eq!(format!("{literal:?}"), "Literal(Decimal,123.45)");
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_05() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("2023-12-31"))?;
    assert_eq!(format!("{literal:?}"), "Literal(Date,2023-12-31)");
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_06() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("2023-12-31 13:21"))?;
    assert_eq!(
        format!("{literal:?}"),
        "Literal(DateTime,2023-12-31 13:21:00 UTC)"
    );
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_deserialize_07() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("2023-12-31 13:21:22"))?;
    assert_eq!(
        format!("{literal:?}"),
        "Literal(DateTime,2023-12-31 13:21:22 UTC)"
    );
    Ok(())
}

// RFC 3339
#[cfg(feature = "serde")]
#[test]
fn test_deserialize_08() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("1996-12-19T16:39:57-08:00"))?;
    assert_eq!(
        format!("{literal:?}"),
        "Literal(DateTime,1996-12-20 00:39:57 UTC)"
    );
    Ok(())
}

// RFC 2822
#[cfg(feature = "serde")]
#[test]
fn test_deserialize_09() -> Result<(), serde_json::Error> {
    let literal: Literal = serde_json::from_value(json!("Wed, 18 Feb 2015 23:16:09 EST"))?;
    assert_eq!(
        format!("{literal:?}"),
        "Literal(DateTime,2015-02-19 04:16:09 UTC)"
    );
    Ok(())
}
