//! Providers of environment data, such as the working directory and environment variables.
//! 
//! TODO example

mod local_provider;
mod virtual_provider;

pub use self::local_provider::Local;
pub use self::virtual_provider::Virtual;

use std::io;
use std::path::{Path, PathBuf};

/// Provides access to environment data, such as working directory and environment variables.
///
/// This trait acts more-or-less as a drop-in replacement for `std::env` functions. The only
/// real difference is that all of the path parameters on `env::Provider` methods require
/// `&Path` as opposed to `AsRef<Path>`. In order to allow trait objects, `env::Provider` must
/// have object safety, and this requires non-generic methods (among other things). As such, we
/// have to put up with the less ergonomic `&Path`.
pub trait Provider: {
    /// Returns the current working directory as a `PathBuf`.
    ///
    /// See `std::env::current_dir` for more information.
    fn current_dir(&self) -> io::Result<PathBuf>;

    /// Changes the current working directory to the specified path, returning whether the change
    /// was completed successfully or not.
    ///
    /// See `std::env::set_current_dir` for more information.
    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;
}
