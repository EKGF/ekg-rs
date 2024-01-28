/// A `Namespace` represents a namespace IRI that can also be shown
/// in abbreviated format, also known as "prefix".
///
/// For instance, the namespace IRI <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
/// can also be shown (in [RDF Turtle](https://www.w3.org/TR/turtle/#prefixed-name)
/// or SPARQL for instance) as `rdf:`.
/// A "local name" such as "type" in such a namespace would look
/// like <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> or like `rdf:type`.
#[derive(Debug, Clone)]
pub struct Namespace {
    /// assumed to end with ':'
    pub name: String,
    /// assumed to end with either '/' or '#'
    pub iri:  fluent_uri::Uri<String>,
}

impl Eq for Namespace {}

impl PartialEq for Namespace {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.iri.as_str() == other.iri.as_str()
    }
}

impl std::fmt::Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name.as_str(), self.iri)
    }
}

impl Namespace {
    pub fn declare(name: &str, iri: &fluent_uri::Uri<&str>) -> Result<Self, ekg_error::Error> {
        match iri.to_string().chars().last() {
            Some('/') | Some('#') => Ok(Self { name: name.to_string(), iri: iri.to_owned() }),
            _ => {
                tracing::error!("{} does not end with either / or #", iri);
                Err(ekg_error::Error::IncorrectBaseIRI { iri: iri.to_string() })
            },
        }
    }

    pub fn declare_from_str(name: &str, iri: &str) -> Result<Self, ekg_error::Error> {
        Self::declare(name, &fluent_uri::Uri::parse(iri)?)
    }

    /// Return an identifier based on the current namespace IRI and the given
    /// local name within that namespace.
    pub fn with_local_name(&self, name: &str) -> Result<fluent_uri::Uri<String>, ekg_error::Error> {
        let iri_str = match self.iri.to_string().chars().last().unwrap() as char {
            '/' | '#' => format!("{}{name}", self.iri),
            _ => {
                panic!("{} does not end with either / or #", self.iri)
            },
        };

        Ok(fluent_uri::Uri::parse_from(iri_str).map_err(|(_s, e)| e)?)
    }

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    pub fn as_rdftk_iri_ref(&self) -> Result<rdftk_iri::IRIRef, rdftk_iri::error::Error> {
        Ok(rdftk_iri::IRIRef::new(self.as_rdftk_iri()?))
    }

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    pub fn as_rdftk_iri(&self) -> Result<rdftk_iri::IRI, rdftk_iri::error::Error> {
        use std::str::FromStr;
        rdftk_iri::IRI::from_str(self.iri.as_str())
    }

    pub fn as_sparql_prefix(&self) -> String { format!("PREFIX {} <{}>", self.name, self.iri) }

    pub fn as_turtle_prefix(&self) -> String { format!("@prefix {} <{}> .", self.name, self.iri) }
}

#[cfg(test)]
mod tests {
    #[test_log::test]
    fn test_a_prefix() -> Result<(), ekg_error::Error> {
        let namespace = crate::Namespace::declare(
            "test:",
            &fluent_uri::Uri::from_static("http://whatever.kom/test#"),
        );
        let x = namespace.with_local_name("abc")?;

        assert_eq!(
            x.to_string().as_str(),
            "http://whatever.kom/test#abc"
        );
        Ok(())
    }

    #[test_log::test]
    fn test_b_prefix() -> Result<(), ekg_error::Error> {
        let namespace = crate::Namespace::declare(
            "test:",
            &fluent_uri::Uri::from_static("http://whatever.kom/test/"),
        );
        let x = namespace.with_local_name("abc")?;

        assert_eq!(
            x.to_string().as_str(),
            "http://whatever.kom/test/abc"
        );
        Ok(())
    }
}
