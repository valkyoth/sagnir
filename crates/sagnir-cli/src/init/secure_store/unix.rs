use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path};

use rustix::fd::{AsFd, OwnedFd};
use rustix::fs::{AtFlags, CWD, Mode, OFlags, fchmod, fsync, mkdirat, openat, renameat, unlinkat};

use sagnir::store::STORE_DIR;

const OWNER_DIR: Mode = Mode::RUSR.union(Mode::WUSR).union(Mode::XUSR);
const OWNER_FILE: Mode = Mode::RUSR.union(Mode::WUSR);

pub(crate) struct SecureStore {
    dir: OwnedFd,
}

impl SecureStore {
    pub(crate) fn open(root: &Path) -> io::Result<Self> {
        let root = open_absolute_directory(root)?;
        match mkdirat(&root, STORE_DIR, OWNER_DIR) {
            Ok(()) | Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.into()),
        }

        let dir = open_directory(&root, STORE_DIR).map_err(|_| unsafe_directory_error())?;
        fchmod(&dir, OWNER_DIR).map_err(io::Error::from)?;
        Ok(Self { dir })
    }

    pub(crate) fn ensure_directory(&self, store_path: &str) -> io::Result<()> {
        let name = store_entry_name(store_path)?;
        match mkdirat(&self.dir, name, OWNER_DIR) {
            Ok(()) | Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.into()),
        }

        let child = open_directory(&self.dir, name).map_err(|_| unsafe_directory_error())?;
        fchmod(&child, OWNER_DIR).map_err(io::Error::from)
    }

    pub(crate) fn create_new_file(&self, store_path: &str) -> io::Result<fs::File> {
        let name = store_entry_name(store_path)?;
        let flags =
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC;
        let fd = openat(&self.dir, name, flags, OWNER_FILE).map_err(io::Error::from)?;
        fchmod(&fd, OWNER_FILE).map_err(io::Error::from)?;
        let file = fs::File::from(fd);
        validate_regular_file(&file)?;
        Ok(file)
    }

    pub(crate) fn open_existing_file(
        &self,
        store_path: &str,
        writable: bool,
    ) -> io::Result<Option<fs::File>> {
        let name = store_entry_name(store_path)?;
        let access = if writable {
            OFlags::RDWR
        } else {
            OFlags::RDONLY
        };
        let flags = access | OFlags::NOFOLLOW | OFlags::CLOEXEC;
        let fd = match openat(&self.dir, name, flags, Mode::empty()) {
            Ok(fd) => fd,
            Err(rustix::io::Errno::NOENT) => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let file = fs::File::from(fd);
        validate_regular_file(&file)?;
        Ok(Some(file))
    }

    pub(crate) fn remove_file_if_exists(&self, store_path: &str) -> io::Result<()> {
        let name = store_entry_name(store_path)?;
        match unlinkat(&self.dir, name, AtFlags::empty()) {
            Ok(()) | Err(rustix::io::Errno::NOENT) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) fn rename_file(&self, from: &str, to: &str) -> io::Result<()> {
        let from = store_entry_name(from)?;
        let to = store_entry_name(to)?;
        renameat(&self.dir, from, &self.dir, to).map_err(io::Error::from)
    }

    pub(crate) fn sync(&self) -> io::Result<()> {
        fsync(&self.dir).map_err(io::Error::from)
    }
}

fn open_absolute_directory(path: &Path) -> io::Result<OwnedFd> {
    if !path.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Sagnir initialization root must be absolute",
        ));
    }

    let mut dir = open_directory(CWD, Path::new("/"))?;
    for component in path.components() {
        match component {
            Component::RootDir => {}
            Component::Normal(name) => dir = open_directory(&dir, name)?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Sagnir initialization root is not normalized",
                ));
            }
        }
    }
    Ok(dir)
}

fn open_directory<Fd: AsFd, P: rustix::path::Arg>(parent: Fd, path: P) -> io::Result<OwnedFd> {
    let flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC;
    openat(parent, path, flags, Mode::empty()).map_err(io::Error::from)
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

fn validate_regular_file(file: &fs::File) -> io::Result<()> {
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Sagnir store entry is not a regular file",
        ));
    }
    if metadata.nlink() != 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "refusing to use a multiply linked Sagnir store file",
        ));
    }
    Ok(())
}

fn unsafe_directory_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::AlreadyExists,
        "refusing to initialize through a symlink or non-directory entry",
    )
}
