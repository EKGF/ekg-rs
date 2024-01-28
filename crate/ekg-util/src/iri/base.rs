use {
    fluent_uri::Uri,
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseIRI(
    #[serde(deserialize_with = "crate::serde_util::deserialize_uri")]
    #[serde(serialize_with = "crate::serde_util::serialize_base_uri")]
    pub Uri<String>,
);

impl BaseIRI {
    #[inline]
    pub fn as_str(&self) -> &str { self.0.as_str() }

    pub fn as_base_iri(&self) -> String {
        let str = self.as_str();
        let last_char = str.chars().last().unwrap();
        if last_char == '/' || last_char == '#' {
            str.to_string()
        } else {
            format!("{str}/")
        }
    }
}

impl From<Uri<String>> for BaseIRI {
    fn from(uri: Uri<String>) -> Self { Self(uri) }
}
