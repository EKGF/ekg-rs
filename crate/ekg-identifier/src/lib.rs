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

pub mod consts;
mod ekg_identifier_context;
pub mod iri;
pub mod namespace;

pub type StaticIRI = fluent_uri::Uri<&'static str>;
pub type IRIref<'a> = &'a fluent_uri::Uri<&'a str>;
