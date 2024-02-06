use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    pub static ref IRI_SELECT: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_SELECT.as_iri().unwrap();
    pub static ref IRI_ASK: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_ASK.as_iri().unwrap();
    pub static ref IRI_CONSTRUCT: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_CONSTRUCT.as_iri().unwrap();
    pub static ref IRI_DESCRIBE: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_DESCRIBE.as_iri().unwrap();
    pub static ref IRI_UPDATE: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_UPDATE.as_iri().unwrap();
    pub static ref IRI_DELETE: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_DELETE.as_iri().unwrap();

    pub static ref IRI_SPARQL10: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_SPARQL10.as_iri().unwrap();
    pub static ref IRI_SPARQL11: fluent_uri::Uri<String> = super::CLASS_STORY_IMPL_SPARQL_SPARQL11.as_iri().unwrap();
}
