use std::fs;
use std::io::Write;
use std::os::unix::fs::symlink;

use sagnir::store::{CONFIG_FILE, FORMAT_FILE, FORMAT_TEMP_FILE};

use super::{INIT_LOCK_FILE, TempRoot, create_dir, write_file};

#[test]
fn open_store_handle_survives_namespace_replacement() -> std::io::Result<()> {
    let root = TempRoot::new("store-namespace-replacement")?;
    let store = super::super::SecureStore::open(root.path())?;
    fs::rename(
        root.path().join(".saga"),
        root.path().join("original-store"),
    )?;
    create_dir(root.path().join(".saga"))?;

    store.ensure_directory(".saga/objects")?;
    let mut temp = store.create_new_file(FORMAT_TEMP_FILE)?;
    temp.write_all(b"anchored")?;
    temp.sync_all()?;

    assert!(root.path().join("original-store/objects").is_dir());
    assert_eq!(
        fs::read(root.path().join("original-store/FORMAT.tmp"))?,
        b"anchored"
    );
    assert!(fs::read_dir(root.path().join(".saga"))?.next().is_none());
    Ok(())
}

#[test]
fn init_rejects_symlinked_metadata_files() -> std::io::Result<()> {
    let root = TempRoot::new("symlinked-metadata")?;
    assert_eq!(crate::dispatch_at(["init"], root.path()).code, 0);
    let outside = root.path().join("outside-config");
    write_file(&outside, b"not trusted")?;
    fs::remove_file(root.path().join(CONFIG_FILE))?;
    symlink(&outside, root.path().join(CONFIG_FILE))?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("saga init failed"));
    assert_eq!(fs::read(outside)?, b"not trusted");
    Ok(())
}

#[test]
fn init_rejects_symlinked_store_root() -> std::io::Result<()> {
    let root = TempRoot::new("symlinked-store-root")?;
    let redirected = root.path().join("redirected");
    create_dir(&redirected)?;
    symlink(&redirected, root.path().join(".saga"))?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("refusing to initialize through"));
    assert!(fs::read_dir(redirected)?.next().is_none());
    Ok(())
}

#[test]
fn init_rejects_symlinked_store_subdirectory() -> std::io::Result<()> {
    let root = TempRoot::new("symlinked-store-subdirectory")?;
    let redirected = root.path().join("redirected");
    create_dir(&redirected)?;
    create_dir(root.path().join(".saga"))?;
    symlink(&redirected, root.path().join(".saga/objects"))?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("refusing to initialize through"));
    assert!(fs::read_dir(redirected)?.next().is_none());
    assert!(!root.path().join(FORMAT_FILE).exists());
    Ok(())
}

#[test]
fn init_rejects_symlinked_lock_file() -> std::io::Result<()> {
    let root = TempRoot::new("symlinked-lock-file")?;
    create_dir(root.path().join(".saga"))?;
    let redirected = root.path().join("redirected-lock");
    write_file(&redirected, b"not trusted")?;
    symlink(&redirected, root.path().join(INIT_LOCK_FILE))?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("saga init failed"));
    assert_eq!(fs::read(redirected)?, b"not trusted");
    assert!(!root.path().join(FORMAT_FILE).exists());
    Ok(())
}

#[test]
fn init_rejects_multiply_linked_lock_file() -> std::io::Result<()> {
    let root = TempRoot::new("multiply-linked-lock-file")?;
    create_dir(root.path().join(".saga"))?;
    let redirected = root.path().join("redirected-lock");
    write_file(&redirected, b"not trusted")?;
    fs::hard_link(&redirected, root.path().join(INIT_LOCK_FILE))?;

    let output = crate::dispatch_at(["init"], root.path());

    assert_eq!(output.code, 1);
    assert!(output.stderr.contains("multiply linked Sagnir store file"));
    assert_eq!(fs::read(redirected)?, b"not trusted");
    assert!(!root.path().join(FORMAT_FILE).exists());
    Ok(())
}
