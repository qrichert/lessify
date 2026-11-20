use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const CHILD_TEST: &str = "LESSIFY_TEST_CHILD";
const COMPLETION_FILE: &str = "completed";

pub fn run(test_name: &str, configure: impl FnOnce(&mut Command), test: impl FnOnce()) {
    if env::var(CHILD_TEST).as_deref() == Ok(test_name) {
        test();
        fs::write(COMPLETION_FILE, []).expect("record test completion");
        return;
    }

    let mut command = Command::new(env::current_exe().expect("locate the test executable"));
    configure(&mut command);

    let dir = reserve_test_dir(test_name);
    let output = command
        .args(["--exact", test_name, "--nocapture"])
        .current_dir(&dir)
        .env(CHILD_TEST, test_name)
        .env("TEST_OUTPUT_FILE", "stdin")
        .env("TEST_ARGS_FILE", "args")
        .env("TEST_LESSCHARSET_FILE", "lesscharset")
        .output();
    // Libtest exits successfully even if the filter selects zero tests.
    let completed = dir.join(COMPLETION_FILE).is_file();
    let cleanup = fs::remove_dir_all(&dir);

    let output = output.expect("run the integration test in a child process");
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
    assert!(
        output.status.success() && completed,
        "child for {test_name} exited with {} (completion recorded: {completed})",
        output.status,
    );
    cleanup.unwrap_or_else(|error| panic!("could not remove {}: {error}", dir.display()));
}

fn reserve_test_dir(test_name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR"));
    fs::create_dir_all(root).expect("create Cargo's temporary directory");

    let pid = process::id();
    let mut attempt = 0;
    loop {
        let dir = root.join(format!("{test_name}-{pid}-{attempt}"));
        match fs::create_dir(&dir) {
            Ok(()) => return dir,
            Err(error) if error.kind() == ErrorKind::AlreadyExists => attempt += 1,
            Err(error) => panic!("could not create {}: {error}", dir.display()),
        }
    }
}
