//! Defines "provider" traits and implementations for different types of I/O operations.
//!
//! The purpose of this is mainly for dependency injection: by having your code depend on a
//! generic provider, it can be tested by giving it a virtual, inspectable implementation of that
//! provider. In production, the "real" implementation can be used.
//!
//! Each type of provider exists in its own submodule and can be used independently. However,
//! this module also contains the all-encompassing `IoProvider` trait which provides access to
//! all of them. If you have a lot of I/O dependencies, it might be easier to create and pass
//! around one `&mut IoProvider` rather than several different providers.
//!
//! # Examples
//!
//! ```
//! extern crate io_providers;
//!
//! use std::io::Write;
//! use std::path::Path;
//! use io_providers::{IoProvider, LocalIoProvider, VirtualIoProvider};
//! use io_providers::env::Provider as EnvProvider;
//! use io_providers::stream::Provider as StreamProvider;
//!
//! /// Gets the current working directory and prints it to stdout.
//! fn do_work<P: IoProvider>(io: &mut P) {
//!     let cur_dir = io.env().current_dir().unwrap();
//!     let stdout = io.stream().output();
//!     writeln!(stdout, "The current directory is: {}", cur_dir.to_str().unwrap()).unwrap();
//! }
//!
//! fn main() {
//!     test_do_work_prints_current_dir();
//!
//!     // Use a local I/O provider here to get real interaction with the system
//!     let mut io = LocalIoProvider::new();
//!     do_work(&mut io);
//! }
//!
//! fn test_do_work_prints_current_dir() {
//!     // Use a virtual I/O provider here so we can verify how it was used
//!     let mut virtual_io = VirtualIoProvider::new();
//!     virtual_io.env().set_current_dir(Path::new("/foo/bar")).unwrap();
//!
//!     do_work(&mut virtual_io);
//!
//!     assert_eq!(
//!         "The current directory is: /foo/bar\n",
//!         ::std::str::from_utf8(virtual_io.stream().read_output()).unwrap());
//! }
//! ```

pub mod env;
pub mod stream;

/// Provides access to an environment provider and a stream provider.
///
/// See `env::Provider` and `stream::Provider` for more information.
pub trait IoProvider {
    // The type of the environment provider.
    type E: env::Provider;

    // The type of the stream provider.
    type S: stream::Provider;

    /// Gets the `env::Provider`.
    fn env<'a>(&'a mut self) -> &'a mut Self::E;

    /// Gets the `stream::Provider`.
    fn stream<'a>(&'a mut self) -> &'a mut Self::S;
}

/// "Real" implementer of `IoProvider`, using standard streams and the local environment.
///
/// See `env::Local` and `stream::Std` for more information.
pub struct LocalIoProvider {
    env: env::Local,
    stream: stream::Std,
}

impl LocalIoProvider {
    /// Creates a new `LocalIoProvider`.
    pub fn new() -> LocalIoProvider {
        LocalIoProvider {
            env: env::Local,
            stream: stream::Std::new(),
        }
    }
}

impl IoProvider for LocalIoProvider {
    type E = env::Local;
    type S = stream::Std;

    fn env<'a>(&'a mut self) -> &'a mut env::Local {
        &mut self.env
    }

    fn stream<'a>(&'a mut self) -> &'a mut stream::Std {
        &mut self.stream
    }
}

/// Virtual implementer of `IoProvider`, using in-memory data which can be inspected.
///
/// See `env::Virtual` and `stream::Virtual` for more information.
pub struct VirtualIoProvider {
    env: env::Virtual,
    stream: stream::Virtual,
}

impl VirtualIoProvider {
    /// Creates a new `VirtualIoProvider`.
    pub fn new() -> VirtualIoProvider {
        VirtualIoProvider {
            env: env::Virtual::new(),
            stream: stream::Virtual::new(),
        }
    }
}

impl IoProvider for VirtualIoProvider {
    type E = env::Virtual;
    type S = stream::Virtual;

    fn env<'a>(&'a mut self) -> &'a mut env::Virtual {
        &mut self.env
    }

    fn stream<'a>(&'a mut self) -> &'a mut stream::Virtual {
        &mut self.stream
    }
}
