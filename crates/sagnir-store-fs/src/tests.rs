use sagnir_store::{FORMAT_FILE, FORMAT_TEMP_FILE};
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn open_store_rejects_namespace_replacement() -> std::io::Result<()> {
    let root = TempRoot::new("store-namespace-replacement")?;
    let store = super::SecureStore::open(root.path())?;
    fs::rename(
        root.path().join(".saga"),
        root.path().join("original-store"),
    )?;
    create_dir(root.path().join(".saga"))?;

    let result = store.ensure_directory(".saga/objects");
    assert!(result.is_err());
    let Err(error) = result else { return Ok(()) };

    assert!(error.to_string().contains("namespace changed"));
    assert!(
        fs::read_dir(root.path().join("original-store"))?
            .next()
            .is_none()
    );
    assert!(fs::read_dir(root.path().join(".saga"))?.next().is_none());
    Ok(())
}

#[test]
fn store_rejects_replaced_temporary_file_before_commit() -> std::io::Result<()> {
    let root = TempRoot::new("replaced-temporary-file")?;
    let store = super::SecureStore::open(root.path())?;
    let temp = store.create_new_file(FORMAT_TEMP_FILE)?;
    fs::rename(
        root.path().join(FORMAT_TEMP_FILE),
        root.path().join(".saga/stolen-temp"),
    )?;
    write_file(root.path().join(FORMAT_TEMP_FILE), b"replacement")?;

    let result = store.commit_file(&temp, FORMAT_TEMP_FILE, FORMAT_FILE);
    assert!(result.is_err());
    let Err(error) = result else { return Ok(()) };

    assert!(error.to_string().contains("replaced before commit"));
    assert!(!root.path().join(FORMAT_FILE).exists());
    assert_eq!(
        fs::read(root.path().join(FORMAT_TEMP_FILE))?,
        b"replacement"
    );
    Ok(())
}

pub(crate) struct TempRoot {
    path: PathBuf,
}

impl TempRoot {
    pub(crate) fn new(name: &str) -> std::io::Result<Self> {
        let path =
            std::env::temp_dir().join(format!("sagnir-fs-test-{}-{name}", std::process::id()));
        create_dir(&path)?;
        Ok(Self { path })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn create_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    fs::create_dir(path)
}

fn write_file<P: AsRef<Path>>(path: P, content: &[u8]) -> std::io::Result<()> {
    fs::write(path, content)
}
