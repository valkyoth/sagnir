#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    run().unwrap_or(ExitCode::FAILURE)
}

fn run() -> io::Result<ExitCode> {
    let args = match decode_args(env::args_os().skip(1)) {
        Ok(args) => args,
        Err(()) => {
            writeln!(io::stderr().lock(), "saga: argument is not valid Unicode")?;
            return Ok(ExitCode::from(2));
        }
    };
    let output = sagnir_cli::dispatch(args);

    let mut stdout = io::stdout().lock();
    if let Err(error) = stdout
        .write_all(output.stdout.as_bytes())
        .and_then(|()| stdout.flush())
        && error.kind() != io::ErrorKind::BrokenPipe
    {
        return Err(error);
    }
    io::stderr().lock().write_all(output.stderr.as_bytes())?;
    Ok(ExitCode::from(output.code))
}

fn decode_args<I>(args: I) -> Result<Vec<String>, ()>
where
    I: IntoIterator<Item = OsString>,
{
    args.into_iter()
        .map(|argument| argument.into_string().map_err(|_| ()))
        .collect()
}

#[cfg(all(test, unix))]
mod tests {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    #[test]
    fn invalid_unicode_argument_is_rejected() {
        let args = [OsString::from_vec(vec![0xff])];

        assert_eq!(super::decode_args(args), Err(()));
    }
}
