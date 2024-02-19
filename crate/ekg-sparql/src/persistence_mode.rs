use std::fmt::{Display, Formatter};

pub enum PersistenceMode {
    File,
    FileSequence,
    Off,
}

impl Display for PersistenceMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_str()) }
}

impl PersistenceMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            PersistenceMode::File => "file",
            PersistenceMode::FileSequence => "file-sequence",
            PersistenceMode::Off => "off",
        }
    }
}
