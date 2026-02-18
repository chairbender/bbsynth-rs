use std::env;
use std::process::Command;

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("bundle") => bundle()?,
        _ => print_help(),
    }
    Ok(())
}

fn bundle() -> Result<(), DynError> {
    let status = Command::new("cargo")
        .args([
            "run",
            "-p",
            "cargo-coupler",
            "--",
            "coupler",
            "bundle",
            "-p",
            "bbsynth-vst3",
        ])
        .status()?;

    if !status.success() {
        return Err("bundle failed".into());
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
    bundle    Bundle the vst3 plugin
"
    )
}
