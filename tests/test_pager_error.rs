mod common;
#[cfg(unix)]
#[path = "common/fixture.rs"]
mod fixture;

use lessify::{PAGER, Pager};
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{self, Child, Command, Stdio};

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
const FALLBACK_CHILD: &str = "LESSIFY_FALLBACK_CHILD";
const STDOUT_MARKER: u8 = 0;

#[test]
fn failing_pager_in_env() {
    common::run(
        "failing_pager_in_env",
        |command, _| configure_pager(command),
        || {
            assert_eq!(*PAGER, format!("{FIXTURES_DIR}/pager-fail"));
        },
    );
}

#[test]
fn page_or_print_with_newline() {
    assert_fallback_stdout("page_or_print_with_newline", "hello\n", b"hello\n");
}

#[test]
fn page_or_print_without_newline() {
    assert_fallback_stdout("page_or_print_without_newline", "hello", b"hello\n");
}

#[test]
fn page_or_print_empty() {
    assert_fallback_stdout("page_or_print_empty", "", b"\n");
}

#[test]
fn page_or_print_with_closed_stdout() {
    let mut child = start_fallback_caller("page_or_print_with_closed_stdout", "hello");
    // Close the reader before wait closes stdin and releases the child.
    drop(child.stdout.take());
    let status = child.wait().expect("wait for the fallback caller");
    assert!(status.success(), "fallback caller exited with {status}");
}

#[test]
fn page_cannot_spawn_pager() {
    common::run(
        "page_cannot_spawn_pager",
        |command, _| configure_pager(command),
        || {
            let res = Pager::page("foo bar baz");

            dbg!(&res);

            assert!(res.is_err());
        },
    );
}

#[cfg(unix)]
#[test]
fn page_preserves_permission_denied_for_literal_pager() {
    common::run(
        "page_preserves_permission_denied_for_literal_pager",
        |command, dir| {
            fixture::install(dir, "capture");
            command.env("PAGER", "./capture denied");
        },
        || {
            std::fs::write("capture denied", []).unwrap();

            let error = Pager::page("hello").unwrap_err();

            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
            assert!(!std::path::Path::new("stdin").exists());
        },
    );
}

fn assert_fallback_stdout(test_name: &str, content: &str, expected: &[u8]) {
    let output = start_fallback_caller(test_name, content)
        .wait_with_output()
        .expect("collect the fallback caller's stdout");
    assert!(
        output.status.success(),
        "fallback caller exited with {}",
        output.status
    );
    assert_eq!(output.stdout, expected);
}

fn start_fallback_caller(test_name: &str, content: &str) -> Child {
    if env::var(FALLBACK_CHILD).as_deref() == Ok(test_name) {
        let mut stdout = io::stdout();
        stdout
            .write_all(&[STDOUT_MARKER])
            .expect("write stdout marker");
        stdout.flush().expect("flush stdout marker");
        io::copy(&mut io::stdin().lock(), &mut io::sink()).expect("wait for stdin EOF");

        Pager::page_or_print(content);
        // Exit before libtest writes its footer to stdout.
        process::exit(0);
    }

    let mut command = Command::new(env::current_exe().expect("locate the test executable"));
    configure_pager(&mut command);
    let mut child = command
        .args(["--exact", test_name, "--nocapture"])
        .env(FALLBACK_CHILD, test_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("start the fallback caller");

    // Discard libtest's startup output. The child writes the fallback after stdin EOF.
    let mut startup = Vec::new();
    BufReader::new(child.stdout.as_mut().expect("borrow the caller's stdout"))
        .read_until(STDOUT_MARKER, &mut startup)
        .expect("read stdout marker");
    if startup.last() != Some(&STDOUT_MARKER) {
        let status = child.wait().expect("wait for the fallback caller");
        panic!("fallback caller for {test_name} exited with {status} before writing its marker");
    }

    child
}

fn configure_pager(command: &mut Command) {
    command.env("PAGER", format!("{FIXTURES_DIR}/pager-fail"));
}
