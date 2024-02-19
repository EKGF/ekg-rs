#![allow(missing_docs)]

use {
    ekg_identifier::OwnedIRI,
    ekg_metadata::{CLASS_STORY_IMPL_SPARQL_SPARQL10, CLASS_STORY_IMPL_SPARQL_SPARQL11},
    lazy_static::lazy_static,
};

#[rustfmt::skip]
lazy_static! {
    pub static ref IRI_SPARQL10: OwnedIRI = CLASS_STORY_IMPL_SPARQL_SPARQL10.as_iri().unwrap().into();
    pub static ref IRI_SPARQL11: OwnedIRI = CLASS_STORY_IMPL_SPARQL_SPARQL11.as_iri().unwrap().into();
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum SPARQLFlavor {
    SPARQL10,
    SPARQL11,
}

impl SPARQLFlavor {
    pub fn from_iri(iri: Option<&iri_string::types::IriReferenceStr>) -> Option<Self> {
        match iri {
            Some(iri) if iri.as_str() == IRI_SPARQL10.as_str() => Some(Self::SPARQL10),
            Some(iri) if iri.as_str() == IRI_SPARQL11.as_str() => Some(Self::SPARQL11),
            _ => None,
        }
    }

    pub fn from_literal(literal: &ekg_metadata::Literal) -> Option<Self> {
        literal
            .as_iri_ref()
            .and_then(|iri| Self::from_iri(Some(iri)))
    }
}
