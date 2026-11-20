mod common;

use lessify::{OutputPaged, PAGER, Pager};
use std::process::Command;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn pager_in_env() {
    common::run("pager_in_env", configure_pager, || {
        assert_eq!(*PAGER, format!("{FIXTURES_DIR}/pager"));
    });
}

#[test]
fn page_or_print_with_newline() {
    common::run("page_or_print_with_newline", configure_pager, || {
        Pager::page_or_print("hello\nworld\n");

        let res = std::fs::read_to_string("stdin").unwrap();

        assert_eq!(res, "hello\nworld\n");
    });
}

#[test]
fn page_or_print_without_newline() {
    common::run("page_or_print_without_newline", configure_pager, || {
        Pager::page_or_print("hello\nworld");

        let res = std::fs::read_to_string("stdin").unwrap();

        assert_eq!(res, "hello\nworld\n");
    });
}

#[test]
fn page_with_newline() {
    common::run("page_with_newline", configure_pager, || {
        Pager::page("hello\nworld\n").unwrap();

        let res = std::fs::read_to_string("stdin").unwrap();

        assert_eq!(res, "hello\nworld\n");
    });
}

#[test]
fn page_without_newline() {
    common::run("page_without_newline", configure_pager, || {
        Pager::page("hello\nworld").unwrap();

        let res = std::fs::read_to_string("stdin").unwrap();

        assert_eq!(res, "hello\nworld\n");
    });
}

#[test]
fn output_paged_trait() {
    common::run("output_paged_trait", configure_pager, || {
        "very\nlong\nstring\n".output_paged();

        let res = std::fs::read_to_string("stdin").unwrap();

        assert_eq!(res, "very\nlong\nstring\n");
    });
}

// Pager is a mock script that parrots stdin.
fn configure_pager(command: &mut Command) {
    command.env("PAGER", format!("{FIXTURES_DIR}/pager"));
}
