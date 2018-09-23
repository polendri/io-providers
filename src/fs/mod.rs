//! Defines traits and implementations for filesystem manipulation operations.

mod native;
mod temp;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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
        open_options
            .read(self.read)
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
    fn open<P: AsRef<Path>>(&mut self, path: P, open_options: &OpenOptions)
        -> io::Result<fs::File>;

    /// Returns the canonical, absolute form of a path with all intermediate components normalized
    /// and symbolic links resolved.
    ///
    /// See [std::fs::canonicalize](https://doc.rust-lang.org/std/fs/fn.canonicalize.html) for more
    /// information.
    fn canonicalize<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf>;

    /// Copies the contents of one file to another. This function will also copy the permission bits
    /// of the original file to the destination file.
    ///
    /// See [std::fs::copy](https://doc.rust-lang.org/std/fs/fn.copy.html) for more information.
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> io::Result<u64>;

    /// Creates a new, empty directory at the provided path.
    ///
    /// See [std::fs::create_dir](https://doc.rust-lang.org/std/fs/fn.create_dir.html) for more
    /// information.
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// See [std::fs::create_dir_all](https://doc.rust-lang.org/std/fs/fn.create_dir_all.html) for
    /// more information.
    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;

    /// Creates a new hard link on the filesystem.
    ///
    /// The `dst` path will be a link pointing to the `src` path. Note that systems often require
    /// these two paths to both be located on the same filesystem.
    ///
    /// See [std::fs::hard_link](https://doc.rust-lang.org/std/fs/fn.hard_link.html) for
    /// more information.
    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, src: P, dst: Q) -> io::Result<()>;

    /// Given a path, query the file system to get information about a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query information about the destination file.
    ///
    /// See [std::fs::metadata](https://doc.rust-lang.org/std/fs/fn.metadata.html) for more
    /// information.
    fn metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::Metadata>;

    /// Read the entire contents of a file into a bytes vector.
    ///
    /// This is a convenience function for using `fs::File::open` and `fs::read_to_end`
    /// with fewer imports and without an intermediate variable.  It pre-allocates a
    /// buffer based on the file size when available, so it is generally faster than
    /// reading into a vector created with `Vec::new()`.
    ///
    /// See [std::fs::read](https://doc.rust-lang.org/std/fs/fn.read.html) for more information.
    fn read<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<u8>>;

    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of `io::Result<fs::DirEntry]>`.
    /// New errors may be encountered after an iterator is initially constructed.
    ///
    /// See [std::fs::read_dir](https://doc.rust-lang.org/std/fs/fn.read_dir.html) for more
    /// information.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::ReadDir>;

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// See [std::fs::read_link](https://doc.rust-lang.org/std/fs/fn.read_link.html) for more
    /// information.
    fn read_link<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf>;

    /// Read the entire contents of a file into a string.
    ///
    /// This is a convenience function for using `fs::File::open` and `fs::read_to_string`
    /// with fewer imports and without an intermediate variable.  It pre-allocates a
    /// buffer based on the file size when available, so it is generally faster than
    /// reading into a string created with `String::new()`.
    ///
    /// See [std::fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) for
    /// more information.
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String>;

    /// Removes an existing, empty directory.
    ///
    /// See [std::fs::remove_dir](https://doc.rust-lang.org/std/fs/fn.remove_dir.html) for more
    /// information.
    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;

    /// Removes a directory at this path, after removing all its contents. Use
    /// carefully!
    ///
    /// This function does **not** follow symbolic links and it will simply remove the
    /// symbolic link itself.
    ///
    /// See [std::fs::remove_dir_all](https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html) for
    /// more information.
    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;

    /// Removes a file from the filesystem.
    ///
    /// Note that there is no
    /// guarantee that the file is immediately deleted (e.g. depending on
    /// platform, other open file descriptors may prevent immediate removal).
    ///
    /// See [std::fs::remove_file](https://doc.rust-lang.org/std/fs/fn.remove_file.html) for more
    /// information.
    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()>;

    /// Rename a file or directory to a new name, replacing the original file if
    /// `to` already exists.
    ///
    /// This will not work if the new name is on a different mount point.
    ///
    /// See [std::fs::rename](https://doc.rust-lang.org/std/fs/fn.rename.html) for more information.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> io::Result<()>;

    /// Changes the permissions found on a file or a directory.
    ///
    /// See [std::fs::set_permissions](https://doc.rust-lang.org/std/fs/fn.set_permissions.html) for
    /// more information.
    fn set_permissions<P: AsRef<Path>>(&mut self, path: P, perm: fs::Permissions)
        -> io::Result<()>;

    /// Query the metadata about a file without following symlinks.
    ///
    /// See [std::fs::symlink_metadata](https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html)
    /// for more information.
    fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::Metadata>;

    /// Write a slice as the entire contents of a file.
    ///
    /// This function will create a file if it does not exist,
    /// and will entirely replace its contents if it does.
    ///
    /// This is a convenience function for using `fs::File::create` and `fs::write_all`
    /// with fewer imports.
    ///
    /// See [std::fs::write](https://doc.rust-lang.org/std/fs/fn.write.html) for more information.
    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, contents: C) -> io::Result<()>;

    /// Returns whether the path points at an existing entity.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `false`.
    ///
    /// If you cannot access the directory containing the file, e.g. because of a
    /// permission error, this will return `false`.
    ///
    /// See [std::path::Path.exists](https://doc.rust-lang.org/std/path/struct.Path.html#method.exists)
    /// for more information.
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool;
}
