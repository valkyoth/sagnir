use std::{fs, io, path::Path, process::Command};

fn daemon_init(root: &Path) -> io::Result<bool> {
    Ok(Command::new(env!("CARGO_BIN_EXE_sagad"))
        .current_dir(root)
        .arg("init")
        .output()?
        .status
        .success())
}

fn callers(root: &Path) -> io::Result<[bool; 3]> {
    Ok([
        sagnir_store_fs::initialize(root).is_ok(),
        sagnir_cli::dispatch_at(["init"], root).code == 0,
        daemon_init(root)?,
    ])
}

#[test]
fn shared_initialization_boundary() -> io::Result<()> {
    let root = std::env::temp_dir().join(format!("sagnir-parity-{}", std::process::id()));
    fs::create_dir(&root)?;
    #[cfg(unix)]
    {
        use sagnir::store::{CONFIG_FILE, FORMAT_TEMP_FILE, INIT_LOCK_FILE, REALM_FILE};
        assert!(daemon_init(&root)?);
        let realm = fs::read(root.join(REALM_FILE))?;
        let lock = fs::File::open(root.join(INIT_LOCK_FILE))?;
        lock.lock()?;
        assert_eq!(callers(&root)?, [false; 3]);
        drop(lock);
        fs::write(root.join(FORMAT_TEMP_FILE), b"incomplete")?;
        assert_eq!(callers(&root)?, [true; 3]);
        assert!(!root.join(FORMAT_TEMP_FILE).exists());
        assert_eq!(fs::read(root.join(REALM_FILE))?, realm);
        fs::write(root.join(CONFIG_FILE), b"invalid")?;
        assert_eq!(callers(&root)?, [false; 3]);
        assert_eq!(fs::read(root.join(CONFIG_FILE))?, b"invalid");
    }
    #[cfg(not(unix))]
    {
        assert_eq!(callers(&root)?, [false; 3]);
        assert!(fs::read_dir(&root)?.next().is_none());
    }
    fs::remove_dir_all(root)
}
