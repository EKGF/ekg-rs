use {crate::StaticIRI, lazy_static::lazy_static};

// Namespaces
#[rustfmt::skip]
lazy_static! {
    pub static ref NS_IRI_BN: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/blank-node#").unwrap();
    pub static ref NS_IRI_SD: StaticIRI = StaticIRI::try_from("http://www.w3.org/ns/sparql-service-description#").unwrap();
    pub static ref NS_IRI_WF: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/workflow-definition#").unwrap();
    pub static ref NS_IRI_UI: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/user-interface#").unwrap();
    pub static ref NS_IRI_SBE: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/specification-by-example#").unwrap();
    pub static ref NS_IRI_API: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/api#").unwrap();
    pub static ref NS_IRI_DCT: StaticIRI = StaticIRI::try_from("http://purl.org/dc/terms/").unwrap();
    pub static ref NS_IRI_OWL: StaticIRI = StaticIRI::try_from("http://www.w3.org/2002/07/owl#").unwrap();
    pub static ref NS_IRI_RAW: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/raw#").unwrap();
    pub static ref NS_IRI_RDF: StaticIRI = StaticIRI::try_from("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref NS_IRI_XSD: StaticIRI = StaticIRI::try_from("http://www.w3.org/2001/XMLSchema#").unwrap();
    pub static ref NS_IRI_RDFS: StaticIRI = StaticIRI::try_from("http://www.w3.org/2000/01/rdf-schema#").unwrap();
    pub static ref NS_IRI_DCAT: StaticIRI = StaticIRI::try_from("http://www.w3.org/ns/dcat#").unwrap();
    pub static ref NS_IRI_PROV: StaticIRI = StaticIRI::try_from("http://www.w3.org/ns/prov#").unwrap();
    pub static ref NS_IRI_JIRA: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/jira#").unwrap();
    pub static ref NS_IRI_SDLC: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/software-development-life-cycle#").unwrap();
    pub static ref NS_IRI_SKOS: StaticIRI = StaticIRI::try_from("http://www.w3.org/2004/02/skos/core#").unwrap();
    pub static ref NS_IRI_RDFOX: StaticIRI = StaticIRI::try_from("http://oxfordsemantic.tech/RDFox#").unwrap();
    pub static ref NS_IRI_STORY: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/story#").unwrap();
    pub static ref NS_IRI_EKGMM: StaticIRI = StaticIRI::try_from("https://ekgf.github.io/ekglib/ontology/maturity-model#").unwrap();
    pub static ref NS_IRI_CONCEPT: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/concept#").unwrap();
    pub static ref NS_IRI_DATAOPS: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/dataops#").unwrap();
    pub static ref NS_IRI_DATASET: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/dataset#").unwrap();
    pub static ref NS_IRI_PERSONA: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/persona#").unwrap();
    pub static ref NS_IRI_USE_CASE: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/use-case#").unwrap();
    pub static ref NS_IRI_STORY_IMPL_SPARQL: StaticIRI = StaticIRI::try_from("https://ekgf.org/ontology/story-impl-sparql#").unwrap();
}
