use std::io;
use StreamProvider;
use utils::ReplayReader;

/// Provides streams which log outputs and play back pre-specified input.
pub struct Logger {
    input: ReplayReader,
    output: Vec<u8>,
    error: Vec<u8>,
}

impl Logger {
    /// Constructs a new logging stream provider.
    ///
    /// `inputs` specifies the values which should be returned for each read from input.
    pub fn new<C, I>(inputs: C) -> Logger
        where C: IntoIterator<Item=io::Result<Vec<u8>>, IntoIter=I>,
              I: DoubleEndedIterator<Item=io::Result<Vec<u8>>>
    {
        Logger {
            input: ReplayReader::new(inputs),
            output: Vec::new(),
            error: Vec::new(),
        }
    }

    /// Gets the data which has been written to the output stream.
    pub fn get_output<'a>(&'a self) -> &'a [u8] {
        &self.output[..]
    }

    /// Gets the data which has been written to error stream.
    pub fn get_error<'a>(&'a self) -> &'a [u8] {
        &self.error[..]
    }
}

impl StreamProvider for Logger {
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
