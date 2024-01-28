#![allow(missing_docs)]
#![allow(clippy::wildcard_imports)]

use {
    crate::{consts::*, Namespace, StaticIRI},
    fluent_uri::Uri,
    lazy_static::lazy_static,
};

// Namespaces
#[rustfmt::skip]
lazy_static! {
    pub static ref PREFIX_BN: Namespace = Namespace::declare(PREFIX_NAME_BN, NS_BN.deref()).unwrap();
    pub static ref PREFIX_SD: Namespace = Namespace::declare(PREFIX_NAME_SD, NS_SD.deref()).unwrap();
    pub static ref PREFIX_WF: Namespace = Namespace::declare(PREFIX_NAME_WF, NS_WF.deref()).unwrap();
    pub static ref PREFIX_SBE: Namespace = Namespace::declare(PREFIX_NAME_SBE, NS_SBE.deref()).unwrap();
    pub static ref PREFIX_API: Namespace = Namespace::declare(PREFIX_NAME_API, NS_API.deref()).unwrap();
    pub static ref PREFIX_DCT: Namespace = Namespace::declare(PREFIX_NAME_DCT, NS_DCT.deref()).unwrap();
    pub static ref PREFIX_OWL: Namespace = Namespace::declare(PREFIX_NAME_OWL, NS_OWL.deref()).unwrap();
    pub static ref PREFIX_RAW: Namespace = Namespace::declare(PREFIX_NAME_RAW, NS_RAW.deref()).unwrap();
    pub static ref PREFIX_RDF: Namespace = Namespace::declare(PREFIX_NAME_RDF, NS_RDF.deref()).unwrap();
    pub static ref PREFIX_XSD: Namespace = Namespace::declare(PREFIX_NAME_XSD, NS_XSD.deref()).unwrap();
    pub static ref PREFIX_DCAT: Namespace = Namespace::declare(PREFIX_NAME_DCAT, NS_DCAT.deref()).unwrap();
    pub static ref PREFIX_JIRA: Namespace = Namespace::declare(PREFIX_NAME_JIRA, NS_JIRA.deref()).unwrap();
    pub static ref PREFIX_PROV: Namespace = Namespace::declare(PREFIX_NAME_PROV, NS_PROV.deref()).unwrap();
    pub static ref PREFIX_RDFS: Namespace = Namespace::declare(PREFIX_NAME_RDFS, NS_RDFS.deref()).unwrap();
    pub static ref PREFIX_SDLC: Namespace = Namespace::declare(PREFIX_NAME_SDLC, NS_SDLC.deref()).unwrap();
    pub static ref PREFIX_SKOS: Namespace = Namespace::declare(PREFIX_NAME_SKOS, NS_SKOS.deref()).unwrap();
    pub static ref PREFIX_EKGMM: Namespace = Namespace::declare(PREFIX_NAME_EKGMM, NS_EKGMM.deref()).unwrap();
    pub static ref PREFIX_RDFOX: Namespace = Namespace::declare(PREFIX_NAME_RDFOX, NS_RDFOX.deref()).unwrap();
    pub static ref PREFIX_STORY: Namespace = Namespace::declare(PREFIX_NAME_STORY, NS_STORY.deref()).unwrap();
    pub static ref PREFIX_DATAOPS: Namespace = Namespace::declare(PREFIX_NAME_DATAOPS, NS_DATAOPS.deref()).unwrap();
    pub static ref PREFIX_CONCEPT: Namespace = Namespace::declare(PREFIX_NAME_CONCEPT, NS_CONCEPT.deref()).unwrap();
    pub static ref PREFIX_DATASET: Namespace = Namespace::declare(PREFIX_NAME_DATASET, NS_DATASET.deref()).unwrap();
    pub static ref PREFIX_PERSONA: Namespace = Namespace::declare(PREFIX_NAME_PERSONA, NS_PERSONA.deref()).unwrap();
    pub static ref PREFIX_USE_CASE: Namespace = Namespace::declare(PREFIX_NAME_USE_CASE, NS_USE_CASE.deref()).unwrap();
    pub static ref PREFIX_STORY_IMPL_SPARQL: Namespace = Namespace::declare(PREFIX_NAME_STORY_IMPL_SPARQL, NS_STORY_IMPL_SPARQL.deref()).unwrap();
}

