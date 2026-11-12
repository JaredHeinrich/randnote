pub mod error;

use std::path::PathBuf;

use chrono::Local;

use crate::config::{get_values_from_file, Config};
use crate::core::error::Error;
use crate::core::error::Result;
use crate::message::Message;
use crate::{cli, config};

use crate::system::fs;
use crate::system::shell_command;

const NOTEBOOK_DIR_NAME: &str = "notebook";
const ARCHIVE_DIR_NAME: &str = "archive";

/// Different states a note can be in.
#[derive(Clone, Copy)]
enum NoteState {
    /// `Active` notes are in the notebook and can be directly accessed.
    Active,
    /// `Archived` notes are in the archive and can only be accessed with the `archive` subcommand.
    Archived,
}

/// Get path to the notebook directory, where active notes are stored.
fn notebook_path(config: &Config) -> PathBuf {
    let mut notebook_dir = config.root_dir.clone();
    notebook_dir.push(NOTEBOOK_DIR_NAME);
    notebook_dir
}

/// Get path to the archive directory, where archived notes are stored.
fn archive_path(config: &Config) -> PathBuf {
    let mut archive_dir = config.root_dir.clone();
    archive_dir.push(ARCHIVE_DIR_NAME);
    archive_dir
}

/// Get the path for the directory for notes of specific `state`.
fn dir_path(config: &Config, state: NoteState) -> PathBuf {
    match state {
        NoteState::Active => notebook_path(config),
        NoteState::Archived => archive_path(config),
    }
}

/// Get the path of a note with specific `name` and `state`.
fn note_path(config: &Config, name: &str, state: NoteState) -> PathBuf {
    let mut path = dir_path(config, state);
    path.push(name);
    path
}

/// Get the path of an active note with specific `name`.
fn active_note_path(config: &Config, name: &str) -> PathBuf {
    let mut path = notebook_path(config);
    path.push(name);
    path
}

/// Get the path of an archived note with specific `name`.
fn archived_note_path(config: &Config, name: &str) -> PathBuf {
    let mut path = archive_path(config);
    path.push(name);
    path
}

/// Ensures that expected directories are present.
///
/// * `root_dir` directory used for all application data except config.
/// * `notebook` directory inside root to store active notes.
/// * `archive` directory inside root to store archived notes.
///
/// # Errors
///
/// Forwards errors that may occur when checking or creating directories.
fn check_dir_structure(config: &Config) -> Result<()> {
    if !fs::exists(&config.root_dir)? {
        fs::create_dir(&config.root_dir)?;
    }
    let notebook_path = notebook_path(config);
    if !fs::exists(&notebook_path)? {
        fs::create_dir(&notebook_path)?;
    }
    let archive_path = archive_path(config);
    if !fs::exists(&archive_path)? {
        fs::create_dir(&archive_path)?;
    }
    Ok(())
}

