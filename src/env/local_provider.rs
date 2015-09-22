use std;
use std::io;
use std::path::{Path, PathBuf};
use env;

pub struct Local;

impl env::Provider for Local {
    fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        std::env::set_current_dir(path)
    }
}
