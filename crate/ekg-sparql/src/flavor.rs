#![allow(missing_docs)]

use {
    ekg_namespace::{CLASS_STORY_IMPL_SPARQL_SPARQL10, CLASS_STORY_IMPL_SPARQL_SPARQL11},
    ekg_util::iri::OwnedIRI,
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
    pub fn from_iri(iri: Option<&fluent_uri::Uri<&str>>) -> Option<Self> {
        match iri {
            Some(iri) if iri.as_str() == IRI_SPARQL10.as_str() => Some(Self::SPARQL10),
            Some(iri) if iri.as_str() == IRI_SPARQL11.as_str() => Some(Self::SPARQL11),
            _ => None,
        }
    }
}
