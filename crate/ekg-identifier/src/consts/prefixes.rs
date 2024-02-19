//! static/const strings

use const_format::formatcp;

pub use crate::consts::*;

type PrefixName<'a> = &'a str;

pub const NS_PREFIX_BN: PrefixName<'static> = "bn:";
/// The default prefix for the namespace that contains
/// all EKG IRIs like `https://placeholder.kg/id/<ekg identifier>`
/// where `<ekg identifier>` is a unique identifier for an EKG entity.
pub const NS_PREFIX_ID: PrefixName<'static> = "id:";
pub const NS_PREFIX_SD: PrefixName<'static> = "sd:";
pub const NS_PREFIX_WF: PrefixName<'static> = "wf:";
pub const NS_PREFIX_UI: PrefixName<'static> = "ui:";
pub const NS_PREFIX_SBE: PrefixName<'static> = "sbe:";
pub const NS_PREFIX_API: PrefixName<'static> = "api:";
pub const NS_PREFIX_DCT: PrefixName<'static> = "dct:";
pub const NS_PREFIX_OWL: PrefixName<'static> = "owl:";
pub const NS_PREFIX_RAW: PrefixName<'static> = "raw:";
pub const NS_PREFIX_RDF: PrefixName<'static> = "rdf:";
pub const NS_PREFIX_XSD: PrefixName<'static> = "xsd:";
pub const NS_PREFIX_DCAT: PrefixName<'static> = "dcat:";
pub const NS_PREFIX_JIRA: PrefixName<'static> = "jira:";
pub const NS_PREFIX_PROV: PrefixName<'static> = "prov:";
pub const NS_PREFIX_RDFS: PrefixName<'static> = "rdfs:";
pub const NS_PREFIX_RULE: PrefixName<'static> = "rule:";
pub const NS_PREFIX_SDLC: PrefixName<'static> = "sdlc:";
pub const NS_PREFIX_SKOS: PrefixName<'static> = "skos:";
pub const NS_PREFIX_EKGMM: PrefixName<'static> = "ekgmm:";
pub const NS_PREFIX_GRAPH: PrefixName<'static> = formatcp!("{DEFAULT_GRAPH_PATH}:");
pub const NS_PREFIX_RDFOX: PrefixName<'static> = "rdfox:";
pub const NS_PREFIX_STORY: PrefixName<'static> = "story:";
pub const NS_PREFIX_CONCEPT: PrefixName<'static> = "concept:";
pub const NS_PREFIX_DATASET: PrefixName<'static> = "dataset:";
pub const NS_PREFIX_DATAOPS: PrefixName<'static> = "dataops:";
pub const NS_PREFIX_PERSONA: PrefixName<'static> = "persona:";
pub const NS_PREFIX_USE_CASE: PrefixName<'static> = "usecase:";
pub const NS_PREFIX_STORY_IMPL_SPARQL: PrefixName<'static> = "sparqlstory:";
pub const NS_PREFIX_STORY_IMPL_SQL: PrefixName<'static> = "sqlstory:";
