pub mod print;
use std::process;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Terminate the programm after printing an error message
pub fn quit_error(error_msg: &str) {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stderr.set_color(&ColorSpec::new());
    stderr.set_color(&ColorSpec::new().set_fg(Some(Color::Red)));
    write!(&mut stderr, "Error: ");
    stderr.set_color(&ColorSpec::new());
    write!(&mut stderr, "{}\n", error_msg);
    process::exit(1);
}

/// Return the optional value or terminate the programm with an error
pub fn unpack_or_err<T>(val: Option<T>, error_msg: &str) -> T {
    match val {
        Some(val) => val,
        None => {
            quit_error(error_msg);
            process::exit(1);
        }
    }
}

/// Return the optional value or the default value
pub fn unpack_or_default<T>(val: Option<T>, default: T) -> T {
    match val {
        Some(val) => val,
        None => default,
    }
}

/// Return the result or terminate the programm with an error
pub fn unwrap_or_err<T, E>(val: Result<T, E>, error_msg: &str) -> T
where
    E: std::error::Error,
{
    match val {
        Ok(val) => val,
        Err(e) => {
            quit_error(&format!("{} {}", error_msg, e.to_string()));
            process::exit(1);
        }
    }
}
