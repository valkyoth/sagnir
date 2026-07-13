use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use sagnir::core::{ID_BYTES, RealmId};
use sagnir::store::{
    CONFIG_FILE, CONFIG_FILE_MAX, CONFIG_TEMP_FILE, REALM_FILE, REALM_FILE_MAX, REALM_TEMP_FILE,
    RealmConfig, RealmMetadata, parse_config_toml, parse_realm_toml, write_config_toml,
    write_realm_toml,
};

use super::{remove_stale_temp, secure_new_file};

pub(super) fn ensure_store_metadata(cwd: &Path) -> io::Result<()> {
    let realm_exists = read_realm(&cwd.join(REALM_FILE))?.is_some();
    let config_exists = read_config(&cwd.join(CONFIG_FILE))?.is_some();
    remove_stale_temp(&cwd.join(REALM_TEMP_FILE))?;
    remove_stale_temp(&cwd.join(CONFIG_TEMP_FILE))?;

    if !realm_exists {
        write_new_realm(cwd)?;
    }
    if !config_exists {
        write_default_config(cwd)?;
    }
    Ok(())
}

fn write_new_realm(cwd: &Path) -> io::Result<()> {
    let path = cwd.join(REALM_FILE);
    let metadata =
        RealmMetadata::new(RealmId::new(random_realm_bytes()?)).map_err(metadata_error)?;
    let mut output = [0_u8; REALM_FILE_MAX];
    let len = write_realm_toml(metadata, &mut output).map_err(metadata_error)?;
    write_metadata_file(&path, &cwd.join(REALM_TEMP_FILE), &output[..len])
}

fn write_default_config(cwd: &Path) -> io::Result<()> {
    let path = cwd.join(CONFIG_FILE);
    let mut output = [0_u8; CONFIG_FILE_MAX];
    let len = write_config_toml(RealmConfig::default(), &mut output).map_err(metadata_error)?;
    write_metadata_file(&path, &cwd.join(CONFIG_TEMP_FILE), &output[..len])
}

fn read_realm(path: &Path) -> io::Result<Option<()>> {
    let Some((buffer, len)) = read_bounded::<REALM_FILE_MAX>(path)? else {
        return Ok(None);
    };
    let content = std::str::from_utf8(&buffer[..len])
        .map_err(|_| invalid_data("existing .saga/realm.toml is not valid UTF-8"))?;
    parse_realm_toml(content)
        .map(|_| Some(()))
        .map_err(|error| invalid_data(&format!("invalid .saga/realm.toml: {error}")))
}

fn read_config(path: &Path) -> io::Result<Option<()>> {
    let Some((buffer, len)) = read_bounded::<CONFIG_FILE_MAX>(path)? else {
        return Ok(None);
    };
    let content = std::str::from_utf8(&buffer[..len])
        .map_err(|_| invalid_data("existing .saga/config.toml is not valid UTF-8"))?;
    parse_config_toml(content)
        .map(|_| Some(()))
        .map_err(|error| invalid_data(&format!("invalid .saga/config.toml: {error}")))
}

fn read_bounded<const N: usize>(path: &Path) -> io::Result<Option<([u8; N], usize)>> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    if !metadata.file_type().is_file() {
        return Err(invalid_data("Sagnir metadata path is not a regular file"));
    }
    let mut file = fs::File::open(path)?;
    let mut buffer = [0_u8; N];
    let len = file.read(&mut buffer)?;
    let mut trailing = [0_u8; 1];
    if len == N && file.read(&mut trailing)? != 0 {
        return Err(invalid_data("Sagnir metadata file exceeds its size limit"));
    }
    Ok(Some((buffer, len)))
}

fn write_metadata_file(path: &Path, temp_path: &Path, content: &[u8]) -> io::Result<()> {
    remove_stale_temp(temp_path)?;
    let mut temp = secure_new_file(temp_path)?;
    if let Err(error) = temp.write_all(content).and_then(|()| temp.sync_all()) {
        let _ = remove_stale_temp(temp_path);
        return Err(error);
    }
    drop(temp);
    if let Err(error) = fs::rename(temp_path, path) {
        let _ = remove_stale_temp(temp_path);
        return Err(error);
    }
    sync_metadata_directory(path)?;
    Ok(())
}

fn sync_metadata_directory(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        let parent = path
            .parent()
            .ok_or_else(|| io::Error::other("Sagnir metadata path has no parent"))?;
        fs::File::open(parent)?.sync_all()?;
    }
    Ok(())
}

fn random_realm_bytes() -> io::Result<[u8; ID_BYTES]> {
    let mut bytes = [0_u8; ID_BYTES];
    for _ in 0..2 {
        getrandom::fill(&mut bytes)
            .map_err(|_| io::Error::other("operating system random source unavailable"))?;
        if bytes != [0_u8; ID_BYTES] {
            return Ok(bytes);
        }
    }
    Err(io::Error::other(
        "operating system random source returned an invalid realm ID",
    ))
}

fn metadata_error(error: sagnir::store::StoreMetadataError) -> io::Error {
    invalid_data(&error.to_string())
}

fn invalid_data(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
