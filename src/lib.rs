use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::LazyLock;

pub static PAGER: LazyLock<String> =
    LazyLock::new(|| env::var("PAGER").unwrap_or_else(|_| String::from("less")));

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
    /// Errors if the pager cannot be spawn (e.g., executable missing),
    /// or stdin cannot be written to.
    pub fn page(content: &str) -> Result<(), io::Error> {
        let mut pager = Command::new(&*PAGER);
        pager.stdin(Stdio::piped());
        pager.stdout(Stdio::inherit());
        pager.stderr(Stdio::inherit());

        if *PAGER == "less" || PAGER.ends_with("/less") {
            pager.env("LESSCHARSET", "UTF-8");
            // Use short args for better compatibility.
            pager.arg("-R"); // `--RAW-CONTROL-CHARS` Do not render ANSI sequences as text.
            pager.arg("-F"); // `--quit-if-one-screen` Do not page if the entire output fits on the screen.
        }

        let mut child = pager.spawn()?;

        let Some(stdin) = child.stdin.as_mut() else {
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
