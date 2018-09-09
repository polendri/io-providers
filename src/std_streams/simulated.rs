use std::collections::VecDeque;
use std::io;
use std::io::{Read, Write};
use std_streams::StdStreams;

/// Simulated handles for the standard input streams of a process.
///
/// Simulated input can be provided using
/// [`write_input()`](std_streams/struct.SimulatedStdStreams.html#method.write_input), and output
/// can be observed using [`read_output()`](std_streams/struct.SimulatedStdStreams.html#method.read_output)
/// and [`read_error()`](std_streams/struct.SimulatedStdStreams.html#method.read_error).
#[derive(Default)]
pub struct SimulatedStdStreams {
    inputs: ChunkPipe,
    output: Vec<u8>,
    error: Vec<u8>,
}

impl SimulatedStdStreams {
    /// Creates a new `SimulatedStdStreams`.
    pub fn new() -> SimulatedStdStreams {
        SimulatedStdStreams {
            inputs: ChunkPipe::new(),
            output: Vec::new(),
            error: Vec::new(),
        }
    }

    /// Writes the provided buffer to the queue of buffers to be used when input is requested
    /// using [`StdStreams::input()`].
    ///
    /// In particular, this method does NOT append data to a continuous buffer which is consumed
    /// by [`StdStreams::input()`]; rather, it enqueues a buffer which will be used for a SINGLE
    /// call to [`StdStreams::input()`]. The buffer is then discarded, regardless of how much of it
    /// was (or was not) read.
    ///
    /// This enables precise control over the length of data returned from a call to
    /// [`StdStreams::input()`].
    ///
    /// [`StdStreams::input()`]: trait.StdStreams.html#tymethod.input
    ///
    /// ## Example
    ///
    /// ```
    /// use io_providers::{StdStreams, SimulatedStdStreams};
    ///
    /// let mut streams = SimulatedStdStreams::new();
    /// streams.write_input("foo".as_bytes());
    /// streams.write_input("bar".as_bytes());
    /// // The first read on `streams.input()` will read from "foo"
    /// // The second read on `streams.input()` will read from "bar"
    /// ```
    pub fn write_input(&mut self, input: &[u8]) {
        self.inputs.write_all(input).unwrap();
    }

    /// Gets the data which has been written to the output stream.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::io::Write;
    /// use io_providers::{StdStreams, SimulatedStdStreams};
    ///
    /// let mut streams = SimulatedStdStreams::new();
    /// writeln!(streams.output(), "test1");
    /// write!(streams.output(), "test2");
    /// assert_eq!("test1\ntest2", ::std::str::from_utf8(streams.read_output()).unwrap());
    /// ```
    pub fn read_output(&self) -> &[u8] {
        &self.output[..]
    }

    /// Gets the data which has been written to the error stream.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::io::Write;
    /// use io_providers::{StdStreams, SimulatedStdStreams};
    ///
    /// let mut streams = SimulatedStdStreams::new();
    /// writeln!(streams.error(), "test1");
    /// write!(streams.error(), "test2");
    /// assert_eq!("test1\ntest2", ::std::str::from_utf8(streams.read_error()).unwrap());
    /// ```
    pub fn read_error(&self) -> &[u8] {
        &self.error[..]
    }
}

impl StdStreams for SimulatedStdStreams {
    fn input(&mut self) -> &mut Read {
        &mut self.inputs
    }

    fn output(&mut self) -> &mut Write {
        &mut self.output
    }

    fn error(&mut self) -> &mut Write {
        &mut self.error
    }
}

/// A `Read` and `Write` implementer where data is written in chunks and each read consumes a
/// single chunk.
#[derive(Default)]
struct ChunkPipe {
    items: VecDeque<Vec<u8>>,
}

impl ChunkPipe {
    /// Creates a new, empty `ChunkPipe`.
    pub fn new() -> ChunkPipe {
        ChunkPipe {
            items: VecDeque::new(),
        }
    }
}

impl Read for ChunkPipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(item) = self.items.pop_front() {
            io::Cursor::new(item).read(buf)
        } else {
            Ok(0)
        }
    }
}

