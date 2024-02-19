use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    pub static ref IRI_SELECT: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_SELECT.as_iri().unwrap();
    pub static ref IRI_ASK: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_ASK.as_iri().unwrap();
    pub static ref IRI_CONSTRUCT: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_CONSTRUCT.as_iri().unwrap();
    pub static ref IRI_DESCRIBE: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_DESCRIBE.as_iri().unwrap();
    pub static ref IRI_UPDATE: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_UPDATE.as_iri().unwrap();
    pub static ref IRI_DELETE: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_DELETE.as_iri().unwrap();

    pub static ref IRI_SPARQL10: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_SPARQL10.as_iri().unwrap();
    pub static ref IRI_SPARQL11: iri_string::types::IriReferenceString = super::CLASS_STORY_IMPL_SPARQL_SPARQL11.as_iri().unwrap();
}
