use {
    crate::{iri::ABoxNamespaceIRI, mandatory_env_var_base_iri},
    ekg_error::Error,
};

pub struct EkgIdentifierContext {
    pub ekg_base:          ABoxNamespaceIRI,
    pub ekg_id_base:       ABoxNamespaceIRI,
    pub ekg_graph_base:    ABoxNamespaceIRI,
    pub ekg_ontology_base: ABoxNamespaceIRI,
}

impl EkgIdentifierContext {
    pub fn from_env(suffix: &'static str) -> Result<Self, Error> {
        Ok(Self {
            ekg_base:          mandatory_env_var_base_iri("EKG_BASE", Some(suffix))?,
            ekg_id_base:       mandatory_env_var_base_iri("EKG_ID_BASE", Some(suffix))?,
            ekg_graph_base:    mandatory_env_var_base_iri("EKG_GRAPH_BASE", Some(suffix))?,
            ekg_ontology_base: mandatory_env_var_base_iri("EKG_ONTOLOGY_BASE", Some(suffix))?,
        })
    }
}

pub struct EkgIdentifierContexts {
    pub internal: EkgIdentifierContext,
    pub external: EkgIdentifierContext,
}

impl EkgIdentifierContexts {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            internal: EkgIdentifierContext::from_env("_INTERNAL")?,
            external: EkgIdentifierContext::from_env("_EXTERNAL")?,
        })
    }

    // #[cfg(test)]
    pub fn default_test() {
        std::env::set_var("EKG_BASE_INTERNAL", crate::PLACEHOLDER_BASE_IRI);
        std::env::set_var(
            "EKG_ID_BASE_INTERNAL",
            crate::PLACEHOLDER_ID_BASE_IRI,
        );
        std::env::set_var(
            "EKG_GRAPH_BASE_INTERNAL",
            crate::PLACEHOLDER_GRAPH_BASE_IRI,
        );
        std::env::set_var(
            "EKG_ONTOLOGY_BASE_INTERNAL",
            crate::PLACEHOLDER_ONTOLOGY_BASE_IRI,
        );
        std::env::set_var("EKG_BASE_EXTERNAL", "http://localhost:3000");
        std::env::set_var("EKG_ID_BASE_EXTERNAL", "http://localhost:3000/id");
        std::env::set_var(
            "EKG_GRAPH_BASE_EXTERNAL",
            "http://localhost:3000/graph",
        );
        std::env::set_var(
            "EKG_ONTOLOGY_BASE_EXTERNAL",
            "http://localhost:3000/ontology",
        );
    }
}
