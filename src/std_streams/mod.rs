//! Providers of input/output/error streams (i.e. stdin, stdout and stderr).
//!
//! # Examples
//!
//! ```
//! extern crate io_providers;
//!
//! use std::io::Write;
//! use std::path::PathBuf;
//! use io_providers::std_streams::{NativeStdStreams, SimulatedStdStreams, StdStreams};
//!
//! /// Takes input from stdin and prints it to stdout
//! fn passthrough<S: StdStreams>(streams: &mut S)  {
//!     let mut input = String::new();
//!     streams.input().read_to_string(&mut input).unwrap();
//!     write!(streams.output(), "{}", input);
//! }
//!
//! fn main() {
//!     // Use a simulated stream provider to test `passthrough()`
//!     let mut simulated_streams = SimulatedStdStreams::new();
//!     simulated_streams.write_input("test".as_bytes());
//!     passthrough(&mut simulated_streams);
//!     let actual = ::std::str::from_utf8(simulated_streams.read_output()).unwrap();
//!     assert_eq!("test", actual);
//!
//!     // Now use a standard stream provider here to interact with the real console
//!     let mut real_streams = NativeStdStreams::new();
//!     passthrough(&mut real_streams);
//! }
//! ```

mod native;
mod simulated;

use std::io;

pub use self::native::NativeStdStreams;
pub use self::simulated::SimulatedStdStreams;

/// Provides access to input, output and error streams.
pub trait StdStreams {
    /// Gets the input stream.
    fn input(&mut self) -> &mut io::Read;

    /// Gets the output stream.
    fn output(&mut self) -> &mut io::Write;

    /// Gets the error stream.
    fn error(&mut self) -> &mut io::Write;
}
