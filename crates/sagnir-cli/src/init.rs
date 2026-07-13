use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};

use sagnir::store::{
    CONFIG_FILE, FORMAT_FILE, FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES,
    INIT_LOCK_FILE, REALM_FILE, STORE_DIR,
};

use crate::{CliOutput, runtime_error, sanitize_for_display, unexpected_argument};

const FORMAT_FILE_READ_MAX: usize = FORMAT_FILE_CONTENT.len() + 1;
const INIT_LOCK_PREFIX: &str = "sagnir-init-lock-v1\npid=";
const INIT_LOCK_READ_MAX: usize = 96;

mod metadata;

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
    if is_restricted_init_root(cwd) {
        return runtime_error("saga init failed: refusing to initialize a system directory");
    }

    match create_store_layout(cwd) {
        Ok(StoreInitStatus::Created) => init_success("Initialized Sagnir store", cwd),
        Ok(StoreInitStatus::AlreadyInitialized) => {
            init_success("Sagnir store already initialized", cwd)
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
    create_secure_dir(&cwd.join(STORE_DIR))?;
    let _lock = InitLock::acquire(cwd)?;

    let format_path = cwd.join(FORMAT_FILE);
    let already_initialized = existing_format_is_valid(&format_path)?;

    for dir in INIT_DIRECTORIES {
        create_secure_dir(&cwd.join(dir))?;
    }

    let temp_path = cwd.join(FORMAT_TEMP_FILE);
    remove_stale_temp(&temp_path)?;

    if already_initialized {
        metadata::ensure_store_metadata(cwd)?;
        return Ok(StoreInitStatus::AlreadyInitialized);
    }

    write_format_file(&format_path, &temp_path)?;
    metadata::ensure_store_metadata(cwd)?;
    Ok(StoreInitStatus::Created)
}

fn existing_format_is_valid(path: &Path) -> io::Result<bool> {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error),
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

pub(super) fn remove_stale_temp(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn write_format_file(format_path: &Path, temp_path: &Path) -> io::Result<()> {
    let mut temp_file = match secure_new_file(temp_path) {
        Ok(file) => file,
        Err(error) => {
            let _ = remove_stale_temp(temp_path);
            return Err(error);
        }
    };

    if let Err(error) = temp_file
        .write_all(FORMAT_FILE_CONTENT.as_bytes())
        .and_then(|()| temp_file.sync_all())
    {
        let _ = remove_stale_temp(temp_path);
        return Err(error);
    }
    drop(temp_file);

    if let Err(error) = fs::rename(temp_path, format_path) {
        let _ = remove_stale_temp(temp_path);
        return Err(error);
    }

    Ok(())
}

fn create_secure_dir(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    }

    #[cfg(not(unix))]
    {
        fs::create_dir_all(path)?;
    }

    Ok(())
}

pub(super) fn secure_new_file(path: &Path) -> io::Result<fs::File> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    {
        options.mode(0o600);
    }

    let file = options.open(path)?;

    #[cfg(unix)]
    {
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
    }

    Ok(file)
}

struct InitLock {
    path: PathBuf,
    _file: fs::File,
}

impl InitLock {
    fn acquire(cwd: &Path) -> io::Result<Self> {
        let path = cwd.join(INIT_LOCK_FILE);
        let mut file = match create_init_lock(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                remove_stale_init_lock(&path)?;
                create_init_lock(&path).map_err(|retry_error| {
                    if retry_error.kind() == io::ErrorKind::AlreadyExists {
                        io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            "another saga init is already running",
                        )
                    } else {
                        retry_error
                    }
                })?
            }
            Err(error) => return Err(error),
        };
        write_init_lock_content(&mut file)?;
        Ok(Self { path, _file: file })
    }
}

impl Drop for InitLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
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

fn create_init_lock(path: &Path) -> io::Result<fs::File> {
    secure_new_file(path)
}

fn write_init_lock_content(file: &mut fs::File) -> io::Result<()> {
    writeln!(file, "{}{}", INIT_LOCK_PREFIX, std::process::id())?;
    file.sync_all()
}

fn remove_stale_init_lock(path: &Path) -> io::Result<()> {
    if init_lock_is_stale(path)? {
        remove_stale_temp(path)?;
    }
    Ok(())
}

fn init_lock_is_stale(path: &Path) -> io::Result<bool> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0_u8; INIT_LOCK_READ_MAX];
    let read = file.read(&mut buffer)?;
    let content = match std::str::from_utf8(&buffer[..read]) {
        Ok(content) => content,
        Err(_) => return Ok(true),
    };
    let Some(pid_text) = content.strip_prefix(INIT_LOCK_PREFIX) else {
        return Ok(true);
    };
    let Ok(pid) = pid_text.trim().parse::<u32>() else {
        return Ok(true);
    };
    Ok(!process_may_exist(pid))
}

fn process_may_exist(pid: u32) -> bool {
    if pid == std::process::id() {
        return true;
    }

    #[cfg(target_os = "linux")]
    {
        Path::new("/proc").join(pid.to_string()).exists()
    }

    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

#[cfg(test)]
mod tests;