// Namespaces
#[rustfmt::skip]
lazy_static! {
    pub static ref NS_BN: StaticIRI = Uri::parse("https://ekgf.org/ontology/blank-node/").unwrap();
    pub static ref NS_SD: StaticIRI = Uri::parse("http://www.w3.org/ns/sparql-service-description#").unwrap();
    pub static ref NS_WF: StaticIRI = Uri::parse("https://ekgf.org/ontology/workflow-definition/").unwrap();
    pub static ref NS_SBE: StaticIRI = Uri::parse("https://ekgf.org/ontology/specification-by-example/").unwrap();
    pub static ref NS_API: StaticIRI = Uri::parse("https://ekgf.org/ontology/api/").unwrap();
    pub static ref NS_DCT: StaticIRI = Uri::parse("http://purl.org/dc/terms/").unwrap();
    pub static ref NS_OWL: StaticIRI = Uri::parse("http://www.w3.org/2002/07/owl#").unwrap();
    pub static ref NS_RAW: StaticIRI = Uri::parse("https://ekgf.org/ontology/raw/").unwrap();
    pub static ref NS_RDF: StaticIRI = Uri::parse("http://www.w3.org/1999/02/22-rdf-syntax-ns#").unwrap();
    pub static ref NS_XSD: StaticIRI = Uri::parse("http://www.w3.org/2001/XMLSchema#").unwrap();
    pub static ref NS_RDFS: StaticIRI = Uri::parse("http://www.w3.org/2000/01/rdf-schema#").unwrap();
    pub static ref NS_DCAT: StaticIRI = Uri::parse("http://www.w3.org/ns/dcat#").unwrap();
    pub static ref NS_PROV: StaticIRI = Uri::parse("http://www.w3.org/ns/prov#").unwrap();
    pub static ref NS_JIRA: StaticIRI = Uri::parse("https://ekgf.org/ontology/jira/").unwrap();
    pub static ref NS_SDLC: StaticIRI = Uri::parse("https://ekgf.org/ontology/software-development-life-cycle/").unwrap();
    pub static ref NS_SKOS: StaticIRI = Uri::parse("http://www.w3.org/2004/02/skos/core#").unwrap();
    pub static ref NS_RDFOX: StaticIRI = Uri::parse("http://oxfordsemantic.tech/RDFox#").unwrap();
    pub static ref NS_STORY: StaticIRI = Uri::parse("https://ekgf.org/ontology/story/").unwrap();
    pub static ref NS_EKGMM: StaticIRI = Uri::parse("https://ekgf.github.io/ekglib/ontology/maturity-model/").unwrap();
    pub static ref NS_CONCEPT: StaticIRI = Uri::parse("https://ekgf.org/ontology/concept/").unwrap();
    pub static ref NS_DATAOPS: StaticIRI = Uri::parse("https://ekgf.org/ontology/dataops/").unwrap();
    pub static ref NS_DATASET: StaticIRI = Uri::parse("https://ekgf.org/ontology/dataset/").unwrap();
    pub static ref NS_PERSONA: StaticIRI = Uri::parse("https://ekgf.org/ontology/persona/").unwrap();
    pub static ref NS_USE_CASE: StaticIRI = Uri::parse("https://ekgf.org/ontology/use-case/").unwrap();
    pub static ref NS_STORY_IMPL_SPARQL: StaticIRI = Uri::parse("https://ekgf.org/ontology/story-impl-sparql#").unwrap();
}
