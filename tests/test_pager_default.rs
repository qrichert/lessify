mod common;

use lessify::PAGER;

#[test]
fn default_pager() {
    common::run(
        "default_pager",
        |command| {
            command.env_remove("PAGER");
        },
        || assert_eq!(*PAGER, "less"),
    );
}
