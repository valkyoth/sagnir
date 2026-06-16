#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use std::process::ExitCode;

fn main() -> ExitCode {
    println!(
        "sagad scaffold for Sagnir format {}",
        sagnir::format_version()
    );
    ExitCode::SUCCESS
}
