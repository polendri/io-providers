use std::io;
use std::path::{Path, PathBuf};
use env;

/// Provides access to a virtual environment, which can be configured independently from the
/// local system.
pub struct Virtual {
    current_dir: PathBuf,
}

impl Virtual {
    pub fn new() -> Virtual {
        Virtual {
            current_dir: PathBuf::from("/"),
        }
    }
}

impl env::Provider for Virtual {
    fn current_dir(&self) -> io::Result<PathBuf> {
        Ok(self.current_dir.clone())
    }

    fn set_current_dir(&mut self, path: &Path) -> io::Result<()> {
        self.current_dir = PathBuf::from(path);
        Ok(())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::path::{Path, PathBuf};
    use super::Virtual;
    use env::Provider;

    #[test]
    fn current_dir__default__returns_root() {
        let provider = Virtual::new();
        let result = provider.current_dir().unwrap();
        assert_eq!(PathBuf::from("/"), result);
    }

    #[test]
    fn current_dir__set_and_get__success() {
        let mut provider = Virtual::new();
        let path = Path::new("/foo/bar");

        provider.set_current_dir(path).unwrap();
        let result = provider.current_dir().unwrap();

        assert_eq!(path, result.as_path());
    }
}
