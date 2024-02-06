use {
    crate::prefixes::this::Prefixes,
    ekg_namespace::{Namespace, PREFIX_OWL, PREFIX_RDF, PREFIX_RDFS, PREFIX_XSD},
    std::ops::Deref,
};

#[derive(Default, Clone)]
pub struct PrefixesBuilder {
    prefixes: Vec<Namespace>,
}

impl PrefixesBuilder {
    pub fn default_builder() -> Self { PrefixesBuilder { prefixes: Vec::new() } }

    pub fn declare_with_name_and_iri(
        mut self,
        name: &str,
        iri: &fluent_uri::Uri<&str>,
    ) -> Result<Self, ekg_error::Error> {
        self.prefixes.push(Namespace::declare(name, iri)?);
        Ok(self)
    }

    pub fn declare(mut self, namespace: &Namespace) -> Self {
        self.prefixes.push(namespace.clone());
        self
    }

    /// Return the default namespaces: `RDF`, `RDFS`, `OWL` and `XSD`
    pub fn default_namespaces(self) -> Self {
        tracing::trace!("Declaring default namespaces");
        self.declare(PREFIX_RDF.deref())
            .declare(PREFIX_RDFS.deref())
            .declare(PREFIX_OWL.deref())
            .declare(PREFIX_XSD.deref())
    }

    pub fn declare_namespaces(mut self, namespaces: &Vec<Namespace>) -> Self {
        self.prefixes.append(&mut namespaces.clone());
        self
    }

    pub fn build(self) -> Result<Prefixes, ekg_error::Error> {
        let mut to_build = Prefixes::empty()?;
        for namespace in self.prefixes.iter() {
            to_build.declare_namespace(namespace)?;
        }
        Ok(to_build)
    }
}
