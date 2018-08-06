use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::vec;

use env::Env;

/// Provides inspection and manipulation of a simulated process's environment.
#[derive(Default)]
pub struct SimulatedEnv {
    args: Vec<String>,
    args_os: Vec<ffi::OsString>,
    current_dir: PathBuf,
}

impl SimulatedEnv {
    /// Creates a new virtual environment.
    pub fn new() -> SimulatedEnv {
        SimulatedEnv {
            args: Vec::new(),
            args_os: Vec::new(),
            current_dir: PathBuf::from("/"),
        }
    }

    /// Sets the arguments which this program was started with (normally passed via the command
    /// line).
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }

    /// Sets the arguments which this program was started with (normally passed via the command
    /// line).
    pub fn set_args_os(&mut self, args: Vec<ffi::OsString>) {
        self.args_os = args;
    }
}

impl Env for SimulatedEnv {
    type ArgsIter = vec::IntoIter<String>;
    type ArgsOsIter = vec::IntoIter<ffi::OsString>;

    fn args(&self) -> Self::ArgsIter {
        self.args.clone().into_iter()
    }

    fn args_os(&self) -> Self::ArgsOsIter {
        self.args_os.clone().into_iter()
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
    use std::ffi::OsString;
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
        let result: Vec<String> = provider.args().collect();

        assert_eq!(args, result);
    }

    #[test]
    fn args_os__default__returns_empty() {
        let provider = SimulatedEnv::new();
        let result = provider.args_os();
        assert_eq!(0, result.len());
    }

    #[test]
    fn args_os__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let args = vec![OsString::from("app"), OsString::from("arg1"), OsString::from("arg2")];

        provider.set_args_os(args.clone());
        let result: Vec<OsString> = provider.args_os().collect();

        assert_eq!(args, result);
    }
}
