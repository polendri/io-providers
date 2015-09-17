use std::io;
use StreamProvider;

/// Provides access to the standard streams (stdin, stdout and stderr).
pub struct Std {
    stdin: io::Stdin,
    stdout: io::Stdout,
    stderr: io::Stderr,
}

impl Std {
    /// Constructs a new standard stream provider.
    pub fn new() -> Std {
        Std {
            stdin: io::stdin(),
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    }
}

impl StreamProvider for Std {
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
