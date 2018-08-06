//! Defines traits and implementations for the inspection and manipulation of the process's
//! environment.

mod native;
mod simulated;

pub use self::native::NativeEnv;
pub use self::simulated::SimulatedEnv;

use std::io;
use std::path::{Path, PathBuf};

/// Provides inspection and manipulation of the process's environment.
///
/// This roughly corresponds to `[std::env](https://doc.rust-lang.org/std/env/)`.
///
/// # Examples
///
/// ```
/// extern crate io_providers;
///
/// use std::path::{Path, PathBuf};
/// use io_providers::{Env, NativeEnv, SimulatedEnv};
///
/// /// Uses `Env` to check if the currect working directory is "/foo/bar"
/// fn curdir_is_foobar<E: Env>(env: &mut E) -> bool {
///     let cur_dir = env.current_dir().unwrap();
///     cur_dir == PathBuf::from("/foo/bar")
/// }
///
/// fn main() {
///     // By creating a fake `Env` and set its current working directory, we can use it to test
///     // the behaviour of `curdir_is_foobar()`.
///     let mut env = SimulatedEnv::new();
///     env.set_current_dir(Path::new("/nope"));
///
///     // Test that our function returns false with a current working directory of "/nope"
///     assert!(!curdir_is_foobar(&mut env));
///
///     // Now set the fake working directory to "/foo/bar" and confirm that our function returns
///     // `true`
///     env.set_current_dir(Path::new("/foo/bar"));
///     assert!(curdir_is_foobar(&mut env));
///
///     // To use the real system environment, we use a `NativeEnv` instead
///     assert!(!curdir_is_foobar(&mut NativeEnv));
/// }
/// ```
pub trait Env {
    /// Returns the arguments which this program was started with (normally passed via the command
    /// line).
    ///
    /// See `[std::env::args](https://doc.rust-lang.org/std/env/fn.args.html)` for more information.
    fn args(&self) -> Vec<String>;

    /// Returns the current working directory as a `PathBuf`.
    ///
    /// See `[std::env::current_dir](https://doc.rust-lang.org/std/env/fn.current_dir.html)` for more information.
    fn current_dir(&self) -> io::Result<PathBuf>;

    /// Changes the current working directory to the specified path, returning whether the change
    /// was completed successfully or not.
    ///
    /// See `[std::env::set_current_dir](https://doc.rust-lang.org/std/env/fn.set_current_dir.html)` for more information.
    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;
}
