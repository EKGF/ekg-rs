pub use {builder::PrefixesBuilder, declare_result::PrefixesDeclareResult};
use {
    ekg_namespace::{Class, Namespace, Predicate, PREFIX_OWL, PREFIX_RDF, PREFIX_RDFS, PREFIX_XSD},
    std::{
        fmt::{Display, Formatter},
        ops::Deref,
    },
};

mod builder;
mod declare_result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prefixes {
    pub(crate) prefixes: Vec<Namespace>,
}

impl Display for Prefixes {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for namespace in self.prefixes.iter() {
            writeln!(f, "{}", namespace.as_sparql_prefix())?;
        }
        Ok(())
    }
}

impl From<Vec<Namespace>> for Prefixes {
    fn from(prefixes: Vec<Namespace>) -> Self { Self { prefixes } }
}

impl From<Vec<&Namespace>> for Prefixes {
    fn from(prefixes: Vec<&Namespace>) -> Self {
        Self {
            prefixes: prefixes.into_iter().map(|ns| ns.clone()).collect(),
        }
    }
}

impl Prefixes {
    pub fn builder() -> PrefixesBuilder { PrefixesBuilder::default() }

    pub fn empty() -> Result<Self, ekg_error::Error> { Ok(Self { prefixes: vec![] }) }

    /// Return the default namespaces: `RDF`, `RDFS`, `OWL` and `XSD`
    pub fn try_default() -> Result<Self, ekg_error::Error> {
        Self::empty()?
            .add_namespace(PREFIX_RDF.deref())?
            .add_namespace(PREFIX_RDFS.deref())?
            .add_namespace(PREFIX_OWL.deref())?
            .add_namespace(PREFIX_XSD.deref())
    }

    pub fn declare_namespace(
        &mut self,
        namespace: &Namespace,
    ) -> Result<PrefixesDeclareResult, ekg_error::Error> {
        tracing::trace!("Register prefix {namespace}");
        for namespace in &self.prefixes {
            if namespace.name == namespace.name {
                return Ok(PrefixesDeclareResult::PREFIXES_NO_CHANGE)
            }
        }
        tracing::info!(
            "Declaring prefix {} for namespace {}",
            namespace.name.as_str(),
            namespace.iri
        );
        self.prefixes.push(namespace.clone());
        Ok(PrefixesDeclareResult::PREFIXES_DECLARED_NEW)
    }

    pub fn declare(
        &mut self,
        name: &str,
        iri: &fluent_uri::Uri<&str>,
    ) -> Result<PrefixesDeclareResult, ekg_error::Error> {
        self.declare_namespace(&Namespace::declare(name, iri)?)
    }

    pub fn add_namespace(&mut self, namespace: &Namespace) -> Result<Self, ekg_error::Error> {
        let _ = self.declare_namespace(namespace);
        Ok(self.clone())
    }

    pub fn add_class(&mut self, clazz: &Class) -> Result<Self, ekg_error::Error> {
        self.add_namespace(&clazz.namespace)
    }

    pub fn add_predicate(&mut self, predicate: &Predicate) -> Result<Self, ekg_error::Error> {
        self.add_namespace(predicate.namespace)
    }

    pub fn for_each_namespace_do<F: FnMut(&str, &Namespace) -> Result<(), E>, E>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        for namespace in self.prefixes.iter() {
            f(namespace.name.as_str(), namespace)?;
        }
        Ok(())
    }
}
