//! Defines "provider" traits and implementations for different types of I/O operations, enabling
//! dependency injection that's very helpful for testing.
//!
//! A number of different I/O types are supported:
//!
//! * Process environment (variables, working directy etc), via [`Env`](env/trait.Env.html)
//! * Standard streams (stdin, stdout and stderr), via [`StdStreams`](std_streams/trait.StdStreams.html)
//! * Filesystem access, via [`Fs`](fs/trait.Fs.html)
//!
//! In addition to "native" implementations for each trait, "simulated" implementations are also
//! built-in:
//!
//! * [`SimulatedEnv`](env/trait.SimulatedEnv.html) for faking process environment state
//! * [`SimulatedStdStreams`](std_streams/trait.SimulatedStdStreams.html) for faking standard
//!   stream input and inspecting output
//! * [`TempFs`](fs/trait.TempFs.html) for performing filesystem access in a `chroot`-like sandbox
//!   isolated from the rest of the filesystem
//!
//! Each provider trait can be used independently, however there is also the all-encompassing
//! [`Io`](trait.Io.html) which provides access to all of them. If you have a variety of I/O
//! dependencies, it might be easiest to create and pass around a single `&mut Io`.
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
//!     let stdout = io.std_streams().output();
//!     writeln!(stdout, "The current directory is: {}", cur_dir.to_str().unwrap()).unwrap();
//! }
//!
//! fn main() {
//!     // Test `do_work()` using a simulated I/O environment
//!     let mut simulated_io = SimulatedIo::new().unwrap();
//!     simulated_io.env().set_current_dir(Path::new("/foo/bar")).unwrap();
//!     do_work(&mut simulated_io);
//!     assert_eq!(
//!         "The current directory is: /foo/bar\n",
//!         ::std::str::from_utf8(simulated_io.std_streams().read_output()).unwrap());
//!
//!     // Now use a native I/O provided to access the real system
//!     let mut real_io = NativeIo::new();
//!     do_work(&mut real_io);
//! }
//! ```

extern crate tempfile;

use std::io;

pub mod env;
pub mod fs;
pub mod std_streams;

pub use env::{Env, NativeEnv, SimulatedEnv};
pub use fs::{Fs, NativeFs, OpenOptions, TempFs};
pub use std_streams::{NativeStdStreams, SimulatedStdStreams, StdStreams};

/// Provides access to the process environment, filesystem, and standard streams.
///
/// See [`env::Env`](env/trait.Env.html),
/// [`std_streams::StdStreams`](std_streams/trait.StdStreams.html) and
/// [`fs::Fs`](fs/trait.Fs.html) for details.
pub trait Io {
    // The type of the environment provider.
    type E: env::Env;

    // The type of the filesystem provider.
    type F: fs::Fs;

    // The type of the stream provider.
    type S: std_streams::StdStreams;

    /// Gets the [`env::Env`](env/trait.Env.html) provider.
    fn env(&mut self) -> &mut Self::E;

    /// Gets the [`fs::Fs`](fs/trait.Fs.html) provider.
    fn fs(&mut self) -> &mut Self::F;

    /// Gets the [`std_streams::StdStreams`](std_streams/trait.StdStreams.html).
    fn std_streams(&mut self) -> &mut Self::S;
}

/// `Io` implementation using the native system.
///
/// See `env::NativeEnv` and `std_streams::NativeStdStreams` for more information.
#[derive(Default)]
pub struct NativeIo {
    env: env::NativeEnv,
    fs: fs::NativeFs,
    stream: std_streams::NativeStdStreams,
}

impl NativeIo {
    /// Creates a new `LocalIoProvider`.
    pub fn new() -> NativeIo {
        NativeIo {
            env: env::NativeEnv,
            fs: fs::NativeFs,
            stream: std_streams::NativeStdStreams::new(),
        }
    }
}

impl Io for NativeIo {
    type E = env::NativeEnv;
    type F = fs::NativeFs;
    type S = std_streams::NativeStdStreams;

    fn env(&mut self) -> &mut env::NativeEnv {
        &mut self.env
    }

    fn fs(&mut self) -> &mut fs::NativeFs {
        &mut self.fs
    }

    fn std_streams(&mut self) -> &mut std_streams::NativeStdStreams {
        &mut self.stream
    }
}

/// `Io` implementation using a simulated environment.
///
/// See `env::SimulatedEnv` and `std_streams::SimulatedStdStreams` for more information.
pub struct SimulatedIo {
    env: env::SimulatedEnv,
    fs: fs::TempFs,
    stream: std_streams::SimulatedStdStreams,
}

impl SimulatedIo {
    /// Creates a new `SimulatedIo`.
    pub fn new() -> io::Result<SimulatedIo> {
        Ok(SimulatedIo {
            env: env::SimulatedEnv::new(),
            fs: fs::TempFs::new()?,
            stream: std_streams::SimulatedStdStreams::new(),
        })
    }
}

impl Io for SimulatedIo {
    type E = env::SimulatedEnv;
    type F = fs::TempFs;
    type S = std_streams::SimulatedStdStreams;

    fn env(&mut self) -> &mut env::SimulatedEnv {
        &mut self.env
    }

    fn fs(&mut self) -> &mut fs::TempFs {
        &mut self.fs
    }

    fn std_streams(&mut self) -> &mut std_streams::SimulatedStdStreams {
        &mut self.stream
    }
}
