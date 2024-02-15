use {crate::StaticIRI, fluent_uri::Uri, lazy_static::lazy_static};

// Namespaces
#[rustfmt::skip]
lazy_static! {
    pub static ref NS_BN: StaticIRI = Uri::parse("https://ekgf.org/ontology/blank-node#").unwrap();
    pub static ref NS_SD: StaticIRI = Uri::parse("http://www.w3.org/ns/sparql-service-description#").unwrap();
    pub static ref NS_WF: StaticIRI = Uri::parse("https://ekgf.org/ontology/workflow-definition#").unwrap();
    pub static ref NS_UI: StaticIRI = Uri::parse("https://ekgf.org/ontology/user-interface#").unwrap();
    pub static ref NS_SBE: StaticIRI = Uri::parse("https://ekgf.org/ontology/specification-by-example#").unwrap();
    pub static ref NS_API: StaticIRI = Uri::parse("https://ekgf.org/ontology/api#").unwrap();
    pub static ref NS_DCT: StaticIRI = Uri::parse("http://purl.org/dc/terms/").unwrap();
    pub static ref NS_OWL: StaticIRI = Uri::parse("http://www.w3.org/2002/07/owl#").unwrap();
    pub static ref NS_RAW: StaticIRI = Uri::parse("https://ekgf.org/ontology/raw#").unwrap();
    pub static ref NS_RDF: StaticIRI = Uri::parse("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref NS_XSD: StaticIRI = Uri::parse("http://www.w3.org/2001/XMLSchema#").unwrap();
    pub static ref NS_RDFS: StaticIRI = Uri::parse("http://www.w3.org/2000/01/rdf-schema#").unwrap();
    pub static ref NS_DCAT: StaticIRI = Uri::parse("http://www.w3.org/ns/dcat#").unwrap();
    pub static ref NS_PROV: StaticIRI = Uri::parse("http://www.w3.org/ns/prov#").unwrap();
    pub static ref NS_JIRA: StaticIRI = Uri::parse("https://ekgf.org/ontology/jira#").unwrap();
    pub static ref NS_SDLC: StaticIRI = Uri::parse("https://ekgf.org/ontology/software-development-life-cycle#").unwrap();
    pub static ref NS_SKOS: StaticIRI = Uri::parse("http://www.w3.org/2004/02/skos/core#").unwrap();
    pub static ref NS_RDFOX: StaticIRI = Uri::parse("http://oxfordsemantic.tech/RDFox#").unwrap();
    pub static ref NS_STORY: StaticIRI = Uri::parse("https://ekgf.org/ontology/story#").unwrap();
    pub static ref NS_EKGMM: StaticIRI = Uri::parse("https://ekgf.github.io/ekglib/ontology/maturity-model#").unwrap();
    pub static ref NS_CONCEPT: StaticIRI = Uri::parse("https://ekgf.org/ontology/concept#").unwrap();
    pub static ref NS_DATAOPS: StaticIRI = Uri::parse("https://ekgf.org/ontology/dataops#").unwrap();
    pub static ref NS_DATASET: StaticIRI = Uri::parse("https://ekgf.org/ontology/dataset#").unwrap();
    pub static ref NS_PERSONA: StaticIRI = Uri::parse("https://ekgf.org/ontology/persona#").unwrap();
    pub static ref NS_USE_CASE: StaticIRI = Uri::parse("https://ekgf.org/ontology/use-case#").unwrap();
    pub static ref NS_STORY_IMPL_SPARQL: StaticIRI = Uri::parse("https://ekgf.org/ontology/story-impl-sparql#").unwrap();
}
