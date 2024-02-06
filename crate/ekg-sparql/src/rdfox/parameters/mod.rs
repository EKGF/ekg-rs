#![cfg(feature = "_rdfox")]

extern crate alloc;

pub use this::Parameters;
use {
    crate::persistence_mode::PersistenceMode,
    std::fmt::{Display, Formatter},
};

mod builder;
mod handle;
#[cfg(test)]
mod tests;
mod this;

impl Display for PersistenceMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceMode::File => write!(f, "file"),
            PersistenceMode::FileSequence => write!(f, "file-sequence"),
            PersistenceMode::Off => write!(f, "off"),
        }
    }
}

pub enum DataStoreType {
    ParallelNN,
    ParallelNW,
    ParallelWW,
}

const SENSITIVE_PARAMETERS: [&str; 1] = ["license-content"];
