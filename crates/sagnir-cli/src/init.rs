use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};

use sagnir::store::{
    FORMAT_FILE, FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES, INIT_LOCK_FILE, STORE_DIR,
};

use crate::{CliOutput, runtime_error, sanitize_for_display, unexpected_argument};

const FORMAT_FILE_READ_MAX: usize = FORMAT_FILE_CONTENT.len() + 1;

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
    stdout.push_str("\nFiles:\n  ");
    stdout.push_str(FORMAT_FILE);
    stdout.push_str("\n\nNo changes written.\n");

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
        return Ok(StoreInitStatus::AlreadyInitialized);
    }

    write_format_file(&format_path, &temp_path)?;
    Ok(StoreInitStatus::Created)
}

fn existing_format_is_valid(path: &Path) -> io::Result<bool> {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error),
    };

    let mut buffer = [0_u8; FORMAT_FILE_READ_MAX];
    let read = file.read(&mut buffer)?;
    if read == FORMAT_FILE_CONTENT.len() && buffer[..read] == *FORMAT_FILE_CONTENT.as_bytes() {
        Ok(true)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "existing .saga/FORMAT has unexpected content",
        ))
    }
}

fn remove_stale_temp(path: &Path) -> io::Result<()> {
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

fn secure_new_file(path: &Path) -> io::Result<fs::File> {
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
        let file = secure_new_file(&path).map_err(|error| {
            if error.kind() == io::ErrorKind::AlreadyExists {
                io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "another saga init is already running or a stale init lock exists",
                )
            } else {
                error
            }
        })?;
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
        const RESTRICTED: [&str; 15] = [
            "/", "/bin", "/boot", "/dev", "/etc", "/lib", "/lib64", "/proc", "/root", "/run",
            "/sbin", "/sys", "/tmp", "/usr", "/var",
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

#[cfg(test)]
mod tests {
    use super::{FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES};
    use std::fs;
    use std::path::{Path, PathBuf};

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn init_dry_run_lists_layout_without_writing() -> std::io::Result<()> {
        let root = TempRoot::new("dry-run")?;
        let output = crate::dispatch_at(["init", "--dry-run"], root.path());

        assert_eq!(output.code, 0);
        assert!(output.stdout.contains("saga init --dry-run\nRoot: "));
        assert!(output.stdout.contains("  .saga\n"));
        assert!(output.stdout.contains("  .saga/objects\n"));
        assert!(output.stdout.contains("  .saga/FORMAT\n"));
        assert!(output.stdout.contains("No changes written.\n"));
        assert_eq!(output.stderr, "");
        assert!(!root.path().join(".saga").exists());
        Ok(())
    }

    #[test]
    fn init_sanitizes_root_path_in_output() {
        let root = Path::new("/tmp/sagnir-\u{1b}[2J\u{1b}[H");
        let output = crate::dispatch_at(["init", "--dry-run"], root);

        assert_eq!(output.code, 0);
        assert!(!output.stdout.contains('\u{1b}'));
        assert!(output.stdout.contains("Root: /tmp/sagnir-?[2J?[H"));
    }

    #[test]
    fn init_creates_store_layout() -> std::io::Result<()> {
        let root = TempRoot::new("create")?;
        let output = crate::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 0);
        assert!(output.stdout.contains("Initialized Sagnir store\nRoot: "));
        assert_format_file(root.path())?;
        for dir in INIT_DIRECTORIES {
            assert!(root.path().join(dir).is_dir(), "missing {dir}");
        }
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn init_creates_owner_only_store_permissions() -> std::io::Result<()> {
        let root = TempRoot::new("permissions")?;
        let output = crate::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 0);
        assert_mode(root.path().join(".saga"), 0o700)?;
        assert_mode(root.path().join(".saga/keys"), 0o700)?;
        assert_mode(root.path().join(".saga/policies"), 0o700)?;
        assert_mode(root.path().join(".saga/FORMAT"), 0o600)?;
        Ok(())
    }

    #[test]
    fn init_is_idempotent() -> std::io::Result<()> {
        let root = TempRoot::new("idempotent")?;
        assert_eq!(crate::dispatch_at(["init"], root.path()).code, 0);

        let output = crate::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 0);
        assert!(
            output
                .stdout
                .contains("Sagnir store already initialized\nRoot: ")
        );
        Ok(())
    }

    #[test]
    fn init_removes_stale_format_temp_file() -> std::io::Result<()> {
        let root = TempRoot::new("stale-temp")?;
        create_dir(root.path().join(".saga"))?;
        write_file(root.path().join(FORMAT_TEMP_FILE), b"stale")?;

        let output = crate::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 0);
        assert!(!root.path().join(FORMAT_TEMP_FILE).exists());
        assert_format_file(root.path())?;
        Ok(())
    }

    #[test]
    fn init_rejects_oversized_format_file() -> std::io::Result<()> {
        let root = TempRoot::new("oversized-format")?;
        create_dir(root.path().join(".saga"))?;
        write_file(
            root.path().join(".saga/FORMAT"),
            b"sagnir-format = 1\nunexpected",
        )?;

        let output = crate::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 1);
        assert!(
            output
                .stderr
                .contains("existing .saga/FORMAT has unexpected content")
        );
        Ok(())
    }

    #[test]
    fn init_rejects_system_root() {
        let output = crate::dispatch_at(["init"], Path::new("/"));

        assert_eq!(output.code, 1);
        assert!(
            output
                .stderr
                .contains("refusing to initialize a system directory")
        );
    }

    #[test]
    fn init_rejects_unexpected_argument() {
        let expected = format!(
            "unexpected saga argument after init: now\n\n{}",
            crate::HELP_TEXT
        );

        crate::tests::assert_output(&["init", "now"], 2, "", &expected);
    }

    struct TempRoot {
        path: PathBuf,
    }

    impl TempRoot {
        fn new(name: &str) -> std::io::Result<Self> {
            let path =
                std::env::temp_dir().join(format!("sagnir-cli-test-{}-{name}", std::process::id()));
            let _ = fs::remove_dir_all(&path);
            create_dir(&path)?;
            Ok(Self { path })
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn assert_format_file(root: &Path) -> std::io::Result<()> {
        let content = fs::read_to_string(root.join(".saga/FORMAT"))?;
        assert_eq!(content, FORMAT_FILE_CONTENT);
        Ok(())
    }

    #[cfg(unix)]
    fn assert_mode<P: AsRef<Path>>(path: P, expected: u32) -> std::io::Result<()> {
        let mode = fs::metadata(path)?.permissions().mode() & 0o777;
        assert_eq!(mode, expected);
        Ok(())
    }

    fn create_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        fs::create_dir(path)
    }

    fn write_file<P: AsRef<Path>>(path: P, content: &[u8]) -> std::io::Result<()> {
        fs::write(path, content)
    }
}
