//! Providers of input/output/error streams (i.e. stdin, stdout and stderr).
//!
//! # Examples
//! 
//! ```
//! extern crate io_providers;
//!
//! use std::io::Write;
//! use std::path::PathBuf;
//! use io_providers::stream;
//! use io_providers::stream::Provider;
//!
//! /// Takes input from stdin and prints it to stdout
//! fn mirror<S: stream::Provider>(streams: &mut S)  {
//!     let mut input = String::new();
//!     streams.input().read_to_string(&mut input).unwrap();
//!     write!(streams.output(), "{}", input);
//! }
//!
//! fn main() {
//!     test_mirror();
//!
//!     // Use a standard stream provider here to interact with the console
//!     let mut std = stream::Std::new();
//!     mirror(&mut std);
//! }
//!
//! fn test_mirror() {
//!     // Use a virtual stream provider here to verify the functionality of `mirror()`
//!     let mut streams = stream::Virtual::new();
//!     let expected = "test";
//!     streams.write_input(expected.as_bytes());
//!
//!     mirror(&mut streams);
//!
//!     let actual = ::std::str::from_utf8(streams.read_output()).unwrap();
//!     assert_eq!(expected, actual);
//! }
//! ```

mod std_provider;
mod virtual_provider;

use std::io;

pub use self::std_provider::Std;
pub use self::virtual_provider::Virtual;

/// Provides access to input, output and error streams.
pub trait Provider {
    /// Gets the input stream.
    fn input(&mut self) -> &mut io::Read;

    /// Gets the output stream.
    fn output(&mut self) -> &mut io::Write;

    /// Gets the error stream.
    fn error(&mut self) -> &mut io::Write;
}
