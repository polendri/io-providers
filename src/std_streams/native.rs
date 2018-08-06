use std::io;
use std_streams::StdStreams;

/// Handles for the standard input streams of a process, using
/// `[std::io](https://doc.rust-lang.org/stable/std/io/)`.
pub struct NativeStdStreams {
    input: io::Stdin,
    output: io::Stdout,
    error: io::Stderr,
}

impl NativeStdStreams {
    /// Creates a new `NativeStdStreams`.
    ///
    /// This is necessary (as opposed to having `NativeStdStreams` be a unit struct) because the
    /// `std::io` functions return a new handle to their stream, so it's not possible to return
    /// `&mut` references to these handles unless we store them.
    pub fn new() -> Self {
        NativeStdStreams {
            input: io::stdin(),
            output: io::stdout(),
            error: io::stderr(),
        }
    }
}

impl Default for NativeStdStreams {
    fn default() -> Self {
        Self::new()
    }
}

impl StdStreams for NativeStdStreams {
    fn input(&mut self) -> &mut io::Read {
        &mut self.input
    }

    fn output(&mut self) -> &mut io::Write {
        &mut self.output
    }

    fn error(&mut self) -> &mut io::Write {
        &mut self.error
    }
}
