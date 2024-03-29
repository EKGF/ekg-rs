use std::mem::ManuallyDrop;

/// A `LiteralValue` is, together with a [`DataType`](crate::DataType), part
/// of a [`Literal`](crate::Literal).
pub union LiteralValue {
    pub iri:              ManuallyDrop<iri_string::types::IriReferenceString>,
    pub string:           ManuallyDrop<String>,
    pub boolean:          bool,
    pub unsigned_integer: u64,
    pub signed_integer:   i64,
    pub date:             ManuallyDrop<chrono::NaiveDate>,
    pub date_time:        chrono::DateTime<chrono::Utc>,
    pub blank_node:       ManuallyDrop<String>,
}

impl Default for LiteralValue {
    fn default() -> Self { Self { boolean: false } }
}

impl LiteralValue {
    pub fn new_string(str: &str) -> Self { Self { string: ManuallyDrop::new(str.to_string()) } }

    pub fn new_iref_iri(iri: &iref::Iri) -> Result<Self, ekg_error::Error> {
        Ok(Self {
            iri: ManuallyDrop::new(
                iri_string::types::IriReferenceString::try_from(iri.as_str())?.to_owned(),
            ),
        })
    }

    pub fn new_iri(iri: &iri_string::types::IriReferenceStr) -> Self {
        Self { iri: ManuallyDrop::new(iri.to_owned()) }
    }

    pub fn new_boolean(boolean: bool) -> Self { Self { boolean } }

    pub fn new_unsigned_integer(unsigned_integer: u64) -> Self { Self { unsigned_integer } }

    pub fn new_signed_integer(signed_integer: i64) -> Self { Self { signed_integer } }

    pub fn new_date(date: chrono::NaiveDate) -> Self { Self { date: ManuallyDrop::new(date) } }

    pub fn new_date_time(date_time: chrono::DateTime<chrono::Utc>) -> Self { Self { date_time } }

    pub fn new_blank_node(blank_node: &str) -> Self {
        Self {
            blank_node: ManuallyDrop::new(blank_node.to_string()),
        }
    }
}
