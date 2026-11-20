mod common;

use lessify::{PAGER, Pager};
use std::process::Command;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn pager_with_arguments_in_env() {
    common::run("pager_with_arguments_in_env", configure_pager, || {
        assert_eq!(*PAGER, format!("{FIXTURES_DIR}/less -S --"));
    });
}

#[test]
fn page_splits_arguments_and_detects_less() {
    common::run(
        "page_splits_arguments_and_detects_less",
        configure_pager,
        || {
            Pager::page("hello\nworld\n").unwrap();

            let stdin = std::fs::read_to_string("stdin").unwrap();
            let args = std::fs::read_to_string("args").unwrap();
            let lesscharset = std::fs::read_to_string("lesscharset").unwrap();

            assert_eq!(stdin, "hello\nworld\n");

            assert_eq!(args, "-R\n-F\n-X\n-S\n--\n");
            assert_eq!(lesscharset, "UTF-8\n");
        },
    );
}

fn configure_pager(command: &mut Command) {
    command.env("PAGER", format!("{FIXTURES_DIR}/less -S --"));
    // Ensure the crate overrides the inherited value.
    command.env("LESSCHARSET", "inherited-value");
}
