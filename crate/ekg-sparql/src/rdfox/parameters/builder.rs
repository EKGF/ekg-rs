use {
    super::Parameters,
    crate::{FactDomain, PersistenceMode},
    std::default::Default,
};

#[derive(Default)]
pub struct ParametersBuilder {
    fact_domain:       Option<FactDomain>,
    persist_datastore: Option<PersistenceMode>,
}

impl ParametersBuilder {
    pub fn default_builder() -> Self { Self::default() }

    pub fn fact_domain(&mut self, fact_domain: FactDomain) -> &mut Self {
        self.fact_domain = Some(fact_domain);
        self
    }

    pub fn persist_datastore(&mut self, mode: PersistenceMode) -> &mut Self {
        self.persist_datastore = Some(mode);
        self
    }

    pub fn build(&self) -> Result<Parameters, ekg_error::Error> {
        let mut to_build = Parameters::empty()?;
        if let Some(fact_domain) = &self.fact_domain {
            to_build.fact_domain(fact_domain)?;
        }
        if let Some(persist_datastore) = &self.persist_datastore {
            to_build.persist_datastore(persist_datastore)?;
        }
        Ok(to_build)
    }
}
