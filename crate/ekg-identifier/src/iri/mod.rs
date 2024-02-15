pub use {
    a_box_base::ABoxNamespaceIRI,
    env::{mandatory_env_var_base_iri, mandatory_env_var_uri},
    namespace_iri::NamespaceIRI,
    owned::OwnedIRI,
    t_box_base::TBoxNamespaceIRI,
};

mod a_box_base;
mod env;
mod namespace_iri;
mod owned;
mod t_box_base;
