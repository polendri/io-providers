mod local_provider;
mod virtual_provider;

pub use self::local_provider::Local;
pub use self::virtual_provider::Virtual;

use std::ffi;
use std::io;
use std::path::{Path, PathBuf};

/// Provides access to a filesystem.
pub trait Provider {
    /// Representation of a file on the filesystem.
    type F: File;

    /// Iterator over the entries in a directory.
    type D: Iterator<Item=io::Result<Self::E>>;

    /// Representation of a directory entry.
    type E: DirEntry<M=Self::M>;

    /// Representation of file metadata.
    type M: Metadata<P=Self::P>;

    /// Representatino of file permissions.
    type P: Permissions;

    /// Opens a file in write-only mode.
    ///
    /// See `std::fs::File::create` for more information.
    fn create_file<P: AsRef<Path>>(path: P) -> io::Result<Self::F> {
        Self::open_file_as(path, OpenMode::Write)
    }

    /// Opens a file in read-only mode.
    ///
    /// See `std::fs::File::open` for more information.
    fn open_file<P: AsRef<Path>>(path: P) -> io::Result<Self::F> {
        Self::open_file_as(path, OpenMode::Read)
    }

    /// Opens a file using the specified mode.
    ///
    /// See `OpenMode` for a description of what each mode does.
    fn open_file_as<P: AsRef<Path>>(path: P, mode: OpenMode) -> io::Result<Self::F>;

    /// Copies the contents of one file to another. This function will also copy the permission
    /// bits of the original file to the destination file.
    ///
    /// See `std::fs::copy` for more information.
    fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64>;

    /// Removes a file from the underlying filesystem.
    ///
    /// See `std::fs::remove_file` for more information.
    fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Creates a new, empty directory at the provided path.
    ///
    /// See `std::fs::create_dir` for more information.
    fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Recursively create a directory and all of its parent components if they are missing.
    ///
    /// See `std::fs::create_dir_all` for more information.
    fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Returns an iterator over the entries within a directory.
    ///
    /// See `std::fs::read_dir` for more information.
    fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Self::D>;

    /// Removes an existing, empty directory.
    ///
    /// See `std::fs::remove_dir` for more information.
    fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    ///
    /// See `std::fs::remove_dir_all` for more information.
    fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()>;

    /// Given a path, query the file system to get information about a file, directory, etc.
    ///
    /// See `std::fs::metadata` for more information.
    fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Self::M>;

    /// Rename a file or directory to a new name.
    ///
    /// See `std::fs::rename` for more information.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()>;

    /// Changes the permissions found on a file or a directory.
    ///
    /// See `std::fs::set_permissions` for more information.
    fn set_permissions<P: AsRef<Path>>(path: P, perm: Self::P) -> io::Result<()>;

    /// Creates a new hard link on the filesystem.
    ///
    /// See `std::fs::hard_link` for more information.
    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()>;

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// See `std::fs::read_link` for more information.
    fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf>;

    /// Query the metadata about a file without following symlinks.
    ///
    /// See `std::fs::symlink_metadata` for more information.
    fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Self::M>;
}

/// Enumerates the ways in which a file can be opened.
pub enum OpenMode {
    /// Open for reading, starting at the beginning of the file.
    Read,
    /// Open for writing, starting at the beginning of the file. Deletes any existing contents.
    Write,
    /// Open for writing, starting at the end of the file.
    Append,
    /// Open for both reading and writing, starting at the beginning of the file.
    ReadWrite,
}

/// Represents a file on the filesystem.
pub trait File: io::Read + io::Write + io::Seek {
    /// Attempts to sync all OS-internal metadata to disk.
    ///
    /// See `std::fs::File::sync_all` for more information.
    fn sync_all(&self) -> io::Result<()>;

    /// This function is similar to `sync_all`, except that it may not synchronize file metadata to
    /// the filesystem.
    ///
    /// See `std::fs::File::sync_data` for more information.
    fn sync_data(&self) -> io::Result<()>;
}

/// Represents a directory entry.
pub trait DirEntry {
    /// Representation of file metadata.
    type M: Metadata;

    /// Returns the full path to the file that this entry represents.
    ///
    /// See `std::fs::DirEntry::path` for more information.
    fn path(&self) -> PathBuf;

    /// Return the metadata for the file that this entry points at.
    ///
    /// See `std::fs::DirEntry::metadata` for more information.
    fn metadata(&self) -> io::Result<Self::M>;

    /// Return the file type for the file that this entry points at.
    ///
    /// See `std::fs::DirEntry::file_type` for more information.
    fn file_type(&self) -> io::Result<FileType>;

    /// Returns the bare file name of this directory entry without any other leading path
    /// component.
    ///
    /// See `std::fs::DirEntry::file_name` for more information.
    fn file_name(&self) -> ffi::OsString;
}

/// Represents metadata for a file.
pub trait Metadata {
    /// Representatino of file permissions.
    type P: Permissions;

    /// Returns the file type for this metadata.
    ///
    /// See `std::fs::Metadata::file_type` for more information.
    fn file_type(&self) -> FileType;

    /// Returns whether this metadata is for a directory.
    ///
    /// See `std::fs::Metadata::is_dir` for more information.
    fn is_dir(&self) -> bool;

    /// Returns whether this metadata is for a regular file.
    ///
    /// See `std::fs::Metadata::is_file` for more information.
    fn is_file(&self) -> bool;

    /// Returns the size of the file, in bytes, this metadata is for.
    ///
    /// See `std::fs::Metadata::len` for more information.
    fn len(&self) -> u64;

    /// Returns the permissions of the file this metadata is for.
    ///
    /// See `std::fs::Metadata::permissions` for more information.
    fn permissions(&self) -> Self::P;
}

/// Represents the type of a file.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FileType {
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
}

impl FileType {
    /// Test whether this file type represents a directory.
    ///
    /// See `std::fs::FileType::is_dir` for more information.
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// Test whether this file type represents a regular file.
    ///
    /// See `std::fs::FileType::is_file` for more information.
    pub fn is_file(&self) -> bool {
        self.is_file
    }

    /// Test whether this file type represents a symbolic link.
    ///
    /// See `std::fs::FileType::is_symlink` for more information.
    pub fn is_symlink(&self) -> bool {
        self.is_symlink
    }
}

/// Represents the permissions of a file.
pub trait Permissions {
    /// Returns whether these permissions describe a readonly file.
    ///
    /// See `std::fs::Permissions::readonly` for more information.
    fn readonly(&self) -> bool;
}
