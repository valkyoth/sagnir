#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use sagnir::store::{
    FORMAT_FILE, FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, INIT_DIRECTORIES, STORE_DIR,
};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const EXIT_USAGE: u8 = 2;

const HELP_TEXT: &str = concat!(
    "saga - CLI command for Sagnir\n",
    "\n",
    "Usage:\n",
    "  saga <command>\n",
    "\n",
    "Commands:\n",
    "  help       Print this help text\n",
    "  init       Initialize a local Sagnir store\n",
    "  version    Print version information\n",
    "\n",
    "Exit Codes:\n",
    "  0    success\n",
    "  1    runtime failure\n",
    "  2    command line usage error\n",
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliOutput {
    pub code: u8,
    pub stdout: String,
    pub stderr: String,
}

pub fn dispatch<I, S>(args: I) -> CliOutput
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    match std::env::current_dir() {
        Ok(cwd) => dispatch_at(args, &cwd),
        Err(error) => runtime_error(&format!(
            "could not determine current directory: {}",
            sanitize_for_display(&error.to_string())
        )),
    }
}

pub fn dispatch_at<I, S>(args: I, cwd: &Path) -> CliOutput
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();
    let command = args.next();

    match command {
        None => help_output(),
        Some(command) => {
            let command = command.as_ref();

            match command {
                "help" | "--help" | "-h" => no_extra(command, &mut args, help_output),
                "init" => init_command(args, cwd),
                "version" | "--version" | "-V" => no_extra(command, &mut args, || CliOutput {
                    code: EXIT_SUCCESS,
                    stdout: format!(
                        "saga {}\nSagnir format {}\n",
                        env!("CARGO_PKG_VERSION"),
                        sagnir::format_version()
                    ),
                    stderr: String::new(),
                }),
                unknown => usage_error(&format!(
                    "unknown saga command: {}\n",
                    sanitize_for_display(unknown)
                )),
            }
        }
    }
}

fn no_extra<I, S, F>(command: &str, args: &mut I, output: F) -> CliOutput
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
    F: FnOnce() -> CliOutput,
{
    if let Some(extra) = args.next() {
        return unexpected_argument(command, extra.as_ref());
    }
    output()
}

fn init_command<I, S>(mut args: I, cwd: &Path) -> CliOutput
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
        cwd.display()
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
        code: EXIT_SUCCESS,
        stdout,
        stderr: String::new(),
    }
}

