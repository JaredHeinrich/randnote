use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use crate::system::error::Error;
use crate::system::error::Result;
use crate::system::fs::is_file;

/// Check if a given command is installed on the system.
///
/// Uses `which` to check if the given command can be executed.
/// Returns `Ok(true)` if command is installed, `Ok(false)` if it is not and `Err(_)` if
/// installation can neither be confirmed or denied.
///
/// # Arguments
///
/// * `command_name` - command for which to check installation
///
/// # Errors
///
/// * `which` could not be executed, installation of given command could not be verified
fn check_installed(command_name: &str) -> Result<bool> {
    // TODO is which really the best way to check if a command is installed?
    let check_command = "which";
    let status = std::process::Command::new(check_command)
        .arg(command_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|_| Error::CommandNotInstalled(check_command.to_owned()))?
        .code()
        .unwrap_or(1);
    Ok(status == 0)
}

/// Open file at `path` using given editor command.
///
/// # Arguments
///
/// * `editor_command` - command used to open file
/// * `path` - path file to open
///
/// # Errors
///
/// * `editor_command` is not installed
/// * `path` does not point to a file
/// * `path` does not have a parent directory
/// * `path` is no valid UTF-8
/// * execution of the command fails for other reason
pub fn open_file(editor_command: &str, path: &Path) -> Result<()> {
    if !check_installed(editor_command)? {
        return Err(Error::CommandNotInstalled(editor_command.to_owned()));
    }
    if !is_file(path) {
        return Err(Error::NotAFile(path.to_path_buf()));
    }
    let mut command = Command::new(editor_command);
    // TODO try to replace with command.current_dir()
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
