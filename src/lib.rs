//! Defines "provider" traits and implementations for different types of I/O operations, enabling
//! thorough testing via dependency injection.
//!
//! Currently implemented providers are:
//!
//! * Process environment (variables, working directy, etc) via `Env`
//! * Standard streams (stdin, stdout, stderr) via `StdStreams`
//!
//! Each provider exists in its own submodule and can be used independently. However, this module
//! also contains the all-encompassing `IoProvider` trait which provides access to all of them. If
//! you have a variety of I/O dependencies, it might be easier to create and pass around a single
//! `&mut IoProvider` rather than passing around several of them.
//!
//! # Examples
//!
//! ```
//! extern crate io_providers;
//!
//! use std::io::Write;
//! use std::path::Path;
//! use io_providers::{Env, Io, NativeIo, SimulatedIo, StdStreams};
//!
//! /// Gets the current working directory and prints it to stdout.
//! fn do_work<I: Io>(io: &mut I) {
//!     let cur_dir = io.env().current_dir().unwrap();
//!     let stdout = io.stream().output();
//!     writeln!(stdout, "The current directory is: {}", cur_dir.to_str().unwrap()).unwrap();
//! }
//!
//! fn main() {
//!     // Test `do_work()` using a simulated I/O environment
//!     let mut simulated_io = SimulatedIo::new();
//!     simulated_io.env().set_current_dir(Path::new("/foo/bar")).unwrap();
//!     do_work(&mut simulated_io);
//!     assert_eq!(
//!         "The current directory is: /foo/bar\n",
//!         ::std::str::from_utf8(simulated_io.stream().read_output()).unwrap());
//!
//!     // Now use a native I/O provided to access the real system
//!     let mut real_io = NativeIo::new();
//!     do_work(&mut real_io);
//! }
//! ```

pub mod env;
pub mod std_streams;

pub use env::{Env, NativeEnv, SimulatedEnv};
pub use std_streams::{NativeStdStreams, SimulatedStdStreams, StdStreams};

/// Provides access to an environment provider and a stream provider.
///
/// See `env::Env` and `std_streams::StdStreams` for more information.
pub trait Io {
    // The type of the environment provider.
    type E: env::Env;

    // The type of the stream provider.
    type S: std_streams::StdStreams;

    /// Gets the `env::Env` provider.
    fn env(&mut self) -> &mut Self::E;

    /// Gets the `stream::Provider`.
    fn stream(&mut self) -> &mut Self::S;
}

/// `Io` implementation using the native system.
///
/// See `env::NativeEnv` and `std_streams::NativeStdStreams` for more information.
pub struct NativeIo {
    env: env::NativeEnv,
    stream: std_streams::NativeStdStreams,
}

impl NativeIo {
    /// Creates a new `LocalIoProvider`.
    pub fn new() -> NativeIo {
        NativeIo {
            env: env::NativeEnv,
            stream: std_streams::NativeStdStreams::new(),
        }
    }
}

impl Io for NativeIo {
    type E = env::NativeEnv;
    type S = std_streams::NativeStdStreams;

    fn env(&mut self) -> &mut env::NativeEnv {
        &mut self.env
    }

    fn stream(&mut self) -> &mut std_streams::NativeStdStreams {
        &mut self.stream
    }
}

/// `Io` implementation using a simulated environment.
///
/// See `env::SimulatedEnv` and `std_streams::SimulatedStdStreams` for more information.
pub struct SimulatedIo {
    env: env::SimulatedEnv,
    stream: std_streams::SimulatedStdStreams,
}

impl SimulatedIo {
    /// Creates a new `SimulatedIo`.
    pub fn new() -> SimulatedIo {
        SimulatedIo {
            env: env::SimulatedEnv::new(),
            stream: std_streams::SimulatedStdStreams::new(),
        }
    }
}

impl Io for SimulatedIo {
    type E = env::SimulatedEnv;
    type S = std_streams::SimulatedStdStreams;

    fn env(&mut self) -> &mut env::SimulatedEnv {
        &mut self.env
    }

    fn stream(&mut self) -> &mut std_streams::SimulatedStdStreams {
        &mut self.stream
    }
}
