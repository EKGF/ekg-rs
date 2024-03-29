#![allow(missing_docs)]

use crate::Namespace;

pub struct Predicate<'a> {
    pub namespace:  &'a Namespace,
    pub local_name: String,
}

impl<'a> std::fmt::Display for Predicate<'a> {
    // noinspection SpellCheckingInspection
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}{}>", self.namespace.iri, self.local_name)
    }
}

impl<'a> Predicate<'a> {
    pub fn display_turtle<'b>(&'a self) -> impl std::fmt::Display + 'a + 'b
    where 'a: 'b {
        struct TurtlePredicate<'b>(&'b Predicate<'b>);
        impl<'b> std::fmt::Display for TurtlePredicate<'b> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}{}",
                    self.0.namespace.name, self.0.local_name
                )
            }
        }
        TurtlePredicate(self)
    }

    pub fn declare(namespace: &'a Namespace, local_name: &str) -> Self {
        Self { namespace, local_name: local_name.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Namespace, Predicate};

    #[test]
    fn test_predicate() {
        let ns = Namespace::declare(
            "abc:",
            iri_string::types::IriReferenceString::try_from("https://whatever.kg/def/")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .unwrap();
        let prd = Predicate::declare(&ns, "xyz");

        let str_prd = format!("{:}", prd);

        assert_eq!(str_prd.as_str(), "<https://whatever.kg/def/xyz>");

        let str_prd = format!("{}", prd.display_turtle());

        assert_eq!(str_prd.as_str(), "abc:xyz");
    }
}
