use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use sagnir::store::{
    CONFIG_FILE, FORMAT_FILE, FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES,
    INIT_LOCK_FILE, REALM_FILE, STORE_DIR,
};

use crate::{CliOutput, runtime_error, sanitize_for_display, unexpected_argument};

const FORMAT_FILE_READ_MAX: usize = FORMAT_FILE_CONTENT.len() + 1;
const INIT_LOCK_PREFIX: &str = "sagnir-init-lock-v1\npid=";

mod metadata;
mod secure_store;

use secure_store::SecureStore;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StoreInitStatus {
    Created,
    AlreadyInitialized,
}

pub(crate) fn init_command<I, S>(mut args: I, cwd: &Path) -> CliOutput
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    match args.next() {
        None => init_store(cwd),
        Some(arg) if arg.as_ref() == "--dry-run" => {
            if let Some(extra) = args.next() {
                return unexpected_argument("--dry-run", extra.as_ref());
            }
            init_dry_run(cwd)
        }
        Some(arg) => unexpected_argument("init", arg.as_ref()),
    }
}

fn init_dry_run(cwd: &Path) -> CliOutput {
    let mut stdout = format!(
        "saga init --dry-run\nRoot: {}\n\nDirectories:\n",
        sanitize_path(cwd)
    );
    for dir in INIT_DIRECTORIES {
        stdout.push_str("  ");
        stdout.push_str(dir);
        stdout.push('\n');
    }
    stdout.push_str("\nFiles:\n");
    for file in [FORMAT_FILE, REALM_FILE, CONFIG_FILE] {
        stdout.push_str("  ");
        stdout.push_str(file);
        stdout.push('\n');
    }
    stdout.push_str("\nNo changes written.\n");

    CliOutput {
        code: 0,
        stdout,
        stderr: String::new(),
    }
}

fn init_store(cwd: &Path) -> CliOutput {
    let root = match validated_root(cwd) {
        Ok(root) => root,
        Err(error) => {
            return runtime_error(&format!(
                "saga init failed: {}",
                sanitize_for_display(&error.to_string())
            ));
        }
    };

    match create_store_layout(&root) {
        Ok(StoreInitStatus::Created) => init_success("Initialized Sagnir store", &root),
        Ok(StoreInitStatus::AlreadyInitialized) => {
            init_success("Sagnir store already initialized", &root)
        }
        Err(error) => runtime_error(&format!(
            "saga init failed: {}",
            sanitize_for_display(&error.to_string())
        )),
    }
}

fn init_success(message: &str, cwd: &Path) -> CliOutput {
    CliOutput {
        code: 0,
        stdout: format!(
            "{message}\nRoot: {}\nStore: {}\nFormat: {}\n",
            sanitize_path(cwd),
            STORE_DIR,
            FORMAT_FILE
        ),
        stderr: String::new(),
    }
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

    if already_initialized {
        metadata::ensure_store_metadata(&store)?;
        store.verify_attached()?;
        return Ok(StoreInitStatus::AlreadyInitialized);
    }

    write_format_file(&store)?;
    metadata::ensure_store_metadata(&store)?;
    store.verify_attached()?;
    Ok(StoreInitStatus::Created)
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

fn sanitize_path(path: &Path) -> String {
    sanitize_for_display(&path.display().to_string())
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
