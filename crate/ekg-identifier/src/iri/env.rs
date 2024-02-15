use {crate::ABoxNamespaceIRI, ekg_error::Error, ekg_util::env::mandatory_env_var};

pub fn mandatory_env_var_uri(
    name: &str,
    suffix: Option<&'static str>,
) -> Result<fluent_uri::Uri<String>, Error> {
    let val = mandatory_env_var(name, suffix)?;
    fluent_uri::Uri::parse_from(val)
        .map_err(|e| Error::MandatoryEnvironmentVariableIsNotIRI(e.0.to_string()))
}

pub fn mandatory_env_var_base_iri(
    name: &str,
    suffix: Option<&'static str>,
) -> Result<ABoxNamespaceIRI, Error> {
    let val = mandatory_env_var(name, suffix)?;
    Ok(ABoxNamespaceIRI(
        fluent_uri::Uri::parse_from(val)
            .map_err(|e| Error::MandatoryEnvironmentVariableIsNotIRI(e.0.to_string()))?,
    ))
}
