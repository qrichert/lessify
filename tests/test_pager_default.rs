use lessify::PAGER;
use std::env;

// `PAGER` is initialized once per execution, so we need another bin to
// test the un-mocked default value (not used in other tests).
#[test]
fn default_pager() {
    unsafe {
        env::remove_var("PAGER");
    }

    assert_eq!(*PAGER, "less");
}
