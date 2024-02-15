use {super::prefixes::*, crate::Namespace, lazy_static::lazy_static};

// Prefixes
#[rustfmt::skip]
lazy_static! {
    pub static ref PREFIX_BN: Namespace = Namespace::declare(PREFIX_NAME_BN, NS_BN.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_SD: Namespace = Namespace::declare(PREFIX_NAME_SD, NS_SD.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_WF: Namespace = Namespace::declare(PREFIX_NAME_WF, NS_WF.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_UI: Namespace = Namespace::declare(PREFIX_NAME_UI, NS_UI.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_SBE: Namespace = Namespace::declare(PREFIX_NAME_SBE, NS_SBE.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_API: Namespace = Namespace::declare(PREFIX_NAME_API, NS_API.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_DCT: Namespace = Namespace::declare(PREFIX_NAME_DCT, NS_DCT.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_OWL: Namespace = Namespace::declare(PREFIX_NAME_OWL, NS_OWL.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_RAW: Namespace = Namespace::declare(PREFIX_NAME_RAW, NS_RAW.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_RDF: Namespace = Namespace::declare(PREFIX_NAME_RDF, NS_RDF.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_XSD: Namespace = Namespace::declare(PREFIX_NAME_XSD, NS_XSD.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_DCAT: Namespace = Namespace::declare(PREFIX_NAME_DCAT, NS_DCAT.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_JIRA: Namespace = Namespace::declare(PREFIX_NAME_JIRA, NS_JIRA.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_PROV: Namespace = Namespace::declare(PREFIX_NAME_PROV, NS_PROV.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_RDFS: Namespace = Namespace::declare(PREFIX_NAME_RDFS, NS_RDFS.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_SDLC: Namespace = Namespace::declare(PREFIX_NAME_SDLC, NS_SDLC.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_SKOS: Namespace = Namespace::declare(PREFIX_NAME_SKOS, NS_SKOS.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_EKGMM: Namespace = Namespace::declare(PREFIX_NAME_EKGMM, NS_EKGMM.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_RDFOX: Namespace = Namespace::declare(PREFIX_NAME_RDFOX, NS_RDFOX.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_STORY: Namespace = Namespace::declare(PREFIX_NAME_STORY, NS_STORY.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_DATAOPS: Namespace = Namespace::declare(PREFIX_NAME_DATAOPS, NS_DATAOPS.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_CONCEPT: Namespace = Namespace::declare(PREFIX_NAME_CONCEPT, NS_CONCEPT.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_DATASET: Namespace = Namespace::declare(PREFIX_NAME_DATASET, NS_DATASET.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_PERSONA: Namespace = Namespace::declare(PREFIX_NAME_PERSONA, NS_PERSONA.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_USE_CASE: Namespace = Namespace::declare(PREFIX_NAME_USE_CASE, NS_USE_CASE.as_str().try_into().unwrap()).unwrap();
    pub static ref PREFIX_STORY_IMPL_SPARQL: Namespace = Namespace::declare(PREFIX_NAME_STORY_IMPL_SPARQL, NS_STORY_IMPL_SPARQL.as_str().try_into().unwrap()).unwrap();
}
