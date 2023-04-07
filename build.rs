use std::{fs::canonicalize, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir(canonicalize(std::path::Path::new("client")).unwrap())
        .spawn()
        .expect("ls command failed to start");

    tonic_build::configure().build_server(false).compile(
        &["proto/helloworld/helloworld.proto"],
        &["proto/helloworld"],
    )?;
    Ok(())
}
