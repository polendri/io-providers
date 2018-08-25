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

    fn create_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::create_dir(self.change_path(path)?)
    }

    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::create_dir_all(self.change_path(path)?)
    }

    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        fs::hard_link(self.change_path(src)?, self.change_path(dst)?)
    }

    fn metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::Metadata> {
        fs::metadata(self.change_path(path)?)
    }

    fn read<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<u8>> {
        fs::read(self.change_path(path)?)
    }

    fn read_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::ReadDir> {
        fs::read_dir(self.change_path(path)?)
    }

    fn read_link<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        fs::read_link(self.change_path(path)?)
    }

    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        fs::read_to_string(self.change_path(path)?)
    }

    fn remove_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::remove_dir(self.change_path(path)?)
    }

    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::remove_dir_all(self.change_path(path)?)
    }

    fn remove_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::remove_file(self.change_path(path)?)
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<()> {
        fs::rename(self.change_path(from)?, self.change_path(to)?)
    }

    fn set_permissions<P: AsRef<Path>>(&self, path: P, perm: fs::Permissions) -> io::Result<()> {
        fs::set_permissions(self.change_path(path)?, perm)
    }

    fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::Metadata> {
        fs::symlink_metadata(self.change_path(path)?)
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, contents: C) -> io::Result<()> {
        fs::write(self.change_path(path)?, contents)
    }
}
