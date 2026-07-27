use lessify::{PAGER, Pager};
use std::env;
use std::path::PathBuf;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
const TMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");

// `PAGER` is initialized once per execution, so we need another bin to
// test a pager name containing spaces.
fn set_up_pager(test_name: &str) -> (PathBuf, PathBuf) {
    let stdin = format!("{TMP_DIR}/{test_name}");
    let args = format!("{TMP_DIR}/{test_name}.args");
    let path = env::var("PATH").unwrap_or_default();
    unsafe {
        env::set_var("PATH", format!("{FIXTURES_DIR}/pathbin:{path}"));
        env::set_var("PAGER", "my pager");
        env::set_var("TEST_OUTPUT_FILE", &stdin);
        env::set_var("TEST_ARGS_FILE", &args);
    }
    (PathBuf::from(stdin), PathBuf::from(args))
}

#[test]
fn pager_spaced_name_in_env() {
    set_up_pager("pager_spaced_name_in_env");

    assert_eq!(*PAGER, "my pager");
}

#[test]
fn page_resolves_spaced_name_through_path() {
    let (stdin_file, args_file) = set_up_pager("page_resolves_spaced_name_through_path");

    Pager::page("hello\nworld\n").unwrap();

    let stdin = std::fs::read_to_string(&stdin_file).unwrap();
    let args = std::fs::read_to_string(&args_file).unwrap();

    assert_eq!(stdin, "hello\nworld\n");

    assert_eq!(args, "\n");
}
