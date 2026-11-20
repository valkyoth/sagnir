#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    run().unwrap_or(ExitCode::FAILURE)
}

fn run() -> io::Result<ExitCode> {
    let mut args = std::env::args_os().skip(1);
    if let Some(command) = args.next() {
        if command != "init" || args.next().is_some() {
            writeln!(io::stderr().lock(), "usage: sagad [init]")?;
            return Ok(ExitCode::from(2));
        }
        return Ok(
            match std::env::current_dir().and_then(|root| sagnir_store_fs::initialize(&root)) {
                Ok(_) => ExitCode::SUCCESS,
                Err(error) => {
                    writeln!(io::stderr().lock(), "sagad init failed: {:?}", error.kind())?;
                    ExitCode::FAILURE
                }
            },
        );
    }
    let mut stdout = io::stdout().lock();
    if let Err(error) = writeln!(
        stdout,
        "sagad scaffold for Sagnir format {}",
        sagnir::format_version()
    )
    .and_then(|()| stdout.flush())
        && error.kind() != io::ErrorKind::BrokenPipe
    {
        return Err(error);
    }
    Ok(ExitCode::SUCCESS)
}
