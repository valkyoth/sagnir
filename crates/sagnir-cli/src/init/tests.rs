use super::{
    FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES, INIT_LOCK_FILE, INIT_LOCK_PREFIX,
};
use std::fs;
use std::path::{Path, PathBuf};

use sagnir::store::{
    CONFIG_FILE, CONFIG_TEMP_FILE, FORMAT_FILE, Profile, REALM_FILE, REALM_TEMP_FILE, RealmConfig,
    parse_config_toml, parse_realm_toml,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
mod security_paths;

#[test]
fn init_dry_run_lists_layout_without_writing() -> std::io::Result<()> {
    let root = TempRoot::new("dry-run")?;
    let output = crate::dispatch_at(["init", "--dry-run"], root.path());

    assert_eq!(output.code, 0);
    assert!(output.stdout.contains("saga init --dry-run\nRoot: "));
    assert!(output.stdout.contains("  .saga\n"));
    assert!(output.stdout.contains("  .saga/objects\n"));
    assert!(output.stdout.contains("  .saga/FORMAT\n"));
    assert!(output.stdout.contains("  .saga/realm.toml\n"));
    assert!(output.stdout.contains("  .saga/config.toml\n"));
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
    assert_valid_metadata(root.path())?;
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
    assert_mode(root.path().join(INIT_LOCK_FILE), 0o600)?;
    assert_mode(root.path().join(REALM_FILE), 0o600)?;
    assert_mode(root.path().join(CONFIG_FILE), 0o600)?;
    Ok(())
}

#[test]
fn init_is_idempotent() -> std::io::Result<()> {
    let root = TempRoot::new("idempotent")?;
    assert_eq!(crate::dispatch_at(["init"], root.path()).code, 0);

    let original_realm = fs::read(root.path().join(REALM_FILE))?;
    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 0);
    assert!(
        output
            .stdout
            .contains("Sagnir store already initialized\nRoot: ")
    );
    assert_eq!(fs::read(root.path().join(REALM_FILE))?, original_realm);
    Ok(())
}

#[test]
fn init_upgrades_format_only_store_with_metadata() -> std::io::Result<()> {
    let root = TempRoot::new("metadata-upgrade")?;
    create_dir(root.path().join(".saga"))?;
    write_file(
        root.path().join(FORMAT_FILE),
        FORMAT_FILE_CONTENT.as_bytes(),
    )?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 0);
    assert!(output.stdout.contains("Sagnir store already initialized"));
    assert_valid_metadata(root.path())
}

#[test]
fn init_rejects_malformed_existing_realm_metadata() -> std::io::Result<()> {
    let root = TempRoot::new("malformed-realm")?;
    create_dir(root.path().join(".saga"))?;
    write_file(
        root.path().join(FORMAT_FILE),
        FORMAT_FILE_CONTENT.as_bytes(),
    )?;
    write_file(
        root.path().join(REALM_FILE),
        b"format = 1\nrealm_id = \"forged\"\n",
    )?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("invalid .saga/realm.toml"));
    assert!(!root.path().join(CONFIG_FILE).exists());
    Ok(())
}

#[test]
fn init_rejects_malformed_existing_config_metadata() -> std::io::Result<()> {
    let root = TempRoot::new("malformed-config")?;
    create_dir(root.path().join(".saga"))?;
    write_file(
        root.path().join(FORMAT_FILE),
        FORMAT_FILE_CONTENT.as_bytes(),
    )?;
    write_file(root.path().join(CONFIG_FILE), b"profile = \"solo\"\n")?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("invalid .saga/config.toml"));
    assert!(!root.path().join(REALM_FILE).exists());
    Ok(())
}

#[test]
fn init_rejects_oversized_config_metadata() -> std::io::Result<()> {
    let root = TempRoot::new("oversized-config")?;
    assert_eq!(crate::dispatch_at(["init"], root.path()).code, 0);
    write_file(root.path().join(CONFIG_FILE), &[b'x'; 2_049])?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(
        output
            .stderr
            .contains("metadata file exceeds its size limit")
    );
    Ok(())
}

