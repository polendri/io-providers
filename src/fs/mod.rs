//! Providers of file I/O.
//!
//! # Examples
//!
//! ```
//! // TODO
//! ```

mod native;
mod temp;

use std::fs;
use std::io;
use std::path::Path;

pub use self::native::NativeFs;
pub use self::temp::TempFs;

/// Options and flags which can be used to configure how a file is opened.
///
/// This replicates [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html).
#[derive(Debug, Default)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// All options are initially set to `false`.
    pub fn new() -> OpenOptions {
        Default::default()
    }

    /// Sets the option for read access.
    ///
    /// This option, when true, will indicate that the file should be read-able if opened.
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    /// Sets the option for write access.
    ///
    /// This option, when true, will indicate that the file should be write-able if opened.
    ///
    /// See [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.write)
    /// for more information.
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    /// Sets the option for the append mode.
    ///
    /// This option, when true, means that writes will append to a file instead
    /// of overwriting previous contents.
    /// Note that setting `.write(true).append(true)` has the same effect as
    /// setting only `.append(true)`.
    ///
    /// See [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append)
    /// for more information.
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.append = append;
        self
    }

    /// Sets the option for truncating a previous file.
    ///
    /// If a file is successfully opened with this option set it will truncate
    /// the file to 0 length if it already exists.
    ///
    /// See [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.truncate)
    /// for more information.
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.truncate = truncate;
        self
    }

    /// Sets the option for creating a new file.
    ///
    /// This option indicates whether a new file will be created if the file
    /// does not yet already exist.
    ///
    /// See [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create)
    /// for more information.
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    /// Sets the option to always create a new file.
    ///
    /// This option indicates whether a new file will be created.
    /// No file is allowed to exist at the target location, also no (dangling)
    /// symlink.
    ///
    /// See [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append)
    /// for more information.
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        self.create_new = create_new;
        self
    }

    fn as_std(&self) -> fs::OpenOptions {
        let mut open_options = fs::OpenOptions::new();
        open_options.read(self.read)
            .write(self.write)
            .append(self.append)
            .truncate(self.truncate)
            .create(self.create)
            .create_new(self.create_new);
        open_options
    }
}

/// Provides access to file I/O.
pub trait Fs {
    /// Opens a file at `path` with the options specified by `open_options`.
    ///
    /// See [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open)
    /// for more information.
    fn open<P: AsRef<Path>>(&self, path: &P, open_options: &OpenOptions) -> io::Result<fs::File>;

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> io::Result<u64>;
}
