use std::io;
use std::path::{Path, PathBuf};
use env;

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

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.current_dir = PathBuf::from(path.as_ref());
        Ok(())
    }
}
