use crate::{Literal, Namespace};

/// The `Class` struct represents an RDFS or OWL class identifier
/// consisting of a [`Namespace`] (i.e. a namespace) and a "local name".
#[derive(Debug, Clone)]
pub struct Class {
    pub namespace:  Namespace,
    pub local_name: String,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.namespace.name.as_str(),
            self.local_name.as_str()
        )
    }
}

impl Class {
    pub fn declare(namespace: Namespace, local_name: &str) -> Self {
        Self { namespace, local_name: local_name.to_string() }
    }

    pub fn as_iri(&self) -> Result<fluent_uri::Uri<String>, ekg_error::Error> {
        let iri = fluent_uri::Uri::parse_from(format!(
            "{}{}",
            self.namespace.iri, self.local_name
        ))
        .map_err(|(_s, e)| ekg_error::Error::from(e))?;
        Ok(iri)
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn display_turtle<'a>(&'a self) -> impl std::fmt::Display + 'a {
        struct TurtleClass<'a>(&'a Class);
        impl<'a> std::fmt::Display for TurtleClass<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}{}",
                    self.0.namespace.name, self.0.local_name
                )
            }
        }
        TurtleClass(self)
    }

    pub fn plural_label(&self) -> String { format!("{}s", self.local_name) }

    // TODO: Make this slightly smarter

    pub fn is_literal(&self, literal: &Literal) -> bool {
        if let Some(that_iri) = literal.as_iri_ref() {
            if let Ok(this_iri) = self.as_iri() {
                that_iri.as_str() == this_iri.borrow().as_str()
            } else {
                let iri = self.to_string();
                literal.to_string() == iri
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::Namespace,
        crate::{class::Class, DataType, Literal},
    };

    #[test]
    fn test_a_class_01() {
        let namespace = Namespace::declare(
            "test:",
            &fluent_uri::Uri::from_static("https://whatever.com/test#"),
        );
        let class = Class::declare(namespace, "SomeClass");
        let s = format!("{:}", class);
        assert_eq!(s, "test:SomeClass")
    }

    #[test]
    fn test_a_class_02() {
        let namespace = Namespace::declare(
            "test:",
            &fluent_uri::Uri::from_static("https://whatever.com/test#"),
        );
        let class = Class::declare(namespace, "SomeClass");
        let s = format!("{}", class.as_iri().unwrap());
        assert_eq!(s, "https://whatever.com/test#SomeClass");
    }

    #[test]
    fn test_is_literal() {
        let namespace = Namespace::declare(
            "test:",
            &fluent_uri::Uri::from_static("https://whatever.com/test#"),
        );
        let class = Class::declare(namespace, "SomeClass");
        let literal = Literal::from_type_and_buffer(
            DataType::AnyUri,
            "https://whatever.com/test#SomeClass",
            None,
        )
        .unwrap();
        assert!(literal.is_some());
        assert_eq!(
            class.as_iri().unwrap().to_string().as_str(),
            "https://whatever.com/test#SomeClass"
        );
        assert!(class.is_literal(&literal.unwrap()))
    }
}
