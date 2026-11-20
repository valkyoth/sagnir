#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    if let Some(command) = args.next() {
        if command != "init" || args.next().is_some() {
            eprintln!("usage: sagad [init]");
            return ExitCode::from(2);
        }
        return match std::env::current_dir().and_then(|root| sagnir_store_fs::initialize(&root)) {
            Ok(_) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("sagad init failed: {:?}", error.kind());
                ExitCode::FAILURE
            }
        };
    }
    println!(
        "sagad scaffold for Sagnir format {}",
        sagnir::format_version()
    );
    ExitCode::SUCCESS
}
