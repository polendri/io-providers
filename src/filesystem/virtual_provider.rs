use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use filesystem;
use filesystem::{FileType, OpenMode, Provider};

/// Provides access to a virtual filesystem.
pub struct Virtual {
    root: VirtualEntry,
}

impl Virtual {
    /// Creates a new virtual filesystem.
    pub fn new() -> Virtual {
        Virtual {
            root: VirtualEntry {
                path: PathBuf::from("/"),
                contents: VirtualContents::Directory(VirtualDirectory::new()),
            },
        }
    }
}

struct VirtualEntry {
    path: PathBuf,
    contents: VirtualContents,
}

enum VirtualContents {
    Directory(VirtualDirectory),
    File(VirtualFile),
}

pub struct VirtualDirectory {
    entries: Vec<VirtualEntry>,
}

impl VirtualDirectory {
    fn new() -> VirtualDirectory {
        VirtualDirectory {
            entries: Vec::new(),
        }
    }
}

pub struct VirtualFile {
    contents: RwLock<io::Cursor<Vec<u8>>>,
}

impl VirtualFile {
    fn new() -> VirtualFile {
        VirtualFile {
            contents: RwLock::new(io::Cursor::new(Vec::new())),
        }
    }
}

impl Provider for Virtual {
    type F = File;
    type D = ReadDir;
    type E = DirEntry;
    type M = Metadata;
    type P = Permissions;

    fn open_file_as<P:AsRef<Path>>(path: P, mode: OpenMode) -> io::Result<File> {
        unimplemented!();
    }

    fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
        unimplemented!();
    }

    fn remove_file<P:AsRef<Path>>(path: P) -> io::Result<()> {
        unimplemented!();
    }

    fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
        unimplemented!();
    }

    fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
        unimplemented!();
    }

    fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
        unimplemented!();
    }

    fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
        unimplemented!();
    }

    fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
        unimplemented!();
    }

    fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
        unimplemented!();
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
        unimplemented!();
    }

    fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> io::Result<()> {
        unimplemented!();
    }

    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
        unimplemented!();
    }

    fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        unimplemented!();
    }

    fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
        unimplemented!();
    }
}

/// A virtual file.
pub struct File;

impl filesystem::File for File {
    fn sync_all(&self) -> io::Result<()> {
        unimplemented!();
    }

    fn sync_data(&self) -> io::Result<()> {
        unimplemented!();
    }
}

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!();
    }
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!();
    }
}

impl io::Seek for File {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        unimplemented!();
    }
}

pub struct ReadDir;

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        unimplemented!();
    }
}

pub struct DirEntry;

impl filesystem::DirEntry for DirEntry {
    type M = Metadata;

    fn path(&self) -> PathBuf {
        unimplemented!();
    }

    fn metadata(&self) -> io::Result<Metadata> {
        unimplemented!();
    }

    fn file_type(&self) -> io::Result<FileType> {
        unimplemented!();
    }

    fn file_name(&self) -> ffi::OsString {
        unimplemented!();
    }
}

pub struct Metadata;

impl filesystem::Metadata for Metadata {
    type P = Permissions;

    fn file_type(&self) -> FileType {
        unimplemented!();
    }

    fn is_dir(&self) -> bool {
        unimplemented!();
    }

    fn is_file(&self) -> bool {
        unimplemented!();
    }

    fn len(&self) -> u64 {
        unimplemented!();
    }

    fn permissions(&self) -> Permissions {
        unimplemented!();
    }
}

pub struct Permissions;

impl filesystem::Permissions for Permissions {
    fn readonly(&self) -> bool {
        unimplemented!();
    }
}
