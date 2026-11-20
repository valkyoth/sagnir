#![forbid(unsafe_code)]
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use sagnir_store::{
    FORMAT_FILE, FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES, INIT_LOCK_FILE, STORE_DIR,
};

const FORMAT_FILE_READ_MAX: usize = FORMAT_FILE_CONTENT.len() + 1;
const INIT_LOCK_PREFIX: &str = "sagnir-init-lock-v1\npid=";

mod metadata;
mod secure_store;

use secure_store::SecureStore;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreInitStatus {
    Created,
    AlreadyInitialized,
}

/// Initializes or validates the store. The returned path is for display only.
pub fn initialize(cwd: &Path) -> io::Result<(StoreInitStatus, PathBuf)> {
    let root = validated_root(cwd)?;
    let status = create_store_layout(&root)?;
    Ok((status, root))
}

fn create_store_layout(cwd: &Path) -> io::Result<StoreInitStatus> {
    let store = SecureStore::open(cwd)?;
    let _lock = InitLock::acquire(&store)?;

    let already_initialized = existing_format_is_valid(&store)?;

    for dir in INIT_DIRECTORIES {
        if dir != STORE_DIR {
            store.ensure_directory(dir)?;
        }
    }

    store.remove_file_if_exists(FORMAT_TEMP_FILE)?;

    if !already_initialized {
        write_format_file(&store)?;
    }
    metadata::ensure_store_metadata(&store)?;
    store.finish_initialization()?;
    Ok(if already_initialized {
        StoreInitStatus::AlreadyInitialized
    } else {
        StoreInitStatus::Created
    })
}

fn existing_format_is_valid(store: &SecureStore) -> io::Result<bool> {
    let mut file = match store.open_existing_file(FORMAT_FILE, false)? {
        Some(file) => file,
        None => return Ok(false),
    };

    let mut buffer = [0_u8; FORMAT_FILE_CONTENT.len()];
    if let Err(error) = file.read_exact(&mut buffer) {
        return if error.kind() == io::ErrorKind::UnexpectedEof {
            unexpected_format()
        } else {
            Err(error)
        };
    }
    let mut extra = [0_u8; FORMAT_FILE_READ_MAX - FORMAT_FILE_CONTENT.len()];
    let extra_len = file.read(&mut extra)?;
    if extra_len == 0 && buffer == *FORMAT_FILE_CONTENT.as_bytes() {
        Ok(true)
    } else {
        unexpected_format()
    }
}

fn unexpected_format<T>() -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "existing .saga/FORMAT has unexpected content",
    ))
}

fn write_format_file(store: &SecureStore) -> io::Result<()> {
    let mut temp_file = match store.create_new_file(FORMAT_TEMP_FILE) {
        Ok(file) => file,
        Err(error) => {
            let _ = store.remove_file_if_exists(FORMAT_TEMP_FILE);
            return Err(error);
        }
    };

    if let Err(error) = temp_file
        .write_all(FORMAT_FILE_CONTENT.as_bytes())
        .and_then(|()| temp_file.sync_all())
    {
        let _ = store.remove_file_if_exists(FORMAT_TEMP_FILE);
        return Err(error);
    }
    if let Err(error) = store.commit_file(&temp_file, FORMAT_TEMP_FILE, FORMAT_FILE) {
        let _ = store.remove_file_if_exists(FORMAT_TEMP_FILE);
        return Err(error);
    }
    Ok(())
}

struct InitLock {
    _file: fs::File,
}

impl InitLock {
    fn acquire(store: &SecureStore) -> io::Result<Self> {
        let (mut file, created) = open_init_lock(store)?;
        match file.try_lock() {
            Ok(()) => {}
            Err(fs::TryLockError::WouldBlock) => {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "another saga init is already running",
                ));
            }
            Err(fs::TryLockError::Error(error)) => return Err(error),
        };
        if created {
            write_init_lock_content(&mut file)?;
        }
        Ok(Self { _file: file })
    }
}

fn validated_root(cwd: &Path) -> io::Result<PathBuf> {
    if is_restricted_init_root(cwd) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "refusing to initialize a system directory",
        ));
    }
    let canonical = cwd.canonicalize()?;
    if is_restricted_init_root(&canonical) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "refusing to initialize a system directory",
        ));
    }
    Ok(canonical)
}

fn is_restricted_init_root(cwd: &Path) -> bool {
    #[cfg(unix)]
    {
        const RESTRICTED: [&str; 23] = [
            "/",
            "/bin",
            "/boot",
            "/dev",
            "/etc",
            "/lib",
            "/lib64",
            "/proc",
            "/root",
            "/run",
            "/sbin",
            "/sys",
            "/tmp",
            "/usr",
            "/var",
            "/Applications",
            "/Library",
            "/System",
            "/private",
            "/private/etc",
            "/private/root",
            "/private/tmp",
            "/private/var",
        ];
        RESTRICTED.iter().any(|path| cwd == Path::new(path))
    }

    #[cfg(not(unix))]
    {
        cwd.parent().is_none()
    }
}

fn open_init_lock(store: &SecureStore) -> io::Result<(fs::File, bool)> {
    match store.create_new_file(INIT_LOCK_FILE) {
        Ok(file) => Ok((file, true)),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => store
            .open_existing_file(INIT_LOCK_FILE, true)?
            .map(|file| (file, false))
            .ok_or_else(|| io::Error::other("init lock disappeared while opening")),
        Err(error) => Err(error),
    }
}

fn write_init_lock_content(file: &mut fs::File) -> io::Result<()> {
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    writeln!(file, "{}{}", INIT_LOCK_PREFIX, std::process::id())?;
    file.sync_all()
}

#[cfg(all(test, unix))]
mod tests;
