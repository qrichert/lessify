use lessify::{Pager, PAGER};
use std::env;
use std::path::PathBuf;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
const TMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");

/// Pager is a mock script that parrots stdin.
///
/// Mutating the environment is not a problem because tests are ran
/// sequentially (`--test-threads 1`).
fn set_up_pager(test_name: &str) {
    unsafe {
        env::set_var("PAGER", format!("{FIXTURES_DIR}/pager"));
        env::set_var("TEST_OUTPUT_FILE", format!("{TMP_DIR}/{test_name}"));
    }
}

#[test]
fn pager_in_env() {
    set_up_pager("pager_in_env");

    assert_eq!(*PAGER, format!("{FIXTURES_DIR}/pager"));
}

#[test]
fn page_or_print_with_newline() {
    set_up_pager("page_or_print_with_newline");

    Pager::page_or_print("hello\nworld\n");

    let res = PathBuf::from(TMP_DIR).join("page_or_print_with_newline");
    let res = std::fs::read_to_string(res).unwrap();

    assert_eq!(res, "hello\nworld\n");
}

#[test]
fn page_or_print_without_newline() {
    set_up_pager("page_or_print_without_newline");

    Pager::page_or_print("hello\nworld");

    let res = PathBuf::from(TMP_DIR).join("page_or_print_without_newline");
    let res = std::fs::read_to_string(res).unwrap();

    assert_eq!(res, "hello\nworld\n");
}

#[test]
fn page_with_newline() {
    set_up_pager("page_with_newline");

    Pager::page("hello\nworld\n").unwrap();

    let res = PathBuf::from(TMP_DIR).join("page_with_newline");
    let res = std::fs::read_to_string(res).unwrap();

    assert_eq!(res, "hello\nworld\n");
}

#[test]
fn page_without_newline() {
    set_up_pager("page_without_newline");

    Pager::page("hello\nworld").unwrap();

    let res = PathBuf::from(TMP_DIR).join("page_without_newline");
    let res = std::fs::read_to_string(res).unwrap();

    assert_eq!(res, "hello\nworld\n");
}
