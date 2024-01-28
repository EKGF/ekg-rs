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

impl Term {
    pub fn from_static(iri_str: &'static str) -> Result<Self, ekg_error::Error> {
        Self::new_iri_from_str(iri_str)
    }

    pub fn new_iri(iri: &fluent_uri::Uri<&str>) -> Result<Self, ekg_error::Error> {
        Ok(Term::Iri(Literal::from_iri(iri)?))
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

    /// Display a [`Term`] in human readable format.
    ///
    /// ```rust
    /// use ekg_namespace::Term;
    ///
    /// let term = Term::new_iri(&fluent_uri::Uri::from_static(
    ///     "https://whatever.url",
    /// ))
    /// .unwrap();
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
        let uri = fluent_uri::Uri::from_static("https://whatever.url");
        // Unfortunately, the fluent_uri::Uri displays itself with a trailing slash
        // which is not what we want for an RDF resource identifier
        assert_eq!(format!("{:#?}", uri), "https://whatever.url/");
        assert_eq!(uri.path(), "/");
        // So that's one more reason to wrap it into a Term
        let term = crate::Term::new_iri(&uri).unwrap();

        let turtle = format!("{}", term.display_turtle());

        assert_eq!(turtle, "<https://whatever.url>");
    }

    #[test_log::test]
    fn test_term_02() {
        let term = crate::Term::new_iri(&fluent_uri::Uri::from_static(
            "unknown-protocol://whatever.url",
        ))
        .unwrap();

        let turtle = format!("{}", term.display_turtle());

        assert_eq!(turtle, "<unknown-protocol://whatever.url>");
    }

    #[test_log::test]
    fn test_term_03() {
        // We are not accepting wrongly formatted IRIs
        let term = crate::Term::from_static("https:/x/whatever.url");
        assert!(term.is_err());
        assert!(matches!(
            term.unwrap_err(),
            ekg_error::Error::InvalidUri(_)
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
