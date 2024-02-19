use {crate::ABoxNamespaceIRI, ekg_error::Error, ekg_util::env::mandatory_env_var};

pub fn mandatory_env_var_uri(
    name: &str,
    suffix: Option<&'static str>,
) -> Result<iri_string::types::IriReferenceString, Error> {
    let val = mandatory_env_var(name, suffix)?;
    iri_string::types::IriReferenceString::try_from(val)
        .map_err(|_e| Error::MandatoryEnvironmentVariableIsNotIRI(name.to_string()))
}

pub fn mandatory_env_var_base_iri(
    name: &str,
    suffix: Option<&'static str>,
) -> Result<ABoxNamespaceIRI, Error> {
    let val = mandatory_env_var(name, suffix)?;
    Ok(ABoxNamespaceIRI(
        iri_string::types::IriReferenceString::try_from(val)
            .map_err(|_e| Error::MandatoryEnvironmentVariableIsNotIRI(name.to_string()))?,
    ))
}
