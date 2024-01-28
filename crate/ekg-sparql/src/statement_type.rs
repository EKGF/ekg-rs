#![allow(missing_docs)]

use {crate::SPARQLFlavor, ekg_namespace::IRIref};

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
            Some(iri) if iri.as_str() == ekg_namespace::IRI_SELECT.as_str() => {
                Some(Self::SELECT(flavor))
            },
            Some(iri) if iri.as_str() == ekg_namespace::IRI_ASK.as_str() => Some(Self::ASK(flavor)),
            Some(iri) if iri.as_str() == ekg_namespace::IRI_CONSTRUCT.as_str() => {
                Some(Self::CONSTRUCT(flavor))
            },
            Some(iri) if iri.as_str() == ekg_namespace::IRI_DESCRIBE.as_str() => {
                Some(Self::DESCRIBE(flavor))
            },
            Some(iri) if iri.as_str() == ekg_namespace::IRI_UPDATE.as_str() => {
                Some(Self::UPDATE(flavor))
            },
            Some(iri) if iri.as_str() == ekg_namespace::IRI_DELETE.as_str() => {
                Some(Self::DELETE(flavor))
            },
            Some(iri) => {
                tracing::trace!("Unknown SPARQL Statement Type: {iri}");
                None
            },
            None => None,
        }
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
            Self::SELECT(_) => "application/sparql-results+json",
            Self::ASK(_) => "application/sparql-results+json",
            Self::CONSTRUCT(_) => "application/n-quads",
            Self::DESCRIBE(_) => "application/n-quads",
            Self::UPDATE(_) => "text/plain",
            Self::DELETE(_) => "text/plain",
        }
    }
}
