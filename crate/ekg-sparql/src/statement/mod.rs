//! Represent a SPARQL Statement
pub use {
    statement_type::SPARQLStatementType,
    this::{Statement, RDFOX_QUERY_VALIDATION, RDFOX_QUERY_VALIDATION_STANDARD_COMPLIANT},
    utils::no_comments,
};
mod statement_type;
#[cfg(test)]
mod tests;
mod this;
mod utils;
