mod local_provider;
mod virtual_provider;

pub use self::local_provider::Local;
pub use self::virtual_provider::Virtual;

use std::io;
use std::path::{Path, PathBuf};

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
