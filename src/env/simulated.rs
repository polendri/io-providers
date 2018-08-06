use std::io;
use std::path::{Path, PathBuf};
use env::Env;

/// Provides inspection and manipulation of a simulated process's environment.
pub struct SimulatedEnv {
    args: Vec<String>,
    current_dir: PathBuf,
}

impl SimulatedEnv {
    /// Creates a new virtual environment.
    pub fn new() -> SimulatedEnv {
        SimulatedEnv {
            args: Vec::new(),
            current_dir: PathBuf::from("/"),
        }
    }

    /// Sets the arguments.
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }
}

impl Env for SimulatedEnv {
    fn args(&self) -> Vec<String> {
        self.args.clone()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        Ok(self.current_dir.clone())
    }

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.current_dir = PathBuf::from(path.as_ref());
        Ok(())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::path::{Path, PathBuf};
    use super::SimulatedEnv;
    use env::Env;

    #[test]
    fn current_dir__default__returns_root() {
        let provider = SimulatedEnv::new();
        let result = provider.current_dir().unwrap();
        assert_eq!(PathBuf::from("/"), result);
    }

    #[test]
    fn current_dir__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let path = Path::new("/foo/bar");

        provider.set_current_dir(path).unwrap();
        let result = provider.current_dir().unwrap();

        assert_eq!(path, result.as_path());
    }

    #[test]
    fn args__default__returns_empty() {
        let provider = SimulatedEnv::new();
        let result = provider.args();
        assert_eq!(0, result.len());
    }

    #[test]
    fn args__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let args = vec!["app".to_string(), "arg1".to_string(), "arg2".to_string()];

        provider.set_args(args.clone());
        let result = provider.args();

        assert_eq!(args, result);
    }
}
