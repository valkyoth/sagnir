use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path, PathBuf};

use rustix::fd::{AsFd, OwnedFd};
use rustix::fs::{
    AtFlags, CWD, Dev, Mode, OFlags, fchmod, fstat, fsync, mkdirat, openat, renameat, unlinkat,
};
use rustix::process::geteuid;

use sagnir_store::STORE_DIR;

const OWNER_DIR: Mode = Mode::RUSR.union(Mode::WUSR).union(Mode::XUSR);
const OWNER_FILE: Mode = Mode::RUSR.union(Mode::WUSR);

pub(crate) struct SecureStore {
    canonical_root: PathBuf,
    root_id: FileId,
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
    pub(crate) fn open(path: &Path) -> io::Result<Self> {
        crate::check_root_path(path)?;
        let root = openat(
            CWD,
            path,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )?;
        Self::open_bound(root, path.canonicalize()?, geteuid().as_raw())
    }

    fn open_bound(root: OwnedFd, canonical_root: PathBuf, owner: u32) -> io::Result<Self> {
        crate::check_root_path(&canonical_root)?;
        verify_owner(&root, owner, "Sagnir initialization root")?;
        let root_id = file_id(&root)?;
        verify_root_binding(&canonical_root, root_id)?;
        match mkdirat(&root, STORE_DIR, OWNER_DIR) {
            Ok(()) | Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.into()),
        }

        let dir = open_directory(&root, STORE_DIR).map_err(|_| unsafe_directory_error())?;
        verify_owner(&dir, owner, "Sagnir store")?;
        let store_id = file_id(&dir)?;
        fchmod(&dir, OWNER_DIR).map_err(io::Error::from)?;
        Ok(Self {
            canonical_root,
            root_id,
            root,
            dir,
            store_id,
        })
    }

    pub(crate) fn into_root_path(self) -> PathBuf {
        self.canonical_root
    }

    pub(crate) fn ensure_directory(&self, store_path: &str) -> io::Result<()> {
        self.verify_attached()?;
        let name = store_entry_name(store_path)?;
        match mkdirat(&self.dir, name, OWNER_DIR) {
            Ok(()) | Err(rustix::io::Errno::EXIST) => {}
            Err(error) => return Err(error.into()),
        }

        let child = open_directory(&self.dir, name).map_err(|_| unsafe_directory_error())?;
        verify_owner(&child, geteuid().as_raw(), "Sagnir store directory")?;
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
        let flags = access | OFlags::NONBLOCK | OFlags::NOFOLLOW | OFlags::CLOEXEC;
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

    pub(crate) fn finish_initialization(&self) -> io::Result<()> {
        self.finish_with(|fd| Ok(fsync(fd)?))
    }

    fn finish_with(&self, mut sync: impl FnMut(&OwnedFd) -> io::Result<()>) -> io::Result<()> {
        self.verify_attached()?;
        sync(&self.dir)?;
        sync(&self.root)?;
        self.verify_attached()
    }

    pub(crate) fn verify_attached(&self) -> io::Result<()> {
        verify_root_binding(&self.canonical_root, self.root_id)?;
        let visible =
            open_directory(&self.root, STORE_DIR).map_err(|_| namespace_changed_error())?;
        if file_id(&visible)? != self.store_id {
            return Err(namespace_changed_error());
        }
        Ok(())
    }
}

fn verify_root_binding(path: &Path, expected: FileId) -> io::Result<()> {
    let visible = open_absolute_directory(path).map_err(|_| namespace_changed_error())?;
    if file_id(&visible)? != expected {
        return Err(namespace_changed_error());
    }
    Ok(())
}

fn file_id<Fd: AsFd>(fd: Fd) -> io::Result<FileId> {
    let stat = fstat(fd).map_err(io::Error::from)?;
    Ok(FileId {
        device: stat.st_dev,
        inode: stat.st_ino,
    })
}

fn verify_owner<Fd: AsFd>(fd: Fd, owner: u32, description: &str) -> io::Result<()> {
    let stat = fstat(fd).map_err(io::Error::from)?;
    if stat.st_uid != owner {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admission_requires_same_directory_at_both_paths() -> io::Result<()> {
        let fixture = crate::tests::TempRoot::new("root-admission")?;
        let other = fixture.path().join("other");
        fs::create_dir(&other)?;
        let supplied = open_absolute_directory(&fixture.path().canonicalize()?)?;
        assert!(
            SecureStore::open_bound(supplied, other.canonicalize()?, geteuid().as_raw()).is_err()
        );
        assert!(!fixture.path().join(STORE_DIR).exists());
        assert!(!other.join(STORE_DIR).exists());
        Ok(())
    }

    #[test]
    fn root_owner_is_checked_before_creating_store() -> io::Result<()> {
        let fixture = crate::tests::TempRoot::new("owner-admission")?;
        let path = fixture.path().canonicalize()?;
        let supplied = open_absolute_directory(&path)?;
        let different_owner = fstat(&supplied)?.st_uid ^ 1;
        let result = SecureStore::open_bound(supplied, path, different_owner);
        assert!(matches!(result, Err(e) if e.kind() == io::ErrorKind::PermissionDenied));
        assert!(!fixture.path().join(STORE_DIR).exists());
        Ok(())
    }

    #[test]
    fn final_attachment_requires_original_project_root() -> io::Result<()> {
        let fixture = crate::tests::TempRoot::new("final-root-binding")?;
        let project = fixture.path().join("project");
        fs::create_dir(&project)?;
        let store = SecureStore::open(&project)?;
        let mut calls = 0;
        let result = store.finish_with(|_| {
            calls += 1;
            if calls == 2 {
                fs::rename(&project, fixture.path().join("saved"))?;
                fs::create_dir(&project)?;
            }
            Ok(())
        });
        assert_eq!(calls, 2);
        assert!(result.is_err());
        assert!(fs::read_dir(project)?.next().is_none());
        Ok(())
    }

    #[test]
    fn final_sync_orders_store_before_parent_and_propagates_failure() -> io::Result<()> {
        let root = crate::tests::TempRoot::new("parent-sync")?;
        let store = SecureStore::open(root.path())?;
        let parent = fs::File::open(root.path())?;
        let child = fs::File::open(root.path().join(STORE_DIR))?;
        let mut calls = Vec::new();
        let result = store.finish_with(|fd| {
            calls.push(file_id(fd)?);
            if calls.len() == 2 {
                return Err(io::Error::other("injected parent sync failure"));
            }
            Ok(())
        });
        assert!(result.is_err());
        assert_eq!(calls, [file_id(&child)?, file_id(&parent)?]);
        store.finish_initialization()
    }
}
