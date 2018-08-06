use std;
use std::io;
use std::path::{Path, PathBuf};
use env::Env;

/// Provides inspection and manipulation of the process's environment, using
/// `[std::env](https://doc.rust-lang.org/std/env/)`.
pub struct NativeEnv;

impl Env for NativeEnv {
    fn args(&self) -> Vec<String> {
        std::env::args().collect()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        std::env::set_current_dir(path)
    }
}
