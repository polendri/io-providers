use std::io;
use std::iter::{DoubleEndedIterator, IntoIterator};

/// A `Read` implementer which regurgitates data specified at the time of its creation.
pub struct ReplayReader {
    /// The collection of remaining inputs to return, one per `read()` call, in reverse order.
    inputs: Vec<io::Result<Vec<u8>>>,
}

impl ReplayReader {
    /// Constructs a new `ReplayReader` given a collection of inputs.
    pub fn new<C, I>(inputs: C) -> ReplayReader
        where C: IntoIterator<Item=io::Result<Vec<u8>>, IntoIter=I>,
              I: DoubleEndedIterator<Item=io::Result<Vec<u8>>>
    {
        ReplayReader {
            inputs: inputs.into_iter().rev().collect(),
        }
    }
}

impl io::Read for ReplayReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(input) = self.inputs.pop() {
            input.and_then(|o| {
                let mut cursor: io::Cursor<Vec<u8>> = io::Cursor::new(o);
                cursor.read(buf)
            })
        }
        else {
            Ok(0)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use std::io;
    use std::io::Read;

    #[test]
    fn read__empty_input__returns_length_zero_read() {
        let mut buf: Vec<u8> = vec![0; 8];
        let mut replayer = ReplayReader::new(vec![]);

        let result = replayer.read(&mut buf);
        assert_eq!(0, result.unwrap());
    }

    #[test]
    fn read__single_read__next_read_returns_length_zero() {
        let mut buf = vec![0; 8];
        let mut replayer = ReplayReader::new(vec![Ok(vec![])]);

        replayer.read(&mut buf).unwrap();
        let result = replayer.read(&mut buf);

        assert_eq!(0, result.unwrap());
    }

    #[test]
    fn read__error_input__returns_error() {
        let mut buf: Vec<u8> = vec![0; 8];
        let kind = io::ErrorKind::Other;
        let mut replayer = ReplayReader::new(vec![Err(io::Error::new(kind, ":("))]);

        let result = replayer.read(&mut buf);

        assert_eq!(kind, result.unwrap_err().kind());
    }

    #[test]
    fn read__exact_length_read__success() {
        let mut buf = vec![0; 5];
        let input = vec![1, 2, 3, 4, 5];
        let mut replayer = ReplayReader::new(vec![Ok(input.clone())]);

        let result = replayer.read(&mut buf);

        assert_eq!(5, result.unwrap());
        assert_eq!(input, buf);
    }

    #[test]
    fn read__shorter_read__success() {
        let mut buf = vec![0; 3];
        let input = vec![1, 2, 3, 4, 5];
        let mut replayer = ReplayReader::new(vec![Ok(input.clone())]);

        let result = replayer.read(&mut buf);

        assert_eq!(3, result.unwrap());
        assert_eq!(&input[0..3], &buf[..]);
    }

    #[test]
    fn read__longer_read__success() {
        let mut buf = vec![0; 8];
        let input = vec![1, 2, 3, 4, 5];
        let mut replayer = ReplayReader::new(vec![Ok(input)]);

        let result = replayer.read(&mut buf);
        assert_eq!(5, result.unwrap());
        assert_eq!(vec![1,2,3,4,5,0,0,0], buf);
    }

    #[test]
    fn read__two_reads__success() {
        let mut buf = vec![0; 5];
        let input = vec![1, 2, 3, 4, 5];
        let kind = io::ErrorKind::Other;
        let mut replayer = ReplayReader::new(
            vec![Ok(input.clone()), Err(io::Error::new(kind, ":("))]);

        let result1 = replayer.read(&mut buf);
        let result2 = replayer.read(&mut buf);

        assert_eq!(5, result1.unwrap());
        assert_eq!(input, buf);
        assert_eq!(kind, result2.unwrap_err().kind());
    }
}
