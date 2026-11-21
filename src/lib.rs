//! Output text through a pager.
//!
//! It uses `less` by default (`more.com` on Windows), or any pager set
//! by the [`PAGER`] environment variable.
//!
//! The points of interest are the [`Pager`] struct, and the
//! [`OutputPaged`] trait.
//!
//! # Examples
//!
//! Manually:
//!
//! ```no_run
//! use lessify::Pager;
//!
//! // If pager fails, fall back to printing text.
//! Pager::page_or_print("very long text");
//! ```
//!
//! Or with the trait (blanket implementation for `T: Display`):
//!
//! ```no_run
//! use lessify::OutputPaged;
//!
//! // Same as `page_or_print()`.
//! "very long text".output_paged();
//! ```
//!
//! # Windows Notes
//!
//! The default pager on Windows is set to `more.com`. Unfortunately,
//! at the time of writing `more` is legacy software which does not
//! understand multi-byte characters. This leads to mojibake.
//!
//! Therefore, when using this crate on Windows, it is advisable to
//! restrict what you print to the ASCII character set.
//!
//! Possible alternative solutions include:
//! - Reading the active console codepage and converting the output.
//! - Setting the console codepage, if this is appropriate.

use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::io::{self, IsTerminal, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::LazyLock;

/// Pager to use, lazily determined.
///
/// The logic is as follows:
///
/// 1. Look for `PAGER` in the environment.
/// 2. If not set, default to `less` on Unix or `more.com` on Windows.
///
/// If `PAGER` cannot be found as-is or the filename is invalid, split
/// it on whitespace into the pager and its arguments (e.g., `less -S`).
/// Trying it as-is first allows pager paths containing spaces. When
/// splitting, quotes and backslashes are literal characters. Use
/// [`Pager::page_with`] to pass arguments containing spaces.
///
/// An empty value produces a spawn error. A whitespace-only value does
/// too unless it names an executable.
///
/// For an executable named `less`, the crate passes `-R -F -X` before
/// any user arguments and sets `LESSCHARSET=UTF-8` for the child. On
/// Windows, it recognizes both `less` and `less.exe`, ignoring case.
pub static PAGER: LazyLock<String> = LazyLock::new(|| {
    env::var("PAGER").unwrap_or_else(|_| {
        if cfg!(windows) {
            #[cfg(not(tarpaulin_include))]
            String::from("more.com")
        } else {
            String::from("less")
        }
    })
});

/// Output text through a pager.
pub struct Pager;

impl Pager {
    /// Output `content` with default pager or print to stdout on error.
    ///
    /// This is a helper function for the common case where you don't
    /// really care whether the pager succeeded or not. Worst case, just
    /// print to stdout.
    ///
    /// A successful early quit skips the fallback to stdout. On
    /// failure, the fallback prints all of `content`, including any
    /// text the pager already displayed. It follows the same trailing
    /// newline rule as [`Self::page`].
    ///
    /// When stdout is redirected, write directly to it without starting
    /// the pager.
    pub fn page_or_print(content: &str) {
        let mut stdout = io::stdout();
        if !stdout.is_terminal() || Self::page(content).is_err() {
            // Using write! instead of print! fixes a panic in some circumstances.
            if content.ends_with('\n') {
                let _ = write!(stdout, "{content}");
            } else {
                let _ = writeln!(stdout, "{content}");
            }
        }
    }

    /// Send `content` to the pager selected by [`PAGER`].
    ///
    /// Appends `\n` if `content` does not already end in one. For empty
    /// input, the pager receives a single `\n`.
    ///
    /// # Errors
    ///
    /// Returns an error if spawning the pager, capturing its stdin,
    /// writing to it, or waiting for it fails. A broken pipe during a
    /// write is treated as a successful early quit if the pager exits
    /// successfully. If the pager fails after the write completes, the
    /// error includes its exit status.
    pub fn page(content: &str) -> Result<(), io::Error> {
        // Try `PAGER` as-is first to support paths containing spaces.
        // If the name cannot be found or is invalid, try splitting it
        // into pager and args.
        let child = match spawn_pager(PAGER.as_str(), [] as [&str; 0]) {
            Err(e)
                if matches!(
                    e.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::InvalidFilename
                ) && PAGER.contains(char::is_whitespace) =>
            {
                let mut parts = PAGER.split_whitespace();
                let program = parts.next().unwrap_or_default();
                spawn_pager(program, parts)?
            }
            result => result?,
        };

        write_to_pager(child, content)
    }

    /// Send `content` to a custom pager command.
    ///
    /// `program` selects the executable independently of [`PAGER`].
    /// Each item in `args` is passed separately with [`Command::args`],
    /// including empty arguments and arguments containing spaces. The
    /// usual `less` defaults and trailing newline rule apply.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use lessify::Pager;
    ///
    /// Pager::page_with("very long text", "/opt/my/pager", ["-p", "release notes"])?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// See [`Self::page`] for errors and early-exit behavior.
    pub fn page_with(
        content: &str,
        program: impl AsRef<OsStr>,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> io::Result<()> {
        let child = spawn_pager(program, args)?;
        write_to_pager(child, content)
    }
}

/// Page values that implement [`Display`](fmt::Display) using [`Pager::page_or_print`].
#[cfg(not(tarpaulin_include))]
pub trait OutputPaged {
    /// Output `content` with default pager or print to stdout on error.
    ///
    /// This is the same as [`Pager::page_or_print()`], but through a
    /// trait method. This can lead to nicer syntax in builder-like
    /// contexts (no need to assign to a variable first, or nest calls).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # struct Foo;
    /// # impl Foo {
    /// #     fn foo() -> Self { Self {} }
    /// #     fn barbaz(self) -> String { String::new() }
    /// # }
    /// use lessify::OutputPaged;
    ///
    /// Foo::foo()
    ///     .barbaz()
    ///     .output_paged();
    /// ```
    fn output_paged(&self) {}
}

impl<T> OutputPaged for T
where
    T: fmt::Display,
{
    fn output_paged(&self) {
        Pager::page_or_print(&self.to_string());
    }
}

/// Spawn pager with arguments.
fn spawn_pager(
    program: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> io::Result<Child> {
    let program = program.as_ref();
    let mut pager = Command::new(program);

    #[cfg(not(tarpaulin_include))]
    {
        if is_less(program) {
            pager.env("LESSCHARSET", "UTF-8");
            // Add defaults first in case user args contain `--`.
            // Use short args for better compatibility.
            pager.arg("-R"); // `--RAW-CONTROL-CHARS` Do not render ANSI sequences as text.
            pager.arg("-F"); // `--quit-if-one-screen` Do not page if the entire output fits on the screen.
            pager.arg("-X"); // `--no-init` Leave content on screen after exit.
        }
    }

    pager.args(args);
    pager.stdin(Stdio::piped());
    pager.stdout(Stdio::inherit());
    pager.stderr(Stdio::inherit());

    pager.spawn()
}

fn is_less(program: &OsStr) -> bool {
    Path::new(program).file_name().is_some_and(|name| {
        #[cfg(windows)]
        {
            name.eq_ignore_ascii_case("less") || name.eq_ignore_ascii_case("less.exe")
        }
        #[cfg(not(windows))]
        {
            name == "less"
        }
    })
}

fn write_to_pager(mut child: Child, content: &str) -> io::Result<()> {
    let write_result = child.stdin.as_mut().map(|stdin| {
        if content.ends_with('\n') {
            write!(stdin, "{content}")
        } else {
            writeln!(stdin, "{content}")
        }
    });

    let status = child.wait()?;
    match write_result {
        Some(Err(error)) if error.kind() == io::ErrorKind::BrokenPipe && status.success() => Ok(()),
        Some(Ok(())) if !status.success() => {
            Err(io::Error::other(format!("pager failed with {status}")))
        }
        Some(result) => result,
        None => Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "Failed to open stdin.",
        )),
    }
}

#[test]
fn recognizes_less_by_filename() {
    assert!(is_less(OsStr::new("less")));
    for program in ["less.exe", "LESS", r"C:\Tools\Less.EXE"] {
        assert_eq!(is_less(OsStr::new(program)), cfg!(windows), "{program}");
    }
    assert!(!is_less(OsStr::new("myless.exe")));
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn accepts_successful_early_quit() {
        assert!(write_to_early_quitting_pager("0").is_ok());
    }

    #[test]
    fn preserves_broken_pipe_after_unsuccessful_early_exit() {
        let error = write_to_early_quitting_pager("7").unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn reports_unsuccessful_exit_after_completed_write() {
        let child = Command::new("sh")
            .args(["-c", "cat >/dev/null && exit 7"])
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();

        let error = write_to_pager(child, "document\n").unwrap_err();
        assert!(error.to_string().contains("exit status: 7"), "{error}");
    }

    fn write_to_early_quitting_pager(exit_code: &str) -> io::Result<()> {
        let child = Command::new("sh")
            .args(["-c", "IFS= read -r line && exit \"$1\"", "sh", exit_code])
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        let content = "line\n".repeat(1_000_000);

        write_to_pager(child, &content)
    }
}
