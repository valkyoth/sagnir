#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let output = sagnir_cli::dispatch(env::args().skip(1));

    print!("{}", output.stdout);
    eprint!("{}", output.stderr);

    if output.code == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(output.code)
    }
}
