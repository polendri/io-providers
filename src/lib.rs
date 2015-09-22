mod std_provider;
mod virtual_provider;
//mod utils;

pub use std_provider::StdStreamProvider;
pub use self::virtual_provider::VirtualStreamProvider;

use std::io;

/// Provides access to input, output and error streams.
pub trait StreamProvider {
    /// Gets the input stream.
    fn input(&mut self) -> &mut io::Read;

    /// Gets the output stream.
    fn output(&mut self) -> &mut io::Write;

    /// Gets the error stream.
    fn error(&mut self) -> &mut io::Write;
}
