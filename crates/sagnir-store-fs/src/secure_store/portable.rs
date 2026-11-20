use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub(crate) struct SecureStore(PathBuf);

impl SecureStore {
    pub(crate) fn open(path: &Path) -> io::Result<Self> {
        crate::check_root_path(path)?;
        crate::check_root_path(&path.canonicalize()?)?;
        Err(unsupported())
    }

    pub(crate) fn into_root_path(self) -> PathBuf {
        self.0
    }

    pub(crate) fn ensure_directory(&self, _: &str) -> io::Result<()> {
        Err(unsupported())
    }

    pub(crate) fn create_new_file(&self, _: &str) -> io::Result<fs::File> {
        Err(unsupported())
    }

    pub(crate) fn open_existing_file(&self, _: &str, _: bool) -> io::Result<Option<fs::File>> {
        Err(unsupported())
    }

    pub(crate) fn remove_file_if_exists(&self, _: &str) -> io::Result<()> {
        Err(unsupported())
    }

    pub(crate) fn commit_file(&self, _: &fs::File, _: &str, _: &str) -> io::Result<()> {
        Err(unsupported())
    }

    pub(crate) fn finish_initialization(&self) -> io::Result<()> {
        Err(unsupported())
    }
}

fn unsupported() -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        "secure store initialization requires a handle-relative platform backend",
    )
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;

    use sagnir_store::STORE_DIR;

    #[test]
    fn stateful_initialization_fails_closed_without_writing() -> io::Result<()> {
        let root =
            std::env::temp_dir().join(format!("sagnir-portable-init-test-{}", std::process::id()));
        fs::create_dir(&root)?;

        let result = super::SecureStore::open(&root);
        assert!(result.is_err());
        let Err(error) = result else {
            return Ok(());
        };
        assert_eq!(error.kind(), io::ErrorKind::Unsupported);
        assert!(error.to_string().contains("handle-relative"));
        assert!(!root.join(STORE_DIR).exists());
        fs::remove_dir(root)?;
        Ok(())
    }
}
