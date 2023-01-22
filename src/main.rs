use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];
    let root = "./.sandbox";
    let new_command = isolate(root, &command);
    let output = Command::new(new_command)
        .args(command_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    std::process::exit(output.status.code().unwrap())
}

fn isolate(root: &str, command: &str) -> String {
    let mut command_path = PathBuf::from(command);
    let filename = path.file_name().unwrap().to_str().unwrap();
    command_path.pop();

    // mkdir virtual root
    mkdir_p(PathBuf::from(root));

    // prepare dev/null
    mkdir_p(PathBuf::from(format!("{}/dev", root)));
    fs::File::create(format!("{}/dev/null", root)).expect("creation of null file failed");

    // copy command
    mkdir_p(PathBuf::from(format!(
        "{}/{}",
        root,
        command_path.display()
    )));
    let new_command = format!("{}/{}/{}", root, command_path.display(), file_name);
    fs::copy(command, new_command).expect("could not copy command");

    // chroot virtual root
    chroot(root).expect("not possible to use chroot");
    new_command
}

fn mkdir_p(path: PathBuf) {
    let display = path.display();
    if !Path::new(&format!("{}", display)).exists() {
        fs::create_dir(display).expect(&format!("not possible to create dir, {}", display));
    }
}