fn init_store(cwd: &Path) -> CliOutput {
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
        code: EXIT_SUCCESS,
        stdout: format!(
            "{message}\nRoot: {}\nStore: {}\nFormat: {}\n",
            cwd.display(),
            STORE_DIR,
            FORMAT_FILE
        ),
        stderr: String::new(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StoreInitStatus {
    Created,
    AlreadyInitialized,
}

fn create_store_layout(cwd: &Path) -> io::Result<StoreInitStatus> {
    let format_path = cwd.join(FORMAT_FILE);
    let already_initialized = existing_format_is_valid(&format_path)?;

    for dir in INIT_DIRECTORIES {
        fs::create_dir_all(cwd.join(dir))?;
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
    match fs::read_to_string(path) {
        Ok(content) if content == FORMAT_FILE_CONTENT => Ok(true),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "existing .saga/FORMAT has unexpected content",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
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
    let mut temp_file = match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp_path)
    {
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

fn help_output() -> CliOutput {
    CliOutput {
        code: EXIT_SUCCESS,
        stdout: HELP_TEXT.to_owned(),
        stderr: String::new(),
    }
}

fn runtime_error(message: &str) -> CliOutput {
    CliOutput {
        code: EXIT_FAILURE,
        stdout: String::new(),
        stderr: format!("{message}\n"),
    }
}

fn unexpected_argument(command: &str, extra: &str) -> CliOutput {
    usage_error(&format!(
        "unexpected saga argument after {}: {}\n",
        sanitize_for_display(command),
        sanitize_for_display(extra)
    ))
}

fn usage_error(message: &str) -> CliOutput {
    CliOutput {
        code: EXIT_USAGE,
        stdout: String::new(),
        stderr: format!("{message}\n{HELP_TEXT}"),
    }
}

fn sanitize_for_display(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_graphic() || character == ' ' {
                character
            } else {
                '?'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{FORMAT_FILE_CONTENT, FORMAT_TEMP_FILE, HELP_TEXT, INIT_DIRECTORIES, dispatch};
    use std::fs;
    use std::path::{Path, PathBuf};

    const VERSION_TEXT: &str = concat!("saga ", env!("CARGO_PKG_VERSION"), "\nSagnir format 1\n");

    #[test]
    fn no_args_prints_help() {
        assert_output(&[], 0, HELP_TEXT, "");
    }

    #[test]
    fn help_aliases_print_exact_help() {
        for args in [["help"], ["--help"], ["-h"]] {
            assert_output(&args, 0, HELP_TEXT, "");
        }
    }

    #[test]
    fn version_aliases_print_exact_version() {
        for args in [["version"], ["--version"], ["-V"]] {
            assert_output(&args, 0, VERSION_TEXT, "");
        }
    }

    #[test]
    fn init_dry_run_lists_layout_without_writing() -> std::io::Result<()> {
        let root = TempRoot::new("dry-run")?;
        let output = super::dispatch_at(["init", "--dry-run"], root.path());

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
    fn init_creates_store_layout() -> std::io::Result<()> {
        let root = TempRoot::new("create")?;
        let output = super::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 0);
        assert!(output.stdout.contains("Initialized Sagnir store\nRoot: "));
        assert_format_file(root.path())?;
        for dir in INIT_DIRECTORIES {
            assert!(root.path().join(dir).is_dir(), "missing {dir}");
        }
        Ok(())
    }

    #[test]
    fn init_is_idempotent() -> std::io::Result<()> {
        let root = TempRoot::new("idempotent")?;
        assert_eq!(super::dispatch_at(["init"], root.path()).code, 0);

        let output = super::dispatch_at(["init"], root.path());

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

        let output = super::dispatch_at(["init"], root.path());

        assert_eq!(output.code, 0);
        assert!(!root.path().join(FORMAT_TEMP_FILE).exists());
        assert_format_file(root.path())?;
        Ok(())
    }

    #[test]
    fn init_rejects_unexpected_argument() {
        let expected = format!("unexpected saga argument after init: now\n\n{HELP_TEXT}");

        assert_output(&["init", "now"], 2, "", &expected);
    }

    #[test]
    fn unknown_command_is_usage_error() {
        let expected = format!("unknown saga command: frobnicate\n\n{HELP_TEXT}");

        assert_output(&["frobnicate"], 2, "", &expected);
    }

    #[test]
    fn extra_argument_is_usage_error() {
        let expected = format!("unexpected saga argument after version: extra\n\n{HELP_TEXT}");

        assert_output(&["version", "extra"], 2, "", &expected);
    }

    #[test]
    fn usage_errors_sanitize_terminal_control_characters() {
        let expected = format!("unknown saga command: ?[2J?[HACCESS\n\n{HELP_TEXT}");

        assert_output(&["\u{1b}[2J\u{1b}[HACCESS"], 2, "", &expected);
    }

    fn assert_output(args: &[&str], code: u8, stdout: &str, stderr: &str) {
        let output = dispatch(args.iter().copied());

        assert_eq!(output.code, code);
        assert_eq!(output.stdout, stdout);
        assert_eq!(output.stderr, stderr);
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

    fn create_dir<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        fs::create_dir(path)
    }

    fn write_file<P: AsRef<Path>>(path: P, content: &[u8]) -> std::io::Result<()> {
        fs::write(path, content)
    }
}
