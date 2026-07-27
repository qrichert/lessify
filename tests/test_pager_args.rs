use lessify::{PAGER, Pager};
use std::env;
use std::path::PathBuf;

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
const TMP_DIR: &str = env!("CARGO_TARGET_TMPDIR");

struct Outputs {
    stdin: PathBuf,
    args: PathBuf,
    lesscharset: PathBuf,
}

// `PAGER` is initialized once per execution, so we need another bin to
// test a value with arguments.
fn set_up_pager(test_name: &str) -> Outputs {
    let stdin = format!("{TMP_DIR}/{test_name}");
    let args = format!("{TMP_DIR}/{test_name}.args");
    let lesscharset = format!("{TMP_DIR}/{test_name}.lesscharset");
    unsafe {
        env::set_var("PAGER", format!("{FIXTURES_DIR}/less -S --"));
        env::set_var("TEST_OUTPUT_FILE", &stdin);
        env::set_var("TEST_ARGS_FILE", &args);
        env::set_var("TEST_LESSCHARSET_FILE", &lesscharset);
        // Ensure the crate overrides the inherited value.
        env::set_var("LESSCHARSET", "inherited-value");
    }
    Outputs {
        stdin: PathBuf::from(stdin),
        args: PathBuf::from(args),
        lesscharset: PathBuf::from(lesscharset),
    }
}

#[test]
fn pager_with_arguments_in_env() {
    set_up_pager("pager_with_arguments_in_env");

    assert_eq!(*PAGER, format!("{FIXTURES_DIR}/less -S --"));
}

#[test]
fn page_splits_arguments_and_detects_less() {
    let out = set_up_pager("page_splits_arguments_and_detects_less");

    Pager::page("hello\nworld\n").unwrap();

    let stdin = std::fs::read_to_string(&out.stdin).unwrap();
    let args = std::fs::read_to_string(&out.args).unwrap();
    let lesscharset = std::fs::read_to_string(&out.lesscharset).unwrap();

    assert_eq!(stdin, "hello\nworld\n");

    assert_eq!(args, "-R\n-F\n-X\n-S\n--\n");
    assert_eq!(lesscharset, "UTF-8\n");
}
