use std::env::consts::EXE_SUFFIX;
use std::fs::{self, Permissions};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

struct Recorder {
    bytes: Vec<u8>,
    permissions: Permissions,
}

pub fn install(dir: &Path, name: &str) -> PathBuf {
    static RECORDER: OnceLock<Recorder> = OnceLock::new();

    fs::create_dir_all(dir).expect("create the pager fixture directory");
    let recorder = RECORDER.get_or_init(|| compile(dir));
    let executable = dir.join(format!("{name}{EXE_SUFFIX}"));
    fs::write(&executable, &recorder.bytes).expect("write the pager fixture");
    fs::set_permissions(&executable, recorder.permissions.clone())
        .expect("set the pager fixture permissions");
    executable
}

fn compile(dir: &Path) -> Recorder {
    let executable = dir.join(format!("pager-recorder-build{EXE_SUFFIX}"));
    let output = Command::new(env!("CARGO"))
        .args([
            "test",
            "--quiet",
            "--locked",
            "--test",
            "pager-recorder",
            "--target-dir",
        ])
        .arg(Path::new(env!("CARGO_TARGET_TMPDIR")).join("pager-recorder-target"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("LESSIFY_TEST_INSTALL_RECORDER", &executable)
        .output()
        .expect("run Cargo for the pager fixture");
    assert!(
        output.status.success(),
        "Cargo failed to install the pager fixture ({}):\n{}{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    Recorder {
        bytes: fs::read(&executable).expect("read the compiled pager fixture"),
        permissions: fs::metadata(&executable)
            .expect("read the compiled pager fixture permissions")
            .permissions(),
    }
}
