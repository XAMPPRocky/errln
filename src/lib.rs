/// Macro for printing to the standard error.
///
/// Equivalent to the `errln!` macro except that a newline is not printed at
/// the end of the message.
///
/// Note that stderr is frequently line-buffered by default so it may be
/// necessary to use `io::stderr().flush()` to ensure the output is emitted
/// immediately.
///
/// # Panics
///
/// Panics if writing to `io::stderr()` fails.
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
/// io::stderr().flush().unwrap();
///
/// err!("this string has a newline, why not choose errln! instead?\n");
///
/// io::stderr().flush().unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{
        use std::io::Write;
        write!(&mut ::std::io::stderr(), $($arg)*).unwrap()
    }};
}

/// Macro for printing to the standard err, with a newline. On all
/// platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
/// (no additional CARRIAGE RETURN (`\r`/`U+000D`).
///
/// Use the `format!` syntax to write data to the standard output.
/// See `std::fmt` for more information.
///
/// # Panics
///
/// Panics if writing to `io::stderr()` fails.
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
/// # }
/// ```
#[macro_export]
macro_rules! errln {
    ($fmt:expr) => (err!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (err!(concat!($fmt, "\n"), $($arg)*));
}
