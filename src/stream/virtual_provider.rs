use std::collections::VecDeque;
use std::io;
use std::io::{Read, Write};
use stream;

/// Provides virtual input/output/error streams: input can be provided using
/// `Virtual::write_input()`, and output can be observed using `Virtual::read_output()` and
/// `Virtual::read_error()`.
pub struct Virtual {
    inputs: ChunkPipe,
    output: Vec<u8>,
    error: Vec<u8>,
}

impl Virtual {
    /// Creates a new, empty virtual stream provider.
    ///
    /// TODO example
    pub fn new() -> Virtual {
        Virtual {
            inputs: ChunkPipe::new(),
            output: Vec::new(),
            error: Vec::new(),
        }
    }

    /// Writes the provided buffer to the queue of buffers that to be used when input is requested
    /// from this provider using `Provider::input()`.
    ///
    /// In particular, this method does NOT append data to a continuous buffer which is consumed
    /// by `Provider::input()`; rather, it enqueues a buffer which will be used for a SINGLE call
    /// to `Provider::input()`. The buffer is then discarded, regardless of how much of it was
    /// (or was not) read.
    ///
    /// This enables precise control over the length of data returned from a call to
    /// `Provider::input()`.
    ///
    /// TODO: example
    pub fn write_input(&mut self, input: &[u8]) {
        self.inputs.write(input).unwrap();
    }

    /// Gets the data which has been written to the output stream.
    ///
    /// TODO: example
    pub fn read_output<'a>(&'a self) -> &'a [u8] {
        &self.output[..]
    }

    /// Gets the data which has been written to error stream.
    ///
    /// TODO: example
    pub fn read_error<'a>(&'a self) -> &'a [u8] {
        &self.error[..]
    }
}

impl stream::Provider for Virtual {
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
struct ChunkPipe {
    items: VecDeque<Vec<u8>>
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
        }
        else {
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
    use std::io::{Read, Write};
    use super::{ChunkPipe, Virtual};
    use stream::Provider;

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
        let mut provider = Virtual::new();
        let mut buf = vec![0;4];

        let result = provider.input().read(&mut buf).unwrap();

        assert_eq!(0, result);
    }

    #[test]
    fn provider__write_and_read_input__success() {
        let mut provider = Virtual::new();
        let expected = "test";
        let mut actual = String::new();
        let mut buf = vec![0;4];

        provider.write_input(expected.as_bytes());
        let result = provider.input().read_to_string(&mut actual).unwrap();

        assert_eq!(expected.len(), result);
        assert_eq!(expected, actual);

        let result = provider.input().read(&mut buf).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn provider__two_input_writes__two_reads() {
        let mut provider = Virtual::new();
        let (expected1, expected2) = (vec![1,2,3], vec![4,5,6]);
        let (mut actual1, mut actual2) = (vec![0;3], vec![0;3]);

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
        let mut provider = Virtual::new();

        let result1 = provider.output().write(&[1,2]).unwrap();
        let result2 = provider.output().write(&[3,4]).unwrap();
        let actual = provider.read_output();

        assert_eq!(2, result1);
        assert_eq!(2, result2);
        assert_eq!(&[1,2,3,4], actual);
    }

    #[test]
    fn provider__write_read_error__success() {
        let mut provider = Virtual::new();

        let result1 = provider.error().write(&[1,2]).unwrap();
        let result2 = provider.error().write(&[3,4]).unwrap();
        let actual = provider.read_error();

        assert_eq!(2, result1);
        assert_eq!(2, result2);
        assert_eq!(&[1,2,3,4], actual);
    }
}
