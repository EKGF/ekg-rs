#![deny(unused_crate_dependencies)]

pub use {
    class::Class,
    consts::*,
    data_type::DataType,
    graph::{Graph, GraphDisplayIRI},
    literal::{Literal, LiteralIdUrlDisplay, LiteralUrlDisplay, LiteralValue},
    namespace::Namespace,
    predicate::Predicate,
    term::Term,
};

mod class;
mod consts;
mod data_type;
mod graph;
mod literal;
mod namespace;
mod predicate;
mod term;

pub type StaticIRI = fluent_uri::Uri<&'static str>;
pub type IRIref<'a> = &'a fluent_uri::Uri<&'a str>;
