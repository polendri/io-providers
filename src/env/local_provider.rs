use std;
use std::io;
use std::path::{Path, PathBuf};
use env;

/// Provides access to the local environment (e.g. what the corresponding `std::env` functions
/// would access).
pub struct Local;

impl Local {
    /// Creates a new local environment provider.
    pub fn new() -> Local {
        Local
    }
}

impl env::Provider for Local {
    fn args(&self) -> Vec<String> {
        std::env::args().collect()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    fn set_current_dir(&mut self, path: &Path) -> io::Result<()> {
        std::env::set_current_dir(path)
    }
}