impl Write for ChunkPipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut vec = Vec::new();
        let result = vec.write(buf);
        self.items.push_back(vec);
        result
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::{ChunkPipe, SimulatedStdStreams, StdStreams};
    use std::io::{Read, Write};

    #[test]
    fn chunk_pipe__no_writes__reads_successfully() {
        let mut buf: Vec<u8> = vec![0; 8];
        let mut pipe = ChunkPipe::new();
        pipe.write(&[]).unwrap();

        let result = pipe.read(&mut buf);
        assert_eq!(0, result.unwrap());
    }

    #[test]
    fn chunk_pipe__one_write__reads_successfully() {
        let data = vec![1, 2, 3];
        let mut buf1 = vec![0; 4];
        let mut buf2 = vec![0; 4];
        let mut pipe = ChunkPipe::new();

        pipe.write(&data[..]).unwrap();
        let result1 = pipe.read(&mut buf1).unwrap();
        let result2 = pipe.read(&mut buf2).unwrap();

        assert_eq!(data.len(), result1);
        assert_eq!(vec![1, 2, 3, 0], buf1);
        assert_eq!(0, result2);
    }

    #[test]
    fn chunk_pipe__two_writes__reads_successfully() {
        let data1 = vec![1, 2, 3];
        let data2 = vec![4, 5, 6, 7];
        let mut buf1 = vec![0; 4];
        let mut buf2 = vec![0; 3];
        let mut buf3 = vec![0; 3];
        let mut pipe = ChunkPipe::new();

        pipe.write(&data1[..]).unwrap();
        let result1 = pipe.read(&mut buf1).unwrap();
        pipe.write(&data2[..]).unwrap();
        let result2 = pipe.read(&mut buf2).unwrap();
        let result3 = pipe.read(&mut buf3).unwrap();

        assert_eq!(data1.len(), result1);
        assert_eq!(vec![1, 2, 3, 0], buf1);
        assert_eq!(buf2.len(), result2);
        assert_eq!(vec![4, 5, 6], buf2);
        assert_eq!(0, result3);
    }

    #[test]
    fn provider__empty_input__length_zero_read() {
        let mut provider = SimulatedStdStreams::new();
        let mut buf = vec![0; 4];

        let result = provider.input().read(&mut buf).unwrap();

        assert_eq!(0, result);
    }

    #[test]
    fn provider__write_and_read_input__success() {
        let mut provider = SimulatedStdStreams::new();
        let expected = "test";
        let mut actual = String::new();
        let mut buf = vec![0; 4];

        provider.write_input(expected.as_bytes());
        let result = provider.input().read_to_string(&mut actual).unwrap();

        assert_eq!(expected.len(), result);
        assert_eq!(expected, actual);

        let result = provider.input().read(&mut buf).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn provider__two_input_writes__two_reads() {
        let mut provider = SimulatedStdStreams::new();
        let (expected1, expected2) = (vec![1, 2, 3], vec![4, 5, 6]);
        let (mut actual1, mut actual2) = (vec![0; 3], vec![0; 3]);

        provider.write_input(&expected1[..]);
        provider.write_input(&expected2[..]);
        let result1 = provider.input().read(&mut actual1).unwrap();
        let result2 = provider.input().read(&mut actual2).unwrap();

        assert_eq!(expected1.len(), result1);
        assert_eq!(expected1, actual1);
        assert_eq!(expected2.len(), result2);
        assert_eq!(expected2, actual2);
    }

    #[test]
    fn provider__write_read_output__success() {
        let mut provider = SimulatedStdStreams::new();

        let result1 = provider.output().write(&[1, 2]).unwrap();
        let result2 = provider.output().write(&[3, 4]).unwrap();
        let actual = provider.read_output();

        assert_eq!(2, result1);
        assert_eq!(2, result2);
        assert_eq!(&[1, 2, 3, 4], actual);
    }

    #[test]
    fn provider__write_read_error__success() {
        let mut provider = SimulatedStdStreams::new();

        let result1 = provider.error().write(&[1, 2]).unwrap();
        let result2 = provider.error().write(&[3, 4]).unwrap();
        let actual = provider.read_error();

        assert_eq!(2, result1);
        assert_eq!(2, result2);
        assert_eq!(&[1, 2, 3, 4], actual);
    }
}
