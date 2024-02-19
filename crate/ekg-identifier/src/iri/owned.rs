use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OwnedIRI(
    #[serde(deserialize_with = "ekg_util::serde_util::deserialize_uri")]
    #[serde(serialize_with = "ekg_util::serde_util::serialize_uri")]
    pub iri_string::types::IriReferenceString,
);

impl OwnedIRI {
    #[inline]
    pub fn as_str(&self) -> &str { self.0.as_str() }

    // noinspection DuplicatedCode
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

impl From<iri_string::types::IriReferenceString> for OwnedIRI {
    fn from(iri: iri_string::types::IriReferenceString) -> Self { Self(iri) }
}
