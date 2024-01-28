#![deny(unused_crate_dependencies)]

pub use {
    client::SPARQLClient,
    flavor::SPARQLFlavor,
    parser::ParsedStatement,
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
