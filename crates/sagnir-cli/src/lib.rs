#![forbid(unsafe_code)]
#![deny(unused_must_use)]

const EXIT_SUCCESS: u8 = 0;
const EXIT_USAGE: u8 = 2;

const HELP_TEXT: &str = concat!(
    "saga - CLI command for Sagnir\n",
    "\n",
    "Usage:\n",
    "  saga <command>\n",
    "\n",
    "Commands:\n",
    "  help       Print this help text\n",
    "  version    Print version information\n",
    "\n",
    "Exit Codes:\n",
    "  0    success\n",
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
    let mut args = args.into_iter();
    let command = args.next();

    match command {
        None => help_output(),
        Some(command) => {
            let command = command.as_ref();
            if let Some(extra) = args.next() {
                return usage_error(&format!(
                    "unexpected saga argument after {command}: {}\n",
                    extra.as_ref()
                ));
            }

            match command {
                "help" | "--help" | "-h" => help_output(),
                "version" | "--version" | "-V" => CliOutput {
                    code: EXIT_SUCCESS,
                    stdout: format!(
                        "saga {}\nSagnir format {}\n",
                        env!("CARGO_PKG_VERSION"),
                        sagnir::format_version()
                    ),
                    stderr: String::new(),
                },
                unknown => usage_error(&format!("unknown saga command: {unknown}\n")),
            }
        }
    }
}

fn help_output() -> CliOutput {
    CliOutput {
        code: EXIT_SUCCESS,
        stdout: HELP_TEXT.to_owned(),
        stderr: String::new(),
    }
}

fn usage_error(message: &str) -> CliOutput {
    CliOutput {
        code: EXIT_USAGE,
        stdout: String::new(),
        stderr: format!("{message}\n{HELP_TEXT}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{HELP_TEXT, dispatch};

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
    fn unknown_command_is_usage_error() {
        let expected = format!("unknown saga command: frobnicate\n\n{HELP_TEXT}");

        assert_output(&["frobnicate"], 2, "", &expected);
    }

    #[test]
    fn extra_argument_is_usage_error() {
        let expected = format!("unexpected saga argument after version: extra\n\n{HELP_TEXT}");

        assert_output(&["version", "extra"], 2, "", &expected);
    }

    fn assert_output(args: &[&str], code: u8, stdout: &str, stderr: &str) {
        let output = dispatch(args.iter().copied());

        assert_eq!(output.code, code);
        assert_eq!(output.stdout, stdout);
        assert_eq!(output.stderr, stderr);
    }
}
