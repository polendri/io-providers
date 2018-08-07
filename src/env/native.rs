use std;
use std::env;
use std::ffi;
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
    type VarsIter = env::Vars;
    type VarsOsIter = env::VarsOs;

    fn args(&self) -> Self::ArgsIter {
        std::env::args()
    }

    fn args_os(&self) -> Self::ArgsOsIter {
        std::env::args_os()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    fn current_exe(&self) -> io::Result<PathBuf> {
        std::env::current_exe()
    }

    fn home_dir(&self) -> Option<PathBuf> {
        #[allow(deprecated)]
        std::env::home_dir()
    }

    fn remove_var<K: AsRef<ffi::OsStr>>(&mut self, k: K) {
        std::env::remove_var(k)
    }

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        std::env::set_current_dir(path)
    }

    fn set_var<K: AsRef<ffi::OsStr>, V: AsRef<ffi::OsStr>>(&mut self, k: K, v: V) {
        std::env::set_var(k, v)
    }

    fn var<K: AsRef<ffi::OsStr>>(&self, key: K) -> Result<String, env::VarError> {
        std::env::var(key)
    }

    fn var_os<K: AsRef<ffi::OsStr>>(&self, key: K) -> Option<ffi::OsString> {
        std::env::var_os(key)
    }

    fn vars(&self) -> Self::VarsIter {
        std::env::vars()
    }

    fn vars_os(&self) -> Self::VarsOsIter {
        std::env::vars_os()
    }
}
