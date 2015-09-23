use std::io;
use std::path::{Path, PathBuf};
use env;

/// Provides access to a virtual environment, which can be configured independently from the
/// local system.
pub struct Virtual {
    args: Vec<String>,
    current_dir: PathBuf,
}

impl Virtual {
    /// Creates a new virtual environment.
    pub fn new() -> Virtual {
        Virtual {
            args: Vec::new(),
            current_dir: PathBuf::from("/"),
        }
    }

    /// Sets the arguments.
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }
}

impl env::Provider for Virtual {
    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

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

    #[test]
    fn args__default__returns_empty() {
        let provider = Virtual::new();
        let result = provider.args();
        assert_eq!(0, result.len());
    }

    #[test]
    fn args__set_and_get__success() {
        let mut provider = Virtual::new();
        let args = vec!["app".to_string(), "arg1".to_string(), "arg2".to_string()];

        provider.set_args(args.clone());
        let result = provider.args();

        assert_eq!(args, result);
    }
}
