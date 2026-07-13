use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

use sagnir::store::STORE_DIR;

pub(crate) struct SecureStore {
    path: PathBuf,
}

impl SecureStore {
    pub(crate) fn open(root: &Path) -> io::Result<Self> {
        let path = root.join(STORE_DIR);
        ensure_real_directory(&path)?;
        Ok(Self { path })
    }

    pub(crate) fn ensure_directory(&self, store_path: &str) -> io::Result<()> {
        ensure_real_directory(&self.path.join(store_entry_name(store_path)?))
    }

    pub(crate) fn create_new_file(&self, store_path: &str) -> io::Result<fs::File> {
        let path = self.path.join(store_entry_name(store_path)?);
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
    }

    pub(crate) fn open_existing_file(
        &self,
        store_path: &str,
        writable: bool,
    ) -> io::Result<Option<fs::File>> {
        let path = self.path.join(store_entry_name(store_path)?);
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_file() => metadata,
            Ok(_) => return Err(unsafe_file_error()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error),
        };
        let file = fs::OpenOptions::new()
            .read(true)
            .write(writable)
            .open(&path)?;
        if !file.metadata()?.is_file() || !metadata.is_file() {
            return Err(unsafe_file_error());
        }
        Ok(Some(file))
    }

    pub(crate) fn remove_file_if_exists(&self, store_path: &str) -> io::Result<()> {
        let path = self.path.join(store_entry_name(store_path)?);
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub(crate) fn rename_file(&self, from: &str, to: &str) -> io::Result<()> {
        fs::rename(
            self.path.join(store_entry_name(from)?),
            self.path.join(store_entry_name(to)?),
        )
    }

    pub(crate) fn sync(&self) -> io::Result<()> {
        Ok(())
    }
}

fn ensure_real_directory(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(()),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "refusing to initialize through a symlink or non-directory entry",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => fs::create_dir(path),
        Err(error) => Err(error),
    }
}

fn store_entry_name(path: &str) -> io::Result<&OsStr> {
    let mut components = Path::new(path).components();
    match (components.next(), components.next(), components.next()) {
        (Some(Component::Normal(store)), Some(Component::Normal(name)), None)
            if store == OsStr::new(STORE_DIR) =>
        {
            Ok(name)
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Sagnir store operation requires one fixed relative entry",
        )),
    }
}

fn unsafe_file_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "Sagnir store entry is not a regular file",
    )
}
