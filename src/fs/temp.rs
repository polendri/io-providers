use std::env;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use tempfile::{tempdir, TempDir};

use fs::{Fs, OpenOptions};

/// Provides access to file I/O in a chroot-like temporary filesystem, located in the system's
/// default temp directory. This temporary directory acts like the root of the filesystem: all
/// absolute paths are relative to it, and any path which would traverse out of it is considered
/// invalid.
///
/// This is NOT intended to act as a secure sandbox; while it ought to handle edge cases such as
/// path traversals and symbolic links correctly, no attempt has been made to verify that there
/// is no way to circumvent this.
#[derive(Debug)]
pub struct TempFs {
    temp_dir: TempDir,
}

impl TempFs {
    /// Creates a new `TempFs`.
    pub fn new() -> io::Result<TempFs> {
        Ok(TempFs {
            temp_dir: tempdir()?,
        })
    }

    /// Returns the path to the root of this temporary filesystem.
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    fn change_path<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        let path = path.as_ref();
        let absolute_path = if path.is_absolute() {
            path.to_owned()
        } else {
            env::current_dir()?.join(path)
        };
        let rerooted_path = self
            .temp_dir
            .path()
            .join(absolute_path.strip_prefix("/").unwrap());

        let result = Self::canonicalize(rerooted_path)?;

        if result.starts_with(self.temp_dir.path())
            && !Self::is_traversal(result.strip_prefix(self.temp_dir.path()).unwrap())
        {
            Ok(result)
        } else {
            Err(Self::invalid_path_err())
        }
    }

    /// `canonicalize` implementation that works on non-existent paths. It attempts to canonicalize
    /// the first ancestor of the path (including itself) that exists, appending the remaining
    /// non-existing portion of the path onto the result.
    fn canonicalize<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
        let path = path.as_ref();
        path.ancestors()
            .filter_map(|a| {
                fs::canonicalize(a)
                    .ok()
                    .and_then(|c| if a == c { None } else { Some((a, c)) })
            }).next()
            .map(|(a, c)| path.strip_prefix(a).map(|s| c.join(s)))
            .unwrap_or_else(|| Ok(path.to_owned()))
            .map_err(|_| Self::invalid_path_err())
    }

    fn is_traversal<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref()
            .components()
            .try_fold(0, |depth, c| {
                let depth = match c {
                    Component::Prefix(_) | Component::RootDir | Component::CurDir => depth,
                    Component::ParentDir => depth - 1,
                    Component::Normal(_) => depth + 1,
                };

                if depth < 0 {
                    None
                } else {
                    Some(depth)
                }
            }).map_or(true, |_| false)
    }

    fn invalid_path_err() -> io::Error {
        io::Error::new(io::ErrorKind::Other, "Invalid path")
    }
}

impl Fs for TempFs {
    fn open<P: AsRef<Path>>(
        &mut self,
        path: P,
        open_options: &OpenOptions,
    ) -> io::Result<fs::File> {
        open_options.as_std().open(self.change_path(path)?)
    }

    fn canonicalize<P: AsRef<Path>>(&self, path: P) -> io::Result<PathBuf> {
        fs::canonicalize(self.change_path(path)?)
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> io::Result<u64> {
        fs::copy(self.change_path(from)?, self.change_path(to)?)
    }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::create_dir(self.change_path(path)?)
    }

    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::create_dir_all(self.change_path(path)?)
    }

    fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, src: P, dst: Q) -> io::Result<()> {
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

    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::remove_dir(self.change_path(path)?)
    }

    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::remove_dir_all(self.change_path(path)?)
    }

    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        fs::remove_file(self.change_path(path)?)
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> io::Result<()> {
        fs::rename(self.change_path(from)?, self.change_path(to)?)
    }

    fn set_permissions<P: AsRef<Path>>(
        &mut self,
        path: P,
        perm: fs::Permissions,
    ) -> io::Result<()> {
        fs::set_permissions(self.change_path(path)?, perm)
    }

    fn symlink_metadata<P: AsRef<Path>>(&self, path: P) -> io::Result<fs::Metadata> {
        fs::symlink_metadata(self.change_path(path)?)
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, contents: C) -> io::Result<()> {
        fs::write(self.change_path(path)?, contents)
    }

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.change_path(path).map(|p| p.exists()).unwrap_or(false)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    use std::env;

    #[test]
    fn change_path__absolute_path_to_file_in_root() {
        let fs = TempFs::new().unwrap();
        let expected = fs.temp_dir.path().join("test.txt");

        let result = fs.change_path("/test.txt");

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn change_path__absolute_path_to_nested_file() {
        let fs = TempFs::new().unwrap();
        let expected = fs.temp_dir.path().join("foo/bar/test.txt");

        let result = fs.change_path("/foo/bar/test.txt");

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn change_path__relative_path_to_file_in_current_dir() {
        let fs = TempFs::new().unwrap();
        let expected = fs
            .temp_dir
            .path()
            .join(env::current_dir().unwrap().strip_prefix("/").unwrap())
            .join("test.txt");

        let result = fs.change_path("test.txt");

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn change_path__relative_path_to_nested_file() {
        let fs = TempFs::new().unwrap();
        let expected = fs
            .temp_dir
            .path()
            .join(env::current_dir().unwrap().strip_prefix("/").unwrap())
            .join("foo/bar/test.txt");

        let result = fs.change_path("foo/bar/test.txt");

        assert_eq!(expected, result.unwrap());
    }
}
