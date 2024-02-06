# test data

Some RDF files to test the RDFox feature of the `ekg-sparql` crate with.

## How to run the tests

```shell
RUST_LOG=info cargo test 
```

Or, if you want to see all output:

```shell
RUST_LOG=trace cargo test --manifest-path crate/ekg-sparql/Cargo.toml --test rdfox_tests --features rdfox-dylib,fs  -- --exact --nocapture
```

If you want to run the tests with the dynamic link library of RDFox, then run this:

```shell
RUST_LOG=trace cargo test --package rdfox-rs --features rdfox-dylib --test load load_rdfox -- --exact --nocapture
```
