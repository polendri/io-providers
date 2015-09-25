//! Providers of environment data, such as the working directory and environment variables.
//!
//! # Examples
//! 
//! ```
//! extern crate io_providers;
//!
//! use std::path::{Path, PathBuf};
//! use io_providers::env;
//! use io_providers::env::Provider;
//!
//! fn path_is_foobar<E: env::Provider>(env: &mut E) -> bool {
//!     let cur_dir = env.current_dir().unwrap();
//!     cur_dir == PathBuf::from("/foo/bar")
//! }
//!
//! fn main() {
//!     test_path_is_foobar();
//!
//!     // Use a local environment provider here to interact with the system environment
//!     path_is_foobar(&mut env::Local::new());
//! }
//!
//! fn test_path_is_foobar() {
//!     // Use a virtual environment provider here to test the functionality of `path_is_foobar()`
//!     let mut env = env::Virtual::new();
//!     env.set_current_dir(Path::new("/nope"));
//!
//!     assert!(!path_is_foobar(&mut env));
//!
//!     env.set_current_dir(Path::new("/foo/bar"));
//!
//!     assert!(path_is_foobar(&mut env));
//! }
//! ```

mod local_provider;
mod virtual_provider;

pub use self::local_provider::Local;
pub use self::virtual_provider::Virtual;

use std::io;
use std::path::{Path, PathBuf};

/// Provides access to environment data, such as working directory and environment variables.
///
/// This trait acts more-or-less as a drop-in replacement for `std::env` functions. The only
/// difference is that `env::Provider::args()` returns `Vec<String>`, and not an iterator like
/// `std::env::args` does.
///
/// Note that, since this trait has generic methods, it is not object safe and thus can't be used
/// as a trait object.
pub trait Provider: {
    /// Returns the arguments which this program was started with (normally passed via the command
    /// line).
    ///
    /// See `std::env::args` for more information.
    fn args(&self) -> Vec<String>;

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
