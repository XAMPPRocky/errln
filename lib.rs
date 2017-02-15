/// Macro for printing to the standard error.
///
/// Equivalent to the `errln!` macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stderr is frequently line-buffered by default so it may be
/// necessary to use `io::stderr().flush()` to ensure the output is emitted
/// immediately.
///
/// If writing to stderr fails, there is usually not much you should
/// do about it, so all I/O-errors are ignored.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate errln;
///
/// # fn main() {
///
/// use std::io::{self, Write};
///
/// err!("this ");
/// err!("will ");
/// err!("be ");
/// err!("on ");
/// err!("the ");
/// err!("same ");
/// err!("line ");
///
/// io::stderr().flush();
///
/// err!("this string has a newline, why not choose errln! instead?\n");
///
/// io::stderr().flush();
/// # }
/// ```
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(&mut ::std::io::stderr(), $($arg)*);
    }};
}

/// Macro for printing to the standard err, with a newline. On all
/// platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`).
///
/// Use the `format!` syntax to write data to the standard output.
/// See `std::fmt` for more information.
///
/// If writing to stderr fails, there is usually not much you should
/// do about it, so all I/O-errors are ignored.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate errln;
///
/// # fn main() {
///
/// errln!("hello there!");
/// errln!("format {} arguments", "some");
/// errln!();
/// # }
/// ```
#[macro_export]
macro_rules! errln {
    () => {err!("\n")};
    ($fmt:expr) => (err!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (err!(concat!($fmt, "\n"), $($arg)*));
}
