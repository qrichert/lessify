use lessify::{Pager, PAGER};
use std::env;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

// `PAGER` is initialized once per execution, so we need another bin to
// test with failing mock pager.
fn set_up_pager() {
    unsafe {
        env::set_var("PAGER", format!("{FIXTURES_DIR}/pager-fail"));
    }
}

#[test]
fn failing_pager_in_env() {
    set_up_pager();

    assert_eq!(*PAGER, format!("{FIXTURES_DIR}/pager-fail"));
}

// The `or_print` in `page_or_print()` is by definition a failing case.

#[test]
fn page_or_print_with_newline() {
    set_up_pager();

    // Can't really test this, but we see it work in coverage report.
    Pager::page_or_print("hello\nworld\n");
}

#[test]
fn page_or_print_without_newline() {
    set_up_pager();

    // Can't really test this, but we see it work in coverage report.
    Pager::page_or_print("hello\nworld");
}

#[test]
fn page_cannot_spawn_pager() {
    set_up_pager();

    let res = Pager::page("foo bar baz");

    dbg!(&res);

    assert!(res.is_err());
}
