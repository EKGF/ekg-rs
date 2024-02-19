#![allow(missing_docs)]

use {
    crate::SPARQLFlavor,
    ekg_identifier::IRIref,
    ekg_metadata::{APPLICATION_N_QUADS, APPLICATION_SPARQL_RESULTS_JSON, TEXT_PLAIN},
};

#[allow(missing_docs)]
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum SPARQLStatementType {
    SELECT(SPARQLFlavor),
    ASK(SPARQLFlavor),
    CONSTRUCT(SPARQLFlavor),
    DESCRIBE(SPARQLFlavor),
    UPDATE(SPARQLFlavor),
    DELETE(SPARQLFlavor),
}

impl SPARQLStatementType {
    pub fn from_iri(iri: Option<IRIref>, flavor: SPARQLFlavor) -> Option<Self> {
        match iri {
            Some(iri) if iri.as_str() == ekg_metadata::IRI_SELECT.as_str() => {
                Some(Self::SELECT(flavor))
            },
            Some(iri) if iri.as_str() == ekg_metadata::IRI_ASK.as_str() => Some(Self::ASK(flavor)),
            Some(iri) if iri.as_str() == ekg_metadata::IRI_CONSTRUCT.as_str() => {
                Some(Self::CONSTRUCT(flavor))
            },
            Some(iri) if iri.as_str() == ekg_metadata::IRI_DESCRIBE.as_str() => {
                Some(Self::DESCRIBE(flavor))
            },
            Some(iri) if iri.as_str() == ekg_metadata::IRI_UPDATE.as_str() => {
                Some(Self::UPDATE(flavor))
            },
            Some(iri) if iri.as_str() == ekg_metadata::IRI_DELETE.as_str() => {
                Some(Self::DELETE(flavor))
            },
            Some(iri) => {
                tracing::trace!("Unknown SPARQL Statement Type: {iri}");
                None
            },
            None => None,
        }
    }

    pub fn from_literal(literal: &ekg_metadata::Literal, flavor: SPARQLFlavor) -> Option<Self> {
        Self::from_iri(literal.as_iri_ref(), flavor)
    }

    pub fn is_query_statement(&self) -> bool {
        matches!(
            self,
            Self::SELECT(_) | Self::ASK(_) | Self::CONSTRUCT(_) | Self::DESCRIBE(_)
        )
    }

    pub fn is_update_statement(&self) -> bool { matches!(self, Self::UPDATE(_) | Self::DELETE(_)) }

    pub fn default_statement_response_mime_type(&self) -> &'static str {
        match self {
            Self::SELECT(_) => APPLICATION_SPARQL_RESULTS_JSON.as_ref(),
            Self::ASK(_) => APPLICATION_SPARQL_RESULTS_JSON.as_ref(),
            Self::CONSTRUCT(_) => APPLICATION_N_QUADS.as_ref(),
            Self::DESCRIBE(_) => APPLICATION_N_QUADS.as_ref(),
            Self::UPDATE(_) => TEXT_PLAIN.as_ref(),
            Self::DELETE(_) => TEXT_PLAIN.as_ref(),
        }
    }
}
