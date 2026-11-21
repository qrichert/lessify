mod common;
#[path = "common/fixture.rs"]
mod fixture;

use lessify::{PAGER, Pager};

const DEFAULT_PAGER: &str = if cfg!(windows) { "more.com" } else { "less" };

#[test]
fn default_pager() {
    common::run(
        "default_pager",
        |command, _| {
            command.env_remove("PAGER");
        },
        || assert_eq!(*PAGER, DEFAULT_PAGER),
    );
}

#[cfg(any(unix, windows))]
#[test]
fn non_unicode_pager_uses_default() {
    #[cfg(unix)]
    let value = <std::ffi::OsStr as std::os::unix::ffi::OsStrExt>::from_bytes(b"\xff");
    #[cfg(windows)]
    let value = <std::ffi::OsString as std::os::windows::ffi::OsStringExt>::from_wide(&[0xd800]);

    common::run(
        "non_unicode_pager_uses_default",
        |command, _| {
            command.env("PAGER", value);
        },
        || assert_eq!(*PAGER, DEFAULT_PAGER),
    );
}

#[test]
fn empty_pager_fails_to_start() {
    assert_pager_fails_to_start("empty_pager_fails_to_start", "");
}

#[test]
fn whitespace_only_pager_fails_to_start() {
    assert_pager_fails_to_start("whitespace_only_pager_fails_to_start", " \t ");
}

fn assert_pager_fails_to_start(test_name: &str, value: &str) {
    common::run(
        test_name,
        |command, dir| {
            // A fallback to the platform default would succeed.
            let recorder = fixture::install(dir, "recorder");
            std::fs::rename(recorder, dir.join(DEFAULT_PAGER)).unwrap();
            command.env("PATH", dir).env("PAGER", value);
        },
        || {
            assert_eq!(*PAGER, value);
            assert!(Pager::page("hello").is_err());
        },
    );
}
