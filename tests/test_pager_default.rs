mod common;

use lessify::PAGER;

#[test]
fn default_pager() {
    common::run(
        "default_pager",
        |command, _| {
            command.env_remove("PAGER");
        },
        || assert_eq!(*PAGER, if cfg!(windows) { "more.com" } else { "less" }),
    );
}
