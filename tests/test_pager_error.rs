mod common;

use lessify::{PAGER, Pager};
use std::process::Command;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn failing_pager_in_env() {
    common::run("failing_pager_in_env", configure_pager, || {
        assert_eq!(*PAGER, format!("{FIXTURES_DIR}/pager-fail"));
    });
}

// The `or_print` in `page_or_print()` is by definition a failing case.

#[test]
fn page_or_print_with_newline() {
    common::run("page_or_print_with_newline", configure_pager, || {
        // Can't really test this, but we see it work in coverage report.
        Pager::page_or_print("hello\nworld\n");
    });
}

#[test]
fn page_or_print_without_newline() {
    common::run("page_or_print_without_newline", configure_pager, || {
        // Can't really test this, but we see it work in coverage report.
        Pager::page_or_print("hello\nworld");
    });
}

#[test]
fn page_cannot_spawn_pager() {
    common::run("page_cannot_spawn_pager", configure_pager, || {
        let res = Pager::page("foo bar baz");

        dbg!(&res);

        assert!(res.is_err());
    });
}

fn configure_pager(command: &mut Command) {
    command.env("PAGER", format!("{FIXTURES_DIR}/pager-fail"));
}
