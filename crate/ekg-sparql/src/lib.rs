#![cfg(not(target_family = "wasm"))]
#![deny(unused_crate_dependencies)]
// #![feature(rustc_private)]
// #![feature(ptr_metadata)]

#[cfg(feature = "fs")]
use ignore as _;
pub use {
    client::SPARQLClient,
    flavor::SPARQLFlavor,
    parser::ParsedStatement,
    persistence_mode::PersistenceMode,
    prefixes::Prefixes,
    statement::{no_comments, Statement},
    statement_type::SPARQLStatementType,
};

mod client;
mod flavor;
mod parser;
mod prefixes;
mod statement;
mod statement_type;
#[cfg(test)]
mod tests;

mod persistence_mode;
#[cfg(feature = "_rdfox")]
pub mod rdfox;

pub enum FactDomain {
    ASSERTED,
    INFERRED,
    ALL,
}
