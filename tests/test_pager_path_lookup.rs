mod common;

use lessify::{PAGER, Pager};
use std::env;
use std::process::Command;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn pager_spaced_name_in_env() {
    common::run("pager_spaced_name_in_env", configure_pager, || {
        assert_eq!(*PAGER, "my pager");
    });
}

#[test]
fn page_resolves_spaced_name_through_path() {
    common::run(
        "page_resolves_spaced_name_through_path",
        configure_pager,
        || {
            Pager::page("hello\nworld\n").unwrap();

            let stdin = std::fs::read_to_string("stdin").unwrap();
            let args = std::fs::read_to_string("args").unwrap();

            assert_eq!(stdin, "hello\nworld\n");

            assert_eq!(args, "\n");
        },
    );
}

fn configure_pager(command: &mut Command) {
    let path = env::var("PATH").unwrap_or_default();
    command
        .env("PATH", format!("{FIXTURES_DIR}/pathbin:{path}"))
        .env("PAGER", "my pager");
}
