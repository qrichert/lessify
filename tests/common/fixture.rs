use std::env::{self, consts::EXE_SUFFIX};
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
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pager.rs");
    let executable = dir.join(format!("pager-recorder-build{EXE_SUFFIX}"));
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = Command::new(rustc)
        .arg("--edition=2024")
        .arg(source)
        .arg("-o")
        .arg(&executable)
        .output()
        .expect("run rustc for the pager fixture");
    assert!(
        output.status.success(),
        "rustc failed to compile the pager fixture ({}):\n{}{}",
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
