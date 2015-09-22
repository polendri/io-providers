use std::collections::VecDeque;
use std::io;
use std::io::{Read, Write};
use stream::StreamProvider;

/// Provides streams which log outputs and play back pre-specified input.
pub struct VirtualStreamProvider {
    inputs: ChunkPipe,
    output: Vec<u8>,
    error: Vec<u8>,
}

impl VirtualStreamProvider {
    pub fn new<C, I>() -> VirtualStreamProvider {
        VirtualStreamProvider {
            inputs: ChunkPipe::new(),
            output: Vec::new(),
            error: Vec::new(),
        }
    }

    pub fn write_input(&mut self, input: &[u8]) {
        self.inputs.write(input).unwrap();
    }

    /// Gets the data which has been written to the output stream.
    pub fn read_output<'a>(&'a self) -> &'a [u8] {
        &self.output[..]
    }

    /// Gets the data which has been written to error stream.
    pub fn read_error<'a>(&'a self) -> &'a [u8] {
        &self.error[..]
    }
}

impl StreamProvider for VirtualStreamProvider {
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
pub struct ChunkPipe {
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
    use super::*;
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
}
