mod std_provider;
mod virtual_provider;

pub use self::std_provider::Std;
pub use self::virtual_provider::Virtual;

use std::io;

/// Provides access to input, output and error streams.
pub trait Provider {
    /// Gets the input stream.
    fn input(&mut self) -> &mut io::Read;

    /// Gets the output stream.
    fn output(&mut self) -> &mut io::Write;

    /// Gets the error stream.
    fn error(&mut self) -> &mut io::Write;
}