#[test]
fn init_rejects_oversized_realm_metadata() -> std::io::Result<()> {
    let root = TempRoot::new("oversized-realm")?;
    create_dir(root.path().join(".saga"))?;
    write_file(
        root.path().join(FORMAT_FILE),
        FORMAT_FILE_CONTENT.as_bytes(),
    )?;
    write_file(root.path().join(REALM_FILE), &[b'x'; 257])?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(
        output
            .stderr
            .contains("metadata file exceeds its size limit")
    );
    Ok(())
}

#[test]
fn init_rejects_non_utf8_realm_and_config_metadata() -> std::io::Result<()> {
    for (name, file, expected) in [
        (
            "non-utf8-realm",
            REALM_FILE,
            "realm.toml is not valid UTF-8",
        ),
        (
            "non-utf8-config",
            CONFIG_FILE,
            "config.toml is not valid UTF-8",
        ),
    ] {
        let root = TempRoot::new(name)?;
        assert_eq!(crate::dispatch_at(["init"], root.path()).code, 0);
        write_file(root.path().join(file), &[0xff])?;

        let output = crate::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 1);
        assert!(output.stderr.contains(expected));
    }
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
fn init_removes_stale_metadata_temp_files() -> std::io::Result<()> {
    let root = TempRoot::new("stale-metadata-temp")?;
    assert_eq!(crate::dispatch_at(["init"], root.path()).code, 0);
    write_file(root.path().join(REALM_TEMP_FILE), b"stale")?;
    write_file(root.path().join(CONFIG_TEMP_FILE), b"stale")?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 0);
    assert!(!root.path().join(REALM_TEMP_FILE).exists());
    assert!(!root.path().join(CONFIG_TEMP_FILE).exists());
    assert_valid_metadata(root.path())
}

#[test]
fn init_recovers_from_malformed_stale_init_lock() -> std::io::Result<()> {
    let root = TempRoot::new("stale-lock")?;
    create_dir(root.path().join(".saga"))?;
    write_file(root.path().join(INIT_LOCK_FILE), b"stale")?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 0);
    assert_eq!(fs::read(root.path().join(INIT_LOCK_FILE))?, b"stale");
    assert_format_file(root.path())?;
    Ok(())
}

#[test]
fn init_recovers_from_unlocked_pid_marker() -> std::io::Result<()> {
    let root = TempRoot::new("unlocked-pid-marker")?;
    create_dir(root.path().join(".saga"))?;
    let lock = format!("{INIT_LOCK_PREFIX}{}\n", u32::MAX);
    write_file(root.path().join(INIT_LOCK_FILE), lock.as_bytes())?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 0);
    assert_eq!(fs::read(root.path().join(INIT_LOCK_FILE))?, lock.as_bytes());
    assert_format_file(root.path())?;
    Ok(())
}

#[test]
fn init_refuses_an_active_operating_system_lock() -> std::io::Result<()> {
    let root = TempRoot::new("active-os-lock")?;
    let store = super::SecureStore::open(root.path())?;
    let lock = super::InitLock::acquire(&store)?;

    let blocked = crate::dispatch_at(["init"], root.path());

    assert_eq!(blocked.code, 1);
    assert!(
        blocked
            .stderr
            .contains("another saga init is already running")
    );
    drop(lock);

    let retried = crate::dispatch_at(["init"], root.path());
    assert_eq!(retried.code, 0);
    assert_format_file(root.path())
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
fn init_rejects_lexical_aliases_of_system_roots() {
    for path in ["/tmp/..", "/etc/.", "//"] {
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

fn assert_valid_metadata(root: &Path) -> std::io::Result<()> {
    let realm = fs::read_to_string(root.join(REALM_FILE))?;
    parse_realm_toml(&realm).map_err(metadata_test_error)?;

    let config = fs::read_to_string(root.join(CONFIG_FILE))?;
    let parsed = parse_config_toml(&config).map_err(metadata_test_error)?;
    assert_eq!(parsed, RealmConfig::default());
    assert_eq!(parsed.profile, Profile::Standard);
    Ok(())
}

fn metadata_test_error(error: sagnir::store::StoreMetadataError) -> std::io::Error {
    std::io::Error::other(error.to_string())
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
