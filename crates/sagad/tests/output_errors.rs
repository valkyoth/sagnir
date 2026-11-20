#[path = "../../../tests/output_errors.rs"]
mod common;

#[test]
fn closed_stdout_is_handled() -> std::io::Result<()> {
    common::closed_stdout(env!("CARGO_BIN_EXE_sagad"))
}

#[cfg(target_os = "linux")]
#[test]
fn full_stdout_and_stderr_fail_cleanly() -> std::io::Result<()> {
    common::full_output(env!("CARGO_BIN_EXE_sagad"))
}

#[cfg(unix)]
#[test]
fn failed_init_with_closed_diagnostic_pipe() -> std::io::Result<()> {
    let (reader, writer) = std::io::pipe()?;
    drop(reader);
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_sagad"))
        .arg("init")
        .current_dir("/")
        .stderr(std::process::Stdio::from(writer))
        .status()?;
    assert_eq!(status.code(), Some(1));
    Ok(())
}
