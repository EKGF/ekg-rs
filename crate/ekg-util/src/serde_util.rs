use serde::{de::Error, Deserialize, Serializer};

pub fn serialize_uri<S>(uri: &fluent_uri::Uri<String>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    serializer.serialize_str(uri.as_str())
}

/// Serialize a `fluent_uri::Uri` that represents a base URI making sure
/// that it either ends with a slash or a hash (the latter can be the last
/// character of a namespace IRI in RDF).
pub fn serialize_base_uri<S>(
    uri: &fluent_uri::Uri<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let last_char = uri.as_str().chars().last().unwrap();
    if last_char == '/' || last_char == '#' {
        serializer.serialize_str(uri.as_str())
    } else {
        serializer.serialize_str(format!("{}/", uri.as_str()).as_str())
    }
}

pub fn deserialize_uri<'de, D>(deserializer: D) -> Result<fluent_uri::Uri<String>, D::Error>
where D: serde::Deserializer<'de> {
    let s: String = Deserialize::deserialize(deserializer)?;
    fluent_uri::Uri::parse_from(s).map_err(|e| D::Error::custom(e.0))
}

pub fn serialize_bool_as_uppercase<S>(b: &bool, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    serializer.serialize_str(if *b { "TRUE" } else { "FALSE" })
}

pub fn deserialize_bool_as_uppercase<'de, D>(deserializer: D) -> Result<bool, D::Error>
where D: serde::Deserializer<'de> {
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        _ => {
            Err(D::Error::custom(format!(
                "Expected 'TRUE' or 'FALSE' but got '{}'",
                s
            )))
        },
    }
}
