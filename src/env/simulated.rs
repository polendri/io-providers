use std::collections::HashMap;
use std::env;
use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::vec;

use env::Env;

/// Provides inspection and manipulation of a simulated process's environment.
#[derive(Default)]
pub struct SimulatedEnv {
    args: Option<Vec<String>>,
    args_os: Option<Vec<ffi::OsString>>,
    current_dir: Option<PathBuf>,
    current_exe: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    vars: HashMap<ffi::OsString, ffi::OsString>,
}

impl SimulatedEnv {
    /// Creates a new virtual environment.
    pub fn new() -> SimulatedEnv {
        SimulatedEnv {
            args: None,
            args_os: None,
            current_dir: None,
            current_exe: None,
            home_dir: None,
            vars: HashMap::new(),
        }
    }

    /// Sets the arguments which this program was started with (normally passed via the command
    /// line).
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = Some(args);
    }

    /// Sets the arguments which this program was started with (normally passed via the command
    /// line).
    pub fn set_args_os(&mut self, args: Vec<ffi::OsString>) {
        self.args_os = Some(args);
    }

    /// Sets the path to be returned by `Env::current_exe()`.
    pub fn set_current_exe<P: AsRef<Path>>(&mut self, path: P) {
        self.current_dir = Some(PathBuf::from(path.as_ref()));
    }

    /// Sets the path to be returned by `Env::home_dir()`.
    pub fn set_home_dir<P: AsRef<Path>>(&mut self, path: Option<P>) {
        self.home_dir = path.map(|p| PathBuf::from(p.as_ref()));
    }
}

impl Env for SimulatedEnv {
    type ArgsIter = vec::IntoIter<String>;
    type ArgsOsIter = vec::IntoIter<ffi::OsString>;
    type VarsIter = vec::IntoIter<(String, String)>;

    fn args(&self) -> Self::ArgsIter {
        self.args
            .clone()
            .expect("Env::args() was called before a simulated value was set")
            .into_iter()
    }

    fn args_os(&self) -> Self::ArgsOsIter {
        self.args_os
            .clone()
            .expect("Env::args_os() was called before a simulated value was set")
            .into_iter()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        Ok(self
            .current_dir
            .clone()
            .expect("Env::current_dir() was called before a simulated value was set"))
    }

    fn current_exe(&self) -> io::Result<PathBuf> {
        Ok(self
            .current_exe
            .clone()
            .expect("Env::current_exe() was called before a simulated value was set"))
    }

    fn home_dir(&self) -> Option<PathBuf> {
        self.home_dir.clone()
    }

    fn remove_var<K: AsRef<ffi::OsStr>>(&mut self, k: K) {
        self.vars.remove(k.as_ref());
    }

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.current_dir = Some(PathBuf::from(path.as_ref()));
        Ok(())
    }

    fn set_var<K: AsRef<ffi::OsStr>, V: AsRef<ffi::OsStr>>(&mut self, k: K, v: V) {
        let _ = self
            .vars
            .insert(k.as_ref().to_os_string(), v.as_ref().to_os_string());
    }

    fn var<K: AsRef<ffi::OsStr>>(&self, key: K) -> Result<String, env::VarError> {
        self.vars
            .get(&key.as_ref().to_os_string())
            .ok_or(env::VarError::NotPresent)
            .and_then(|k| k.clone().into_string().map_err(env::VarError::NotUnicode))
    }

    fn vars(&self) -> Self::VarsIter {
        self.vars
            .iter()
            .map(|(k, v)| {
                (
                    k.clone().into_string().unwrap(),
                    v.clone().into_string().unwrap(),
                )
            }).collect::<Vec<(String, String)>>()
            .into_iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::env;
    use std::ffi::OsString;
    use std::path::Path;

    use super::SimulatedEnv;
    use env::Env;

    #[test]
    #[should_panic]
    fn args__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.args();
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
    #[should_panic]
    fn args_os__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.args_os();
    }

    #[test]
    fn args_os__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let args = vec![
            OsString::from("app"),
            OsString::from("arg1"),
            OsString::from("arg2"),
        ];

        provider.set_args_os(args.clone());
        let result: Vec<OsString> = provider.args_os().collect();

        assert_eq!(args, result);
    }

    #[test]
    #[should_panic]
    fn current_dir__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.current_dir();
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
    #[should_panic]
    fn current_exe__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.current_exe();
    }

    #[test]
    fn current_exe__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let path = Path::new("/foo/bar");

        provider.set_current_exe(path);
        let result = provider.current_dir().unwrap();

        assert_eq!(path, result.as_path());
    }

    #[test]
    fn home_dir__called_before_set__returns_none() {
        let provider = SimulatedEnv::new();
        let result = provider.home_dir();

        assert!(result.is_none());
    }

    #[test]
    fn home_dir__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let path = Path::new("/foo/bar");

        provider.set_home_dir(Some(path));
        let result = provider.home_dir().unwrap();

        assert_eq!(path, result.as_path());
    }

    #[test]
    fn var__get_undefined_var__returns_not_present() {
        let provider = SimulatedEnv::new();

        let result = provider.var("FOO");

        assert_eq!(Err(env::VarError::NotPresent), result);
    }

    #[test]
    fn var__get_defined_var__returns_value() {
        let mut provider = SimulatedEnv::new();
        provider.set_var("FOO", "bar");

        let result = provider.var("FOO");

        assert_eq!(Ok("bar".to_owned()), result);
    }

    #[test]
    fn remove_var__value_previously_defined__value_is_removed() {
        let mut provider = SimulatedEnv::new();
        provider.set_var("FOO", "bar");

        provider.remove_var("FOO");

        let result = provider.var("FOO");
        assert_eq!(Err(env::VarError::NotPresent), result);
    }

    #[test]
    fn vars__multiple_vars_defined__returns_all_vars() {
        let mut provider = SimulatedEnv::new();
        provider.set_var("FOO", "bar");
        provider.set_var("ABC", "123");

        let result: Vec<(String, String)> = provider.vars().collect();

        assert_eq!(2, result.len());
        assert!(result.contains(&("FOO".to_owned(), "bar".to_owned())));
        assert!(result.contains(&("ABC".to_owned(), "123".to_owned())));
    }
}
