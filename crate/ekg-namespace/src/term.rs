use {
    crate::{DataType, Literal},
    std::str::FromStr,
};

/// An RDF Term is either an IRI, a literal or a blank node.
///
/// See <https://www.w3.org/TR/rdf11-concepts/#section-triples>
#[derive(Debug)]
pub enum Term {
    Iri(Literal),
    Literal(Literal),
    BlankNode(Literal),
}

const ACCEPTABLE_IRI_PROTOCOLS: [&str; 3] = ["http://", "https://", "s3://"];

impl Term {
    pub fn from_static(iri_str: &'static str) -> Result<Self, ekg_error::Error> {
        Self::new_iri_from_str(iri_str)
    }

    pub fn new_iri(iri: &fluent_uri::Uri<&str>) -> Result<Self, ekg_error::Error> {
        for acceptable_protocol in ACCEPTABLE_IRI_PROTOCOLS.iter() {
            if iri.as_str().starts_with(acceptable_protocol) {
                return Ok(Term::Iri(Literal::from_iri(iri)?));
            }
        }
        Err(ekg_error::Error::InvalidIri(
            iri.as_str().to_string(),
        ))
    }

    pub fn new_iri_from_str(iri_str: &str) -> Result<Self, ekg_error::Error> {
        Term::new_iri(&fluent_uri::Uri::parse(iri_str)?)
    }

    pub fn new_str(str: &str) -> Result<Self, ekg_error::Error> {
        Ok(Term::Literal(Literal::from_str(str)?))
    }

    pub fn new_blank_node(str: &str) -> Result<Self, ekg_error::Error> {
        Ok(Term::BlankNode(
            Literal::from_type_and_buffer(DataType::BlankNode, str, None)?.unwrap(),
        ))
    }

    /// Display a [`Term`] in human-readable format.
    ///
    /// ```no_run
    /// use ekg_namespace::Term;
    ///
    /// let term = Term::new_iri(&fluent_uri::Uri::parse("https://whatever.url").unwrap()).unwrap();
    /// let turtle = format!("{}", term.display_turtle());
    ///
    /// assert_eq!(turtle, "<https://whatever.url>");
    /// ```
    pub fn display_turtle<'a, 'b>(&'a self) -> impl std::fmt::Display + 'a + 'b
    where 'a: 'b {
        struct TurtleTerm<'b>(&'b Term);
        impl<'b> std::fmt::Display for TurtleTerm<'b> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let value = match self.0 {
                    Term::Iri(value) => value,
                    Term::Literal(value) => value,
                    Term::BlankNode(value) => value,
                };
                value.display_turtle().fmt(f)
            }
        }
        TurtleTerm(self)
    }
}

impl FromStr for Term {
    type Err = ekg_error::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> { Term::new_str(str) }
}

impl From<Literal> for Term {
    fn from(value: Literal) -> Self { value.as_term() }
}

#[cfg(test)]
mod tests {
    #[test_log::test]
    fn test_term_01() {
        let uri = fluent_uri::Uri::parse("https://whatever.url/").unwrap();
        assert_eq!(uri.to_string(), "https://whatever.url/");
        assert_eq!(uri.path().as_str(), "/");
        let term = crate::Term::new_iri(&uri).unwrap();
        let turtle = term.display_turtle().to_string();

        assert_eq!(turtle, "<https://whatever.url/>");
    }

    #[test_log::test]
    fn test_term_02() {
        let term = crate::Term::new_iri(
            &fluent_uri::Uri::parse("unknown-protocol://whatever.url").unwrap(),
        );
        assert!(term.is_err());
        assert!(matches!(
            term.unwrap_err(),
            ekg_error::Error::InvalidIri(_)
        ));
    }

    #[test_log::test]
    fn test_term_03() {
        // We are not accepting wrongly formatted IRIs
        let term = crate::Term::from_static("https:/x/whatever.url");
        assert!(term.is_err());
        assert!(matches!(
            term.unwrap_err(),
            ekg_error::Error::InvalidIri(_)
        ));
    }

    #[test_log::test]
    fn test_term_04() {
        let term = crate::Term::new_str("some string").unwrap();

        let turtle = format!("{}", term.display_turtle());

        assert_eq!(turtle, "\"some string\"");
    }

    #[test_log::test]
    fn test_term_05() -> Result<(), ekg_error::Error> {
        let term: crate::Term = "some string".parse()?;

        let turtle = format!("{}", term.display_turtle());

        assert_eq!(turtle, "\"some string\"");

        Ok(())
    }

    #[test_log::test]
    fn test_term_06() -> Result<(), ekg_error::Error> {
        let term: crate::Term = "\"some string\"^^xsd:string".parse()?;

        let turtle = format!("{}", term.display_turtle());

        assert_eq!(turtle, "\"\"some string\"^^xsd:string\""); // TODO: This is incorrect, recognise the XSD data type suffix and process it

        Ok(())
    }

    #[test_log::test]
    fn test_fluent_uri_01() -> Result<(), ekg_error::Error> {
        let uri =
            fluent_uri::Uri::parse("https://placeholder.kg/ontology/abc#xyz").map_err(|e| {
                println!("{}", e);
                ekg_error::Error::Unknown
            })?;

        assert_eq!(
            uri.to_string().as_str(),
            "https://placeholder.kg/ontology/abc#xyz"
        );
        Ok(())
    }

    #[test_log::test]
    fn test_fluent_uri_02() -> Result<(), ekg_error::Error> {
        let uri = fluent_uri::Uri::parse("https://placeholder.kg/ontology/abc#").map_err(|e| {
            println!("{}", e);
            ekg_error::Error::Unknown
        })?;

        assert_eq!(
            uri.to_string().as_str(),
            "https://placeholder.kg/ontology/abc#"
        );
        Ok(())
    }

    #[test_log::test]
    fn test_fluent_uri_03() -> Result<(), ekg_error::Error> {
        let uri = fluent_uri::Uri::parse("https://placeholder.kg/ontology/abc/").map_err(|e| {
            println!("{}", e);
            ekg_error::Error::Unknown
        })?;

        assert_eq!(
            uri.to_string().as_str(),
            "https://placeholder.kg/ontology/abc/"
        );
        Ok(())
    }
}
