extern crate alloc;

pub use this::Parameters;

mod builder;
mod handle;
#[cfg(test)]
mod tests;
mod this;

const SENSITIVE_PARAMETERS: [&str; 1] = ["license-content"];
