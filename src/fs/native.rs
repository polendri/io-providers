use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use fs::{Fs, OpenOptions};

/// Provides access to native file I/O.
#[derive(Debug, Default)]
pub struct NativeFs;

impl Fs for NativeFs {
    fn open<P: AsRef<Path>>(
        &mut self,
        path: P,
        open_options: &OpenOptions,
    ) -> io::Result<fs::File> {
        open_options.as_std().open(path)
    }

    fn canonicalize<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        fs::canonicalize(path)
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> io::Result<u64> {
        fs::copy(from, to)
    }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::create_dir(path)
    }

    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, src: P, dst: Q) -> io::Result<()> {
        fs::hard_link(src, dst)
    }

    fn metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::Metadata> {
        fs::metadata(path)
    }

    fn read<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    fn read_dir<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::ReadDir> {
        fs::read_dir(path)
    }

    fn read_link<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        fs::read_link(path)
    }

    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        fs::read_to_string(path)
    }

    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::remove_dir(path)
    }

    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::remove_dir_all(path)
    }

    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> io::Result<()> {
        fs::rename(from, to)
    }

    fn set_permissions<P: AsRef<Path>>(
        &mut self,
        path: P,
        perm: fs::Permissions,
    ) -> io::Result<()> {
        fs::set_permissions(path, perm)
    }

    fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::Metadata> {
        fs::symlink_metadata(path)
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, contents: C) -> io::Result<()> {
        fs::write(path, contents)
    }

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }
}
