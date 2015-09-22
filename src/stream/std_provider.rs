use std::io;
use stream::StreamProvider;

/// Provides access to the standard streams (stdin, stdout and stderr).
pub struct StdStreamProvider {
    stdin: io::Stdin,
    stdout: io::Stdout,
    stderr: io::Stderr,
}

impl StdStreamProvider {
    /// Constructs a new standard stream provider.
    pub fn new() -> StdStreamProvider {
        StdStreamProvider {
            stdin: io::stdin(),
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    }
}

impl StreamProvider for StdStreamProvider {
    fn input(&mut self) -> &mut io::Read {
        &mut self.stdin
    }

    fn output(&mut self) -> &mut io::Write {
        &mut self.stdout
    }

    fn error(&mut self) -> &mut io::Write {
        &mut self.stderr
    }
}
