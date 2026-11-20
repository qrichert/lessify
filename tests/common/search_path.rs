use std::env;
use std::path::Path;
use std::process::Command;

pub fn install_pager(command: &mut Command, dir: &Path, name: &str) {
    let bin = dir.join("bin");
    crate::fixture::install(&bin, name);

    let mut paths = vec![bin];
    if let Some(path) = env::var_os("PATH") {
        paths.extend(env::split_paths(&path));
    }
    command.env("PATH", env::join_paths(paths).expect("build fixture PATH"));
}
