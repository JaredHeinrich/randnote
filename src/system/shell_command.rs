use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use crate::system::error::Error;
use crate::system::error::Result;
use crate::system::fs::is_file;

fn check_installed(command_name: &str) -> Result<()> {
    let check_command = "which";
    let status = std::process::Command::new(check_command)
        .arg(command_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| Error::CommandNotInstalled(check_command.to_owned()))?
        .code()
        .unwrap_or(1);
    if status != 0 {
        return Err(Error::CommandNotInstalled(command_name.to_owned()));
    }
    Ok(())
}

pub fn open_file(editor_command: &str, path: &Path) -> Result<()> {
    check_installed(editor_command)?;
    if !is_file(path) {
        return Err(Error::NotAFile(path.to_path_buf()));
    }
    let mut command = Command::new(editor_command);
    if editor_command == "nvim" {
        let parent_path = path.parent().ok_or(Error::NoParentDirectory)?;
        let parent_path = parent_path
            .to_str()
            .ok_or(Error::PathNoUTF8(parent_path.to_path_buf()))?;
        command.arg("-c").arg(format!("cd {parent_path}"));
    }
    command
        .arg(path.as_os_str())
        .status()
        .map(|_| ())
        .map_err(Into::into)
}
