use std::convert::From;
use std::ffi;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use filesystem::{DirEntry, File, FileType, OpenMode, Provider, Metadata, Permissions};

/// Represents a local filesystem.
pub struct Local;

impl Provider for Local {
    type F = fs::File;
    type D = fs::ReadDir;
    type E = fs::DirEntry;
    type M = fs::Metadata;
    type P = fs::Permissions;

    fn open_file_as<P:AsRef<Path>>(path: P, mode: OpenMode) -> io::Result<fs::File> {
        match mode {
            OpenMode::Read => fs::File::open(path),
            OpenMode::Write => fs::File::create(path),
            OpenMode::Append => fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path),
            OpenMode::ReadWrite => fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path),
        }
    }

    fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
        fs::copy(from, to)
    }

    fn remove_file<P:AsRef<Path>>(path: P) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::create_dir(path)
    }

    fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<fs::ReadDir> {
        fs::read_dir(path)
    }

    fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::remove_dir(path)
    }

    fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    fn metadata<P: AsRef<Path>>(path: P) -> io::Result<fs::Metadata> {
        fs::metadata(path)
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
        fs::rename(from, to)
    }

    fn set_permissions<P: AsRef<Path>>(path: P, perm: fs::Permissions) -> io::Result<()> {
        fs::set_permissions(path, perm)
    }

    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
        fs::hard_link(src, dst)
    }

    fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        fs::read_link(path)
    }

    fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<fs::Metadata> {
        fs::symlink_metadata(path)
    }
}

impl File for fs::File {
    fn sync_all(&self) -> io::Result<()> {
        self.sync_all()
    }

    fn sync_data(&self) -> io::Result<()> {
        self.sync_data()
    }
}

impl DirEntry for fs::DirEntry {
    type M = fs::Metadata;

    fn path(&self) -> PathBuf {
        self.path()
    }

    fn metadata(&self) -> io::Result<fs::Metadata> {
        self.metadata()
    }

    fn file_type(&self) -> io::Result<FileType> {
        self.file_type().map(FileType::from)
    }

    fn file_name(&self) -> ffi::OsString {
        self.file_name()
    }
}

impl Metadata for fs::Metadata {
    type P = fs::Permissions;

    fn file_type(&self) -> FileType {
        FileType::from(self.file_type())
    }

    fn is_dir(&self) -> bool {
        self.is_dir()
    }

    fn is_file(&self) -> bool {
        self.is_file()
    }

    fn len(&self) -> u64 {
        self.len()
    }

    fn permissions(&self) -> fs::Permissions {
        self.permissions()
    }
}

impl From<fs::FileType> for FileType {
    fn from(m: fs::FileType) -> FileType {
        FileType {
            is_dir: m.is_dir(),
            is_file: m.is_file(),
            is_symlink: m.is_symlink(),
        }
    }
}

impl Permissions for fs::Permissions {
    fn readonly(&self) -> bool {
        self.readonly()
    }
}
