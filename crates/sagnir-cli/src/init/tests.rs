use super::{
    FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES, INIT_LOCK_FILE, INIT_LOCK_PREFIX,
};
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
fn init_recovers_from_malformed_stale_init_lock() -> std::io::Result<()> {
    let root = TempRoot::new("stale-lock")?;
    create_dir(root.path().join(".saga"))?;
    write_file(root.path().join(INIT_LOCK_FILE), b"stale")?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 0);
    assert!(!root.path().join(INIT_LOCK_FILE).exists());
    assert_format_file(root.path())?;
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn init_recovers_from_dead_pid_init_lock() -> std::io::Result<()> {
    let root = TempRoot::new("dead-pid-lock")?;
    create_dir(root.path().join(".saga"))?;
    let lock = format!("{INIT_LOCK_PREFIX}{}\n", u32::MAX);
    write_file(root.path().join(INIT_LOCK_FILE), lock.as_bytes())?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 0);
    assert!(!root.path().join(INIT_LOCK_FILE).exists());
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

#[cfg(unix)]
#[test]
fn init_rejects_common_macos_system_roots() {
    for path in ["/Applications", "/Library", "/System", "/private/var"] {
        let output = crate::dispatch_at(["init"], Path::new(path));
        assert_eq!(output.code, 1, "{path}");
        assert!(
            output
                .stderr
                .contains("refusing to initialize a system directory"),
            "{path}"
        );
    }
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
