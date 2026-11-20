mod common;
#[path = "common/fixture.rs"]
mod fixture;
#[path = "common/search_path.rs"]
mod search_path;

use lessify::{PAGER, Pager};
use std::path::Path;
use std::process::Command;

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

            assert_eq!(args, "");
        },
    );
}

fn configure_pager(command: &mut Command, dir: &Path) {
    search_path::install_pager(command, dir, "my pager");
    command.env("PAGER", "my pager");
}
