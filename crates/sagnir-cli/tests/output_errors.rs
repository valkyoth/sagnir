#[path = "../../../tests/output_errors.rs"]
mod common;

#[test]
fn closed_stdout_is_handled() -> std::io::Result<()> {
    common::closed_stdout(env!("CARGO_BIN_EXE_saga"))
}

#[cfg(target_os = "linux")]
#[test]
fn full_stdout_and_stderr_fail_cleanly() -> std::io::Result<()> {
    common::full_output(env!("CARGO_BIN_EXE_saga"))
}

#[cfg(unix)]
#[test]
fn invalid_unicode_with_closed_diagnostic_pipe() -> std::io::Result<()> {
    use std::os::unix::ffi::OsStrExt;
    let (reader, writer) = std::io::pipe()?;
    drop(reader);
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_saga"))
        .arg(std::ffi::OsStr::from_bytes(&[0xff]))
        .stderr(std::process::Stdio::from(writer))
        .status()?;
    assert_eq!(status.code(), Some(1));
    Ok(())
}
