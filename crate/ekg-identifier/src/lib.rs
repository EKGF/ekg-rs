#![deny(unused_crate_dependencies)]

pub use {
    consts::*,
    ekg_identifier_context::{EkgIdentifierContext, EkgIdentifierContexts},
    iri::{
        mandatory_env_var_base_iri,
        mandatory_env_var_uri,
        ABoxNamespaceIRI,
        NamespaceIRI,
        OwnedIRI,
        TBoxNamespaceIRI,
    },
    namespace::Namespace,
};

pub mod iref {
    pub use iref::*;
}

pub mod consts;
mod ekg_identifier_context;
pub mod iri;
pub mod namespace;

pub type StaticIRI = iri_string::types::IriReferenceString;
pub type IRIref<'a> = &'a iri_string::types::IriReferenceStr;
