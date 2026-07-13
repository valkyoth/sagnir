use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path};

use rustix::fd::{AsFd, OwnedFd};
use rustix::fs::{
    AtFlags, CWD, Dev, Mode, OFlags, fchmod, fstat, fsync, mkdirat, openat, renameat, unlinkat,
};
use rustix::process::geteuid;

use sagnir::store::STORE_DIR;

const OWNER_DIR: Mode = Mode::RUSR.union(Mode::WUSR).union(Mode::XUSR);
const OWNER_FILE: Mode = Mode::RUSR.union(Mode::WUSR);

pub(crate) struct SecureStore {
    root: OwnedFd,
    dir: OwnedFd,
    store_id: FileId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FileId {
    device: Dev,
    inode: u64,
}

impl SecureStore {
    pub(crate) fn open(root: &Path) -> io::Result<Self> {
        let root = open_absolute_directory(root)?;
        match mkdirat(&root, STORE_DIR, OWNER_DIR) {
            Ok(()) | Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.into()),
        }

        let dir = open_directory(&root, STORE_DIR).map_err(|_| unsafe_directory_error())?;
        verify_owner(&dir, "Sagnir store")?;
        let store_id = file_id(&dir)?;
        fchmod(&dir, OWNER_DIR).map_err(io::Error::from)?;
        Ok(Self {
            root,
            dir,
            store_id,
        })
    }

    pub(crate) fn ensure_directory(&self, store_path: &str) -> io::Result<()> {
        self.verify_attached()?;
        let name = store_entry_name(store_path)?;
        match mkdirat(&self.dir, name, OWNER_DIR) {
            Ok(()) | Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.into()),
        }

        let child = open_directory(&self.dir, name).map_err(|_| unsafe_directory_error())?;
        verify_owner(&child, "Sagnir store directory")?;
        fchmod(&child, OWNER_DIR).map_err(io::Error::from)?;
        self.verify_attached()
    }

    pub(crate) fn create_new_file(&self, store_path: &str) -> io::Result<fs::File> {
        self.verify_attached()?;
        let name = store_entry_name(store_path)?;
        let flags =
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC;
        let fd = openat(&self.dir, name, flags, OWNER_FILE).map_err(io::Error::from)?;
        fchmod(&fd, OWNER_FILE).map_err(io::Error::from)?;
        let file = fs::File::from(fd);
        validate_regular_file(&file)?;
        self.verify_attached()?;
        Ok(file)
    }

    pub(crate) fn open_existing_file(
        &self,
        store_path: &str,
        writable: bool,
    ) -> io::Result<Option<fs::File>> {
        self.verify_attached()?;
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
        self.verify_attached()?;
        Ok(Some(file))
    }

    pub(crate) fn remove_file_if_exists(&self, store_path: &str) -> io::Result<()> {
        self.verify_attached()?;
        let name = store_entry_name(store_path)?;
        match unlinkat(&self.dir, name, AtFlags::empty()) {
            Ok(()) | Err(rustix::io::Errno::NOENT) => {}
            Err(error) => return Err(error.into()),
        }
        self.verify_attached()
    }

    pub(crate) fn commit_file(&self, temp: &fs::File, from: &str, to: &str) -> io::Result<()> {
        let expected = file_id(temp)?;
        self.verify_attached()?;
        let visible_temp = self
            .open_existing_file(from, false)?
            .ok_or_else(|| io::Error::other("temporary file disappeared before commit"))?;
        if file_id(&visible_temp)? != expected {
            return Err(io::Error::other(
                "temporary file was replaced before commit",
            ));
        }

        let from_name = store_entry_name(from)?;
        let to_name = store_entry_name(to)?;
        renameat(&self.dir, from_name, &self.dir, to_name).map_err(io::Error::from)?;

        let committed = self
            .open_existing_file(to, false)?
            .ok_or_else(|| io::Error::other("committed file disappeared"))?;
        if file_id(&committed)? != expected {
            return Err(io::Error::other(
                "temporary file was replaced during commit",
            ));
        }
        self.verify_attached()?;
        fsync(&self.dir).map_err(io::Error::from)?;
        self.verify_attached()
    }

    pub(crate) fn verify_attached(&self) -> io::Result<()> {
        let visible =
            open_directory(&self.root, STORE_DIR).map_err(|_| namespace_changed_error())?;
        if file_id(&visible)? != self.store_id {
            return Err(namespace_changed_error());
        }
        Ok(())
    }
}

fn file_id<Fd: AsFd>(fd: Fd) -> io::Result<FileId> {
    let stat = fstat(fd).map_err(io::Error::from)?;
    Ok(FileId {
        device: stat.st_dev,
        inode: stat.st_ino,
    })
}

fn verify_owner<Fd: AsFd>(fd: Fd, description: &str) -> io::Result<()> {
    let stat = fstat(fd).map_err(io::Error::from)?;
    if stat.st_uid != geteuid().as_raw() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("refusing to initialize a {description} owned by another user"),
        ));
    }
    Ok(())
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
    if metadata.uid() != geteuid().as_raw() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "refusing to use a Sagnir store file owned by another user",
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

fn namespace_changed_error() -> io::Error {
    io::Error::other("Sagnir store namespace changed during initialization")
}
