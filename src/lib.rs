//! Defines "provider" traits and implementations for different types of I/O operations.
//!
//! The purpose of this is mainly for dependency injection: by having your code depend on a
//! generic provider, it can be tested by giving it a virtual, inspectable implementation of that
//! provider. In production, the "real" implementation can be used.
//!
//! Each type of provider exists in its own submodule and can be used independently. However,
//! this module also contains the all-encompassing `IoProvider` trait which provides access to
//! all of them. If you have a lot of I/O dependencies, it might be easier to create and pass
//! around one `&IoProvider` rather than several different providers.
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
//! fn do_work(io: &mut IoProvider) {
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
//!         ::std::str::from_utf8(virtual_io.virtual_stream().read_output()).unwrap());
//! }
//! ```

pub mod env;
pub mod stream;

/// Provides access to an environment provider and a stream provider.
pub trait IoProvider {
    /// Gets the `env::Provider`.
    fn env<'a>(&'a mut self) -> &'a mut env::Provider;

    /// Gets the `stream::Provider`.
    fn stream<'a>(&'a mut self) -> &'a mut stream::Provider;
}

/// "Real" implementer of `IoProvider`, using standard streams and the local environment.
pub struct LocalIoProvider {
    env_provider: env::Local,
    stream_provider: stream::Std,
}

impl LocalIoProvider {
    /// Creates a new `LocalIoProvider`.
    pub fn new() -> LocalIoProvider {
        LocalIoProvider {
            env_provider: env::Local,
            stream_provider: stream::Std::new(),
        }
    }
}

impl IoProvider for LocalIoProvider {
    fn env<'a>(&'a mut self) -> &'a mut env::Provider {
        &mut self.env_provider
    }

    fn stream<'a>(&'a mut self) -> &'a mut stream::Provider {
        &mut self.stream_provider
    }
}

/// Virtual implementer of `IoProvider`, using in-memory data which can be inspected.
pub struct VirtualIoProvider {
    env_provider: env::Virtual,
    stream_provider: stream::Virtual,
}

impl VirtualIoProvider {
    /// Creates a new `VirtualIoProvider`.
    pub fn new() -> VirtualIoProvider {
        VirtualIoProvider {
            env_provider: env::Virtual::new(),
            stream_provider: stream::Virtual::new(),
        }
    }

    /// Gets the `env::Virtual` provider.
    pub fn virtual_env<'a>(&'a mut self) -> &'a mut env::Virtual {
        &mut self.env_provider
    }

    /// Gets the `stream::Virtual` provider.
    pub fn virtual_stream<'a>(&'a mut self) -> &'a mut stream::Virtual {
        &mut self.stream_provider
    }
}

impl IoProvider for VirtualIoProvider {
    fn env<'a>(&'a mut self) -> &'a mut env::Provider {
        &mut self.env_provider
    }

    fn stream<'a>(&'a mut self) -> &'a mut stream::Provider {
        &mut self.stream_provider
    }
}
