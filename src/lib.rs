//! Output text through a pager.
//!
//! It uses `less` by default, or any pager set by the `PAGER`
//! environment variable.
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

use std::env;
use std::fmt;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::LazyLock;

/// Pager to use, lazily determined.
///
/// The logic is as follows:
///
/// 1. Look for `PAGER` in the environment.
/// 2. If not set, default to `less`.
pub static PAGER: LazyLock<String> =
    LazyLock::new(|| env::var("PAGER").unwrap_or_else(|_| String::from("less")));

/// Output text through a pager.
pub struct Pager;

impl Pager {
    /// Output `content` with default pager or print to stdout on error.
    ///
    /// This is a helper function for the common case where you don't
    /// really care whether the pager succeeded or not. Worst case
    /// scenario just print to stdout, no big deal.
    pub fn page_or_print(content: &str) {
        if Self::page(content).is_err() {
            if content.ends_with('\n') {
                print!("{content}");
            } else {
                println!("{content}");
            }
        }
    }

    /// Try to use default pager to output `content`.
    ///
    /// The pager is read from the `PAGER` environment variable, or
    /// defaults to `less`.
    ///
    /// # Errors
    ///
    /// Errors if the pager cannot be spawned (e.g., executable
    /// missing), or stdin cannot be captured or written to.
    pub fn page(content: &str) -> Result<(), io::Error> {
        let mut pager = Command::new(&*PAGER);
        pager.stdin(Stdio::piped());
        pager.stdout(Stdio::inherit());
        pager.stderr(Stdio::inherit());

        #[cfg(not(tarpaulin_include))]
        {
            if *PAGER == "less" || PAGER.ends_with("/less") {
                pager.env("LESSCHARSET", "UTF-8");
                // Use short args for better compatibility.
                pager.arg("-R"); // `--RAW-CONTROL-CHARS` Do not render ANSI sequences as text.
                pager.arg("-F"); // `--quit-if-one-screen` Do not page if the entire output fits on the screen.
            }
        }

        let mut child = pager.spawn()?;

        let Some(stdin) = child.stdin.as_mut() else {
            #[cfg(not(tarpaulin_include))]
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Failed to open stdin.",
            ));
        };

        if content.ends_with('\n') {
            write!(stdin, "{content}")?;
        } else {
            writeln!(stdin, "{content}")?;
        }

        child.wait()?;

        Ok(())
    }
}

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
