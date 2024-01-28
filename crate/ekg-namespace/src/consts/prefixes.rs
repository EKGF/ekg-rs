//! static/const strings

use const_format::formatcp;

pub use crate::consts::*;

type PrefixName<'a> = &'a str;

pub const PREFIX_NAME_BN: PrefixName<'static> = "bn:";
pub const PREFIX_NAME_ID: PrefixName<'static> = formatcp!("{DEFAULT_ID_PATH}:");
pub const PREFIX_NAME_SD: PrefixName<'static> = "sd:";
pub const PREFIX_NAME_WF: PrefixName<'static> = "wf:";
pub const PREFIX_NAME_SBE: PrefixName<'static> = "sbe:";
pub const PREFIX_NAME_API: PrefixName<'static> = "api:";
pub const PREFIX_NAME_DCT: PrefixName<'static> = "dct:";
pub const PREFIX_NAME_OWL: PrefixName<'static> = "owl:";
pub const PREFIX_NAME_RAW: PrefixName<'static> = "raw:";
pub const PREFIX_NAME_RDF: PrefixName<'static> = "rdf:";
pub const PREFIX_NAME_XSD: PrefixName<'static> = "xsd:";
pub const PREFIX_NAME_DCAT: PrefixName<'static> = "dcat:";
pub const PREFIX_NAME_JIRA: PrefixName<'static> = "jira:";
pub const PREFIX_NAME_PROV: PrefixName<'static> = "prov:";
pub const PREFIX_NAME_RDFS: PrefixName<'static> = "rdfs:";
pub const PREFIX_NAME_RULE: PrefixName<'static> = "rule:";
pub const PREFIX_NAME_SDLC: PrefixName<'static> = "sdlc:";
pub const PREFIX_NAME_SKOS: PrefixName<'static> = "skos:";
pub const PREFIX_NAME_EKGMM: PrefixName<'static> = "ekgmm:";
pub const PREFIX_NAME_GRAPH: PrefixName<'static> = formatcp!("{DEFAULT_GRAPH_PATH}:");
pub const PREFIX_NAME_RDFOX: PrefixName<'static> = "rdfox:";
pub const PREFIX_NAME_STORY: PrefixName<'static> = "story:";
pub const PREFIX_NAME_CONCEPT: PrefixName<'static> = "concept:";
pub const PREFIX_NAME_DATASET: PrefixName<'static> = "dataset:";
pub const PREFIX_NAME_DATAOPS: PrefixName<'static> = "dataops:";
pub const PREFIX_NAME_PERSONA: PrefixName<'static> = "persona:";
pub const PREFIX_NAME_USE_CASE: PrefixName<'static> = "usecase:";
pub const PREFIX_NAME_STORY_IMPL_SPARQL: PrefixName<'static> = "sparqlstory:";
pub const PREFIX_NAME_STORY_IMPL_SQL: PrefixName<'static> = "sqlstory:";
