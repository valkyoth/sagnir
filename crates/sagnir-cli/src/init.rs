use crate::{CliOutput, runtime_error, sanitize_for_display, unexpected_argument};
use sagnir::store::{CONFIG_FILE, FORMAT_FILE, INIT_DIRECTORIES, REALM_FILE, STORE_DIR};
use sagnir_store_fs::{StoreInitStatus, initialize};
use std::path::Path;

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
    match initialize(cwd) {
        Ok((StoreInitStatus::Created, root)) => init_success("Initialized Sagnir store", &root),
        Ok((StoreInitStatus::AlreadyInitialized, root)) => {
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

fn sanitize_path(path: &Path) -> String {
    sanitize_for_display(&path.display().to_string())
}

#[cfg(all(test, unix))]
mod tests;
