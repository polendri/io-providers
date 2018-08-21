use std::fs;
use std::io;
use std::path::Path;

use fs::{Fs, OpenOptions};

/// Provides access to native file I/O.
#[derive(Debug, Default)]
pub struct NativeFs;

impl Fs for NativeFs {
    fn open<P: AsRef<Path>>(&self, path: &P, open_options: &OpenOptions) -> io::Result<fs::File> {
        open_options.as_std().open(path)
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<u64> {
        fs::copy(from, to)
    }
}
