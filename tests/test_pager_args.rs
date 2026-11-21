mod common;
#[path = "common/fixture.rs"]
mod fixture;
#[path = "common/search_path.rs"]
mod search_path;

use lessify::{PAGER, Pager};
use std::path::Path;
use std::process::Command;

const PAGER_WITH_ARGS: &str = r#"less -S -- "two words" three\ four"#;

#[test]
fn pager_with_arguments_in_env() {
    common::run("pager_with_arguments_in_env", configure_pager, || {
        assert_eq!(*PAGER, PAGER_WITH_ARGS);
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

            assert_eq!(args, "-R\n-F\n-X\n-S\n--\n\"two\nwords\"\nthree\\\nfour\n");
            assert_eq!(lesscharset, "UTF-8\n");
        },
    );
}

#[test]
fn page_with_preserves_argument_boundaries() {
    common::run(
        "page_with_preserves_argument_boundaries",
        |command, dir| {
            let program = fixture::install(&dir.join("my pager"), "less");
            command.env("TEST_PAGER", program);
            command.env("PAGER", dir.join("missing-pager"));
        },
        || {
            let program = std::env::var_os("TEST_PAGER").unwrap();
            Pager::page_with("hello", program, ["--", "two words", ""]).unwrap();

            assert_eq!(std::fs::read("stdin").unwrap(), b"hello\n");
            assert_eq!(
                std::fs::read("args").unwrap(),
                b"-R\n-F\n-X\n--\ntwo words\n\n"
            );
        },
    );
}

#[cfg(unix)]
#[test]
fn page_with_preserves_non_unicode_argument() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    common::run(
        "page_with_preserves_non_unicode_argument",
        |_, dir| {
            fixture::install(dir, "capture");
        },
        || {
            Pager::page_with("hello", "./capture", [OsStr::from_bytes(b"\xff")]).unwrap();

            assert_eq!(std::fs::read("args").unwrap(), b"\xff\n");
        },
    );
}

#[cfg(unix)]
#[test]
fn page_passes_long_argument_intact() {
    let argument = "x".repeat(300);
    common::run(
        "page_passes_long_argument_intact",
        |command, dir| {
            fixture::install(dir, "capture");
            command.env("PAGER", format!("./capture {argument}"));
        },
        || {
            Pager::page("hello").unwrap();

            let args = std::fs::read_to_string("args").unwrap();
            assert_eq!(args, format!("{argument}\n"));
        },
    );
}

fn configure_pager(command: &mut Command, dir: &Path) {
    search_path::install_pager(command, dir, "less");
    command.env("PAGER", PAGER_WITH_ARGS);
    // Ensure the crate overrides the inherited value.
    command.env("LESSCHARSET", "inherited-value");
}
