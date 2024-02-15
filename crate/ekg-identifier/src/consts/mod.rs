#![allow(missing_docs)]
#![allow(clippy::wildcard_imports)]

pub use {consts::*, namespace_iris::*, namespaces::*, prefixes::*};

mod namespace_iris;
mod namespaces;
mod prefixes;
// noinspection RsExternalLinter
#[allow(clippy::module_inception)]
mod consts;
