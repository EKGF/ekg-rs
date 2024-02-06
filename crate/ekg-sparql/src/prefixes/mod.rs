pub use {builder::PrefixesBuilder, declare_result::PrefixesDeclareResult, this::Prefixes};

mod builder;
mod declare_result;
#[cfg(feature = "_rdfox")]
mod handle;
mod tests;
mod this;
