use std;
use std::env;
use std::io;
use std::path::{Path, PathBuf};

use env::Env;

/// Provides inspection and manipulation of the process's environment, using
/// `[std::env](https://doc.rust-lang.org/std/env/)`.
#[derive(Default)]
pub struct NativeEnv;

impl Env for NativeEnv {
    type ArgsIter = env::Args;
    type ArgsOsIter = env::ArgsOs;

    fn args(&self) -> Self::ArgsIter {
        std::env::args()
    }

    fn args_os(&self) -> Self::ArgsOsIter {
        std::env::args_os()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        std::env::set_current_dir(path)
    }
}
