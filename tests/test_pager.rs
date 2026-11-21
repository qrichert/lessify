mod common;
#[path = "common/fixture.rs"]
mod fixture;

use lessify::{OutputPaged, PAGER, Pager};
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[test]
fn pager_in_env() {
    common::run("pager_in_env", configure_pager, || {
        assert_eq!(*PAGER, env::var("PAGER").unwrap());
    });
}

#[test]
fn page_or_print_skips_pager_for_pipe() {
    common::run(
        "page_or_print_skips_pager_for_pipe",
        configure_pager,
        || {
            Pager::page_or_print("hello\nworld");

            assert!(!Path::new("stdin").exists());
        },
    );
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
fn page_empty() {
    common::run("page_empty", configure_pager, || {
        Pager::page("").unwrap();

        let res = std::fs::read("stdin").unwrap();

        assert_eq!(res, b"\n");
    });
}

#[test]
fn output_paged_writes_to_redirected_file() {
    common::run(
        "output_paged_writes_to_redirected_file",
        |command, dir| {
            configure_pager(command, dir);
            command.stdout(std::fs::File::create(dir.join("stdout")).unwrap());
        },
        || {
            let content = "very\nlong\nstring\n";
            content.output_paged();
            io::stdout().flush().unwrap();

            let stdout = std::fs::read_to_string("stdout").unwrap();
            assert!(stdout.contains(content), "{stdout:?}");
            assert!(!Path::new("stdin").exists());
        },
    );
}

// Pager is a mock executable that parrots stdin.
fn configure_pager(command: &mut Command, dir: &Path) {
    command.env("PAGER", fixture::install(dir, "pager"));
}
