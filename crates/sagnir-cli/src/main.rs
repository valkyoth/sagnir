#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();
    let _program = args.next();
    match args.next().as_deref() {
        Some("version") | Some("--version") | Some("-V") => {
            print_version();
            ExitCode::SUCCESS
        }
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(_unknown) => {
            eprintln!("unknown saga command");
            print_help();
            ExitCode::from(2)
        }
    }
}

fn print_version() {
    println!("saga {}", env!("CARGO_PKG_VERSION"));
    println!("Sagnir format {}", sagnir::format_version());
}

fn print_help() {
    println!("saga - CLI command for Sagnir");
    println!();
    println!("Commands:");
    println!("  version    print version information");
    println!("  help       print this help text");
}
