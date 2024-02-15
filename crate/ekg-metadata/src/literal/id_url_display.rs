use {
    crate::literal::this::Literal,
    ekg_identifier::NamespaceIRI,
    std::fmt::{Display, Formatter},
};

pub struct LiteralIdUrlDisplay<'a, T: NamespaceIRI> {
    pub(crate) literal:     &'a Literal,
    pub(crate) id_base_iri: &'a T,
}

impl<'a, T: NamespaceIRI> Display for LiteralIdUrlDisplay<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(literal_str) = self.literal.as_str() {
            if self.id_base_iri.is_in_namespace(literal_str) {
                write!(
                    f,
                    "{}",
                    self.literal
                        .as_id(self.id_base_iri)
                        .map_err(|_| std::fmt::Error)?
                )
            } else {
                write!(f, "{:}", literal_str)
            }
        } else {
            write!(
                f,
                "ERROR: literal is not a string {:?}",
                self.literal
            )?;
            Err(std::fmt::Error)
        }
    }
}

#[cfg(test)]
mod tests {
    use {ekg_identifier::ABoxNamespaceIRI, std::str::FromStr};

    #[test]
    fn test_01() {
        let ns_iri = ABoxNamespaceIRI::from_str("https://placeholder.kg/id/").unwrap();
        let literal = super::Literal::new_iri_with_datatype(
            &fluent_uri::Uri::parse("https://placeholder.kg/id/123").unwrap(),
            crate::DataType::AnyUri,
        )
        .unwrap();

        let x = literal.as_id_url_display(&ns_iri);

        assert_eq!(format!("{}", x), "123");
    }
}
