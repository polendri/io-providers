use std::default::Default;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use fs::{Fs, OpenOptions};

/// Provides access to file I/O in a chroot-like temporary filesystem, located in the system's
/// default temp directory. This temporary directory acts like the root of the filesystem: all
/// absolute paths are relative to it, and any path which would traverse out of it is considered
/// invalid.
///
/// NOTE: This is NOT intended to act as a secure sandbox; while it ought to handle edge cases like
/// path traversals and symbolic links correctly, no attempt has been made to ensure it is fully
/// secure.
#[derive(Debug)]
pub struct TempFs {
    root: PathBuf,
}

impl TempFs {
    /// Creates a new `TempFs`.
    pub fn new() -> TempFs {
        Default::default()
    }

    fn change_path<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        let mut result = self.root.clone();
        result.push(path);
        result = result.canonicalize()?;

        if !result.starts_with(&self.root) {
            return Err(io::Error::new(io::ErrorKind::Other, "Not a valid path"))
        }

        Ok(result)
    }
}

impl Default for TempFs {
    fn default() -> TempFs {
        TempFs {
            root: env::temp_dir(),
        }
    }
}

impl Fs for TempFs {
    fn open<P: AsRef<Path>>(&self, path: &P, open_options: &OpenOptions) -> io::Result<fs::File> {
        open_options.as_std().open(self.change_path(path)?)
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<u64> {
        fs::copy(self.change_path(from)?, self.change_path(to)?)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::env;
    use std::io;
    use std::path::Path;

    use super::TempFs;
    use fs::{Fs, OpenOptions};

    #[test]
    fn open__path_traversal__returns_err() {
        let provider = TempFs::new();
        let mut open_options = OpenOptions::new();
        open_options.read(true);

        let result = provider.open(&Path::new("../home/paulh/.gitconffffig"), &open_options);

        assert!(result.is_err());
        assert_eq!(io::ErrorKind::Other, result.unwrap_err().kind());
    }
}
