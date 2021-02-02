use crate::PleasantResult;
use rusqlite::Connection;
use std::fmt::Formatter;
use std::path::Path;

pub struct CredentialsDatabase {}

pub enum DatabasePath {
    File(Path),
    InMem,
}

impl std::fmt::Display for DatabasePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabasePath::File(path) => write!(f, "Path: {}",  path.display());
            DatabasePath::InMem => write!(f, "In Memory (changes will be lost)"),
        }
    }
}

impl CredentialsDatabase {
    pub fn new(path: DatabasePath) -> PleasantResult<Self> {
        unimplemented!()
    }
}
