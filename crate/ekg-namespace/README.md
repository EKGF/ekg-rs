# ekg-namespace

This crate provides a set of constants and basic functions and classes for
Enterprise Knowledge Graph (EKG) projects.

## ADRs

### Fluent-uri

We're using the [fluent-uri](https://crates.io/crates/fluent-uri) crate for all
IRI/URI/URL related functionality, after having tried many other crates such
as hyper (their Uri type), iref and iri-string, each of which has its own issues,
for instance, hyper's Uri does not allow a hash or slash at the end of the path
which is a problem for RDF IRIs such as `https://placeholder.kg/ontology/abc#`.
