use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut args = capture_file("TEST_ARGS_FILE")?;
    for arg in env::args_os().skip(1) {
        args.write_all(arg.as_encoded_bytes())?;
        args.write_all(b"\n")?;
    }

    let mut lesscharset = capture_file("TEST_LESSCHARSET_FILE")?;
    if let Ok(value) = env::var("LESSCHARSET") {
        lesscharset.write_all(value.as_bytes())?;
        lesscharset.write_all(b"\n")?;
    }

    io::copy(
        &mut io::stdin().lock(),
        &mut capture_file("TEST_OUTPUT_FILE")?,
    )?;
    Ok(())
}

fn capture_file(variable: &str) -> io::Result<File> {
    let path = env::var_os(variable).unwrap_or_else(|| panic!("{variable} is unset"));
    File::create(path)
}
