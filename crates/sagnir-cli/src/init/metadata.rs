use std::io::{self, Read, Write};

use sagnir::core::{ID_BYTES, RealmId};
use sagnir::store::{
    CONFIG_FILE, CONFIG_FILE_MAX, CONFIG_TEMP_FILE, REALM_FILE, REALM_FILE_MAX, REALM_TEMP_FILE,
    RealmConfig, RealmMetadata, parse_config_toml, parse_realm_toml, write_config_toml,
    write_realm_toml,
};

use super::secure_store::SecureStore;

pub(super) fn ensure_store_metadata(store: &SecureStore) -> io::Result<()> {
    let realm_exists = read_realm(store)?.is_some();
    let config_exists = read_config(store)?.is_some();
    store.remove_file_if_exists(REALM_TEMP_FILE)?;
    store.remove_file_if_exists(CONFIG_TEMP_FILE)?;

    if !realm_exists {
        write_new_realm(store)?;
    }
    if !config_exists {
        write_default_config(store)?;
    }
    Ok(())
}

fn write_new_realm(store: &SecureStore) -> io::Result<()> {
    let metadata =
        RealmMetadata::new(RealmId::new(random_realm_bytes()?)).map_err(metadata_error)?;
    let mut output = [0_u8; REALM_FILE_MAX];
    let len = write_realm_toml(metadata, &mut output).map_err(metadata_error)?;
    write_metadata_file(store, REALM_FILE, REALM_TEMP_FILE, &output[..len])
}

fn write_default_config(store: &SecureStore) -> io::Result<()> {
    let mut output = [0_u8; CONFIG_FILE_MAX];
    let len = write_config_toml(RealmConfig::default(), &mut output).map_err(metadata_error)?;
    write_metadata_file(store, CONFIG_FILE, CONFIG_TEMP_FILE, &output[..len])
}

fn read_realm(store: &SecureStore) -> io::Result<Option<()>> {
    let Some((buffer, len)) = read_bounded::<REALM_FILE_MAX>(store, REALM_FILE)? else {
        return Ok(None);
    };
    let content = std::str::from_utf8(&buffer[..len])
        .map_err(|_| invalid_data("existing .saga/realm.toml is not valid UTF-8"))?;
    parse_realm_toml(content)
        .map(|_| Some(()))
        .map_err(|error| invalid_data(&format!("invalid .saga/realm.toml: {error}")))
}

fn read_config(store: &SecureStore) -> io::Result<Option<()>> {
    let Some((buffer, len)) = read_bounded::<CONFIG_FILE_MAX>(store, CONFIG_FILE)? else {
        return Ok(None);
    };
    let content = std::str::from_utf8(&buffer[..len])
        .map_err(|_| invalid_data("existing .saga/config.toml is not valid UTF-8"))?;
    parse_config_toml(content)
        .map(|_| Some(()))
        .map_err(|error| invalid_data(&format!("invalid .saga/config.toml: {error}")))
}

fn read_bounded<const N: usize>(
    store: &SecureStore,
    path: &str,
) -> io::Result<Option<([u8; N], usize)>> {
    let Some(file) = store.open_existing_file(path, false)? else {
        return Ok(None);
    };
    read_bounded_reader(file).map(Some)
}

fn read_bounded_reader<const N: usize, R: Read>(mut reader: R) -> io::Result<([u8; N], usize)> {
    let mut buffer = [0_u8; N];
    let mut len = 0;
    while len < N {
        let read = read_retry(&mut reader, &mut buffer[len..])?;
        if read == 0 {
            return Ok((buffer, len));
        }
        len += read;
    }

    let mut trailing = [0_u8; 1];
    if read_retry(&mut reader, &mut trailing)? != 0 {
        return Err(invalid_data("Sagnir metadata file exceeds its size limit"));
    }
    Ok((buffer, len))
}

fn read_retry<R: Read>(reader: &mut R, output: &mut [u8]) -> io::Result<usize> {
    loop {
        match reader.read(output) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            result => return result,
        }
    }
}

fn write_metadata_file(
    store: &SecureStore,
    path: &str,
    temp_path: &str,
    content: &[u8],
) -> io::Result<()> {
    store.remove_file_if_exists(temp_path)?;
    let mut temp = store.create_new_file(temp_path)?;
    if let Err(error) = temp.write_all(content).and_then(|()| temp.sync_all()) {
        let _ = store.remove_file_if_exists(temp_path);
        return Err(error);
    }
    if let Err(error) = store.commit_file(&temp, temp_path, path) {
        let _ = store.remove_file_if_exists(temp_path);
        return Err(error);
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

#[cfg(test)]
mod tests;
