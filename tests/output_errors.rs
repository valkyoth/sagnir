use std::io;
use std::process::{Command, Stdio};

pub fn closed_stdout(binary: &str) -> io::Result<()> {
    let (reader, writer) = io::pipe()?;
    drop(reader);
    let output = Command::new(binary).stdout(Stdio::from(writer)).output()?;
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty());
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn full_output(binary: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    let full = || OpenOptions::new().write(true).open("/dev/full");
    let output = Command::new(binary).stdout(full()?).output()?;
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let status = Command::new(binary)
        .arg("invalid")
        .stderr(full()?)
        .status()?;
    assert_eq!(status.code(), Some(1));
    Ok(())
}
