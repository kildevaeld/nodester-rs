
use std::path::{Path, PathBuf};
use std::fs;
use super::error::{NodError,Result};
pub struct WorkDir {
    root: PathBuf,
}

impl WorkDir {
    pub fn new<P: AsRef<Path>>(path: P) -> WorkDir {
        WorkDir { root: path.as_ref().to_owned() }
    }

    pub(crate) fn ensure_path(&self, path: &PathBuf) -> Result<()> {
        if !path.is_dir() {
            if path.is_file() {
                return Err(NodError::Other("path is a file"));
            }

            fs::create_dir_all(&path)?;
        }

        Ok(())
    }

    pub fn ensure(&mut self) -> Result<()> {

        self.ensure_path(&self.root)?;
        self.ensure_path(&self.cache())?;
        self.ensure_path(&self.destination())?;

        if !self.root.is_absolute() {
            self.root = fs::canonicalize(&self.root)?;
        }

        Ok(())
    }

    pub fn cache(&self) -> PathBuf {
        self.path("cache")
    }

    pub fn destination(&self) -> PathBuf {
        self.path("node")
    }

    pub fn current(&self) -> PathBuf {
        self.path("current")
    }

    fn path(&self, end: &str) -> PathBuf {
        let mut cache = self.root.clone();
        cache.push(end);
        cache
    }

}