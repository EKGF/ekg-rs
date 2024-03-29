use {
    crate::Graph,
    core::str::FromStr,
    ekg_identifier::NS_RDFOX,
    lazy_static::lazy_static,
    mime::Mime,
};
pub use {classes::*, local_names::*, predicates::*, sparql::*};

mod classes;
mod local_names;
mod predicates;
mod sparql;

/// What it's called
pub const EMPTY_STRING: &String = &String::new();

// All supported MIME types
lazy_static! {
    // As documented here: https://docs.oxfordsemantic.tech/5.6/programmatic-access-APIs.html#formats-encoding-sparql-query-results
    #[doc(hidden)]
    pub static ref TEXT_PLAIN: Mime = Mime::from_str("text/plain").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_TSV: Mime = Mime::from_str("text/tab-separated-values").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_CSV: Mime = Mime::from_str("text/csv").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_X_CSV_ABBREV: Mime = Mime::from_str("text/x.csv-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_TURTLE: Mime = Mime::from_str("text/turtle").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_OWL_FUNCTIONAL: Mime = Mime::from_str("text/owl-functional").unwrap();
    #[doc(hidden)]
    pub static ref TEXT_X_TAB_SEPARATED_VALUES_ABBREV: Mime =
        Mime::from_str("text/x.tab-separated-values-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_TRIG: Mime = Mime::from_str("application/trig").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_N_QUADS: Mime = Mime::from_str("application/n-quads").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_N_TRIPLES: Mime = Mime::from_str("application/n-triples").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_DATALOG: Mime = Mime::from_str("application/x.datalog").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_SPARQL_RESULTS_XML: Mime =
        Mime::from_str("application/sparql-results+xml").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_SPARQL_RESULTS_JSON: Mime =
        Mime::from_str("application/sparql-results+json").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_SPARQL_RESULTS_TURTLE: Mime =
        Mime::from_str("application/sparql-results+turtle").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_XML_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+xml-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_JSON_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+json-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_TURTLE_ABBREV: Mime =
        Mime::from_str("application/x.sparql-results+turtle-abbrev").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_RESOURCEID: Mime =
        Mime::from_str("application/x.sparql-results+resourceid").unwrap();
    #[doc(hidden)]
    pub static ref APPLICATION_X_SPARQL_RESULTS_NULL: Mime =
        Mime::from_str("application/x.sparql-results+null").unwrap();
}

lazy_static! {
    #[doc(hidden)]
    pub static ref DEFAULT_GRAPH_RDFOX: Graph =
        Graph::declare(NS_RDFOX.deref().clone(), "DefaultTriples");
}