/// Open a note with specific `name` and `state` using editor from `config`.
///
/// # Errors
///
/// * Note with given `name` and `state` does not exist.
/// * [`shell_command::open_file`] fails.
fn open_note(config: &Config, name: String, state: NoteState) -> Result<()> {
    let path = note_path(config, name.as_str(), state);
    if !fs::exists(&path)? {
        return Err(Error::NotFound(name));
    }
    shell_command::open_file(&config.editor, &path)?;
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn handle_new(config: &Config, args: cli::NewArgs) -> Result<Message> {
    let name = args.name;
    let path = active_note_path(config, &name);
    if fs::exists(&path)? {
        return Err(Error::AlreadyExists(name));
    }
    fs::create_file(&path)?;
    Ok(Message::CreatedNote)
}

#[allow(clippy::needless_pass_by_value)]
fn handle_remove(config: &Config, args: cli::RemoveArgs) -> Result<Message> {
    let name = args.name;
    let path = active_note_path(config, &name);
    if !fs::exists(&path)? {
        return Err(Error::NotFound(name));
    }
    fs::delete_file(&path)?;
    Ok(Message::DeletedNote)
}

fn handle_list(config: &Config) -> Result<Message> {
    let notes = fs::list_files(&notebook_path(config))?;
    Ok(Message::Notebook(notes))
}

fn handle_open(config: &Config, args: cli::OpenArgs) -> Result<Message> {
    let mut config = config.clone();
    if let Some(editor) = args.editor {
        config.editor = editor;
    }
    let path = active_note_path(&config, &args.name);
    if args.new && !fs::exists(&path)? {
        fs::create_file(&path)?;
    }
    open_note(&config, args.name, NoteState::Active)?;
    Ok(Message::Empty)
}

#[allow(clippy::needless_pass_by_value)]
fn handle_completions(args: cli::CompletionArgs) -> Message {
    let script = match args.shell {
        cli::Shell::Zsh => include_str!("../../completions/_rn").to_owned(),
    };
    Message::CompletionScript(script)
}

fn handle_config(args: cli::ConfigArgs) -> Result<Message> {
    match args.subcommand {
        cli::ConfigSubcommand::Generate(args) => handle_config_generate(args),
        cli::ConfigSubcommand::Get(args) => handle_config_get(args),
        cli::ConfigSubcommand::List => handle_config_list(),
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_config_generate(args: cli::ConfigGenerateArgs) -> Result<Message> {
    let config_file_path = config::config_file()?;
    let config_exists = fs::exists(&config_file_path)?;
    if config_exists && !args.force {
        return Err(Error::ConfigAlreadyExists(config_file_path));
    }
    let config = Config::default()?;
    let config_string = config.to_toml()?;
    #[allow(clippy::unwrap_used)] // `config_file_path` is never empty
    let dir_path = config_file_path.parent().unwrap();
    fs::create_dir(dir_path)?;
    fs::write_file(&config_file_path, &config_string)?;
    Ok(Message::GeneratedConfig(config_file_path))
}

#[allow(clippy::needless_pass_by_value)]
fn handle_config_get(args: cli::ConfigGetArgs) -> Result<Message> {
    Ok(Message::ConfigValues(get_values_from_file(
        &args.value_names,
    )?))
}

fn handle_config_list() -> Result<Message> {
    Ok(Message::ConfigValues(get_values_from_file(
        &config::value_names::ALL,
    )?))
}

fn handle_rename(config: &Config, args: cli::RenameArgs) -> Result<Message> {
    let name = args.name;
    let path = active_note_path(config, &name);
    let new_name = args.new_name;
    let new_path = active_note_path(config, &new_name);
    if !fs::exists(&path)? {
        return Err(Error::NotFound(name));
    }
    if !args.force && fs::exists(&new_path)? {
        return Err(Error::RenameAlreadyExists(new_name));
    }
    fs::move_file(&path, &new_path)?;
    Ok(Message::Renamed((name, new_name)))
}

fn handle_archive_save(config: &Config, args: cli::ArchiveSaveArgs) -> Result<Message> {
    let name = args.name;
    let active_path = active_note_path(config, name.as_str());
    if !fs::exists(&active_path)? {
        return Err(Error::NotFound(name));
    }
    let time_stamp = Local::now().format("%d-%m-%Y-%H:%M:%S").to_string();
    let archived_name = format!("{name}_{time_stamp}");
    let archived_path = archived_note_path(config, &archived_name);
    if fs::exists(&archived_path)? {
        return Err(Error::ArchiveAlreadyExists(archived_name));
    }
    fs::move_file(&active_path, &archived_path)?;
    Ok(Message::ArchivedNote((name, archived_name)))
}

fn handle_archive_list(config: &Config) -> Result<Message> {
    let archived_notes = fs::list_files(&archive_path(config))?;
    Ok(Message::Archive(archived_notes))
}

fn handle_archive_open(config: &Config, args: cli::ArchiveOpenArgs) -> Result<Message> {
    let mut config = config.clone();
    if let Some(editor) = args.editor {
        config.editor = editor;
    }
    open_note(&config, args.name, NoteState::Archived)?;
    Ok(Message::Empty)
}

fn handle_archive_restore(config: &Config, args: cli::ArchiveRestoreArgs) -> Result<Message> {
    let new_name = args.new_name.unwrap_or_else(|| {
        match args.archive_name.rsplit_once('_') {
            Some((name, _time_stamp)) => name,
            None => args.archive_name.as_str(),
        }
        .to_owned()
    });
    let path = active_note_path(config, new_name.as_str());
    let name_taken = fs::exists(&path)?;
    if name_taken && !args.force {
        return Err(Error::RestoreAlreadyExists(new_name));
    }
    let archived_path = archived_note_path(config, args.archive_name.as_str());
    fs::copy_file(&archived_path, &path)?;
    if name_taken {
        Ok(Message::RestoredAndReplacedNote((
            args.archive_name,
            new_name,
        )))
    } else {
        Ok(Message::RestoredNote((args.archive_name, new_name)))
    }
}

fn handle_archive_remove(config: &Config, args: cli::ArchiveRemoveArgs) -> Result<Message> {
    let name = args.name;
    let path = active_note_path(config, name.as_str());
    if !fs::exists(&path)? {
        return Err(Error::NotFound(name));
    }
    fs::delete_file(&path)?;
    Ok(Message::DeletedNote)
}

fn handle_archive(config: &Config, args: cli::ArchiveArgs) -> Result<Message> {
    match args.subcommand {
        cli::ArchiveSubcommand::Save(args) => handle_archive_save(config, args),
        cli::ArchiveSubcommand::List => handle_archive_list(config),
        cli::ArchiveSubcommand::Open(args) => handle_archive_open(config, args),
        cli::ArchiveSubcommand::Restore(args) => handle_archive_restore(config, args),
        cli::ArchiveSubcommand::Remove(args) => handle_archive_remove(config, args),
    }
}

/// Handle a specific command received from the command line.
///
/// # Arguments
///
/// * `config` - config used to execute the command.
/// * `command` - cli command to execute
///
/// # Errors
///
/// Forwards errors from handlers for specific subcommands.
pub fn handle_command(config: &Config, command: cli::Cli) -> Result<Message> {
    check_dir_structure(config)?;
    match command.subcommand {
        cli::Command::New(args) => handle_new(config, args),
        cli::Command::Open(args) => handle_open(config, args),
        cli::Command::Remove(args) => handle_remove(config, args),
        cli::Command::List => handle_list(config),
        cli::Command::Rename(args) => handle_rename(config, args),
        cli::Command::Completions(args) => Ok(handle_completions(args)),
        cli::Command::Config(args) => handle_config(args),
        cli::Command::Archive(args) => handle_archive(config, args),
    }
}
