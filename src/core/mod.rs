pub mod error;

use std::path::PathBuf;

use chrono::Local;

use crate::config::{Config, PartialConfig};
use crate::core::error::Error;
use crate::core::error::Result;
use crate::message::Message;
use crate::{cli, config};

use crate::system::fs;
use crate::system::shell_command;

const NOTEBOOK_DIR_NAME: &str = "notebook";
const ARCHIVE_DIR_NAME: &str = "archive";

#[derive(Clone, Copy)]
enum NoteType {
    Active,
    Archived,
}

fn notebook_dir(config: &Config) -> PathBuf {
    let mut notebook_dir = config.root_dir.clone();
    notebook_dir.push(NOTEBOOK_DIR_NAME);
    notebook_dir
}

fn archive_dir(config: &Config) -> PathBuf {
    let mut archive_dir = config.root_dir.clone();
    archive_dir.push(ARCHIVE_DIR_NAME);
    archive_dir
}

fn get_dir_path(config: &Config, note_type: NoteType) -> PathBuf {
    match note_type {
        NoteType::Active => notebook_dir(config),
        NoteType::Archived => archive_dir(config),
    }
}

fn get_note_path(config: &Config, name: &str, note_type: NoteType) -> PathBuf {
    let mut path = get_dir_path(config, note_type);
    path.push(name);
    path
}

fn check_dir_structure(config: &Config) -> Result<()> {
    let rn_root_dir = &config.root_dir;
    if !fs::exists(rn_root_dir)? {
        fs::create_dir(rn_root_dir)?;
    }
    let active_dir = get_dir_path(config, NoteType::Active);
    if !fs::exists(&active_dir)? {
        fs::create_dir(&active_dir)?;
    }
    let archive_dir = get_dir_path(config, NoteType::Archived);
    if !fs::exists(&archive_dir)? {
        fs::create_dir(&archive_dir)?;
    }
    Ok(())
}

fn open_note(config: &Config, name: String, note_type: NoteType) -> Result<()> {
    let path = get_note_path(config, name.as_str(), note_type);
    if !fs::exists(&path)? {
        return Err(Error::NotFound(name));
    }
    shell_command::open_file(&config.editor, &path)?;
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn handle_new(config: &Config, args: cli::NewArgs) -> Result<Message> {
    let name = args.name;
    let path = get_note_path(config, &name, NoteType::Active);
    if fs::exists(&path)? {
        return Err(Error::AlreadyExists(name));
    }
    fs::create_file(&path)?;
    Ok(Message::CreatedNote)
}

#[allow(clippy::needless_pass_by_value)]
fn handle_remove(config: &Config, args: cli::RemoveArgs) -> Result<Message> {
    let name = args.name;
    let path = get_note_path(config, &name, NoteType::Active);
    if !fs::exists(&path)? {
        return Err(Error::NotFound(name));
    }
    fs::delete_file(&path)?;
    Ok(Message::DeletedNote)
}

fn handle_list(config: &Config) -> Result<Message> {
    let notes = fs::list_files(&get_dir_path(config, NoteType::Active))?;
    Ok(Message::Notebook(notes))
}

fn handle_open(config: &Config, args: cli::OpenArgs) -> Result<Message> {
    let mut config = config.clone();
    if let Some(editor) = args.editor {
        config.editor = editor;
    }
    let path = get_note_path(&config, &args.name, NoteType::Active);
    if args.new && !fs::exists(&path)? {
        fs::create_file(&path)?;
    }
    open_note(&config, args.name, NoteType::Active)?;
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
    let config = PartialConfig::default();
    let config_string = config.to_toml()?;
    #[allow(clippy::unwrap_used)] // `config_file_path` is never empty
    let dir_path = config_file_path.parent().unwrap();
    fs::create_dir(dir_path)?;
    fs::write_file(&config_file_path, &config_string)?;
    Ok(Message::GeneratedConfig(config_file_path))
}

#[allow(clippy::needless_pass_by_value)]
fn handle_config_get(args: cli::ConfigGetArgs) -> Result<Message> {
    Ok(Message::ConfigValues(PartialConfig::get_values_from_file(
        &args.value_names,
    )?))
}

fn handle_config_list() -> Result<Message> {
    Ok(Message::ConfigValues(PartialConfig::get_values_from_file(
        &config::value_names::ALL,
    )?))
}

fn handle_rename(config: &Config, args: cli::RenameArgs) -> Result<Message> {
    let name = args.name;
    let path = get_note_path(config, &name, NoteType::Active);
    let new_name = args.new_name;
    let new_path = get_note_path(config, &new_name, NoteType::Active);
    if !fs::exists(&path)? {
        return Err(Error::NotFound(name));
    }
    if !args.force && fs::exists(&new_path)? {
        return Err(Error::RenameAlreadyExists(new_name));
    }
    fs::rename_file(&path, &new_path)?;
    Ok(Message::Renamed((name, new_name)))
}

fn handle_archive_save(config: &Config, args: cli::ArchiveSaveArgs) -> Result<Message> {
    let name = args.name;
    let active_path = get_note_path(config, name.as_str(), NoteType::Active);
    if !fs::exists(&active_path)? {
        return Err(Error::NotFound(name));
    }
    let time_stamp = Local::now().format("%d-%m-%Y-%H:%M:%S").to_string();
    let archived_name = format!("{name}_{time_stamp}");
    let archived_path = get_note_path(config, &archived_name, NoteType::Archived);
    if fs::exists(&archived_path)? {
        return Err(Error::ArchiveAlreadyExists(archived_name));
    }
    fs::rename_file(&active_path, &archived_path)?;
    Ok(Message::ArchivedNote((name, archived_name)))
}

fn handle_archive_list(config: &Config) -> Result<Message> {
    let archived_notes = fs::list_files(&get_dir_path(config, NoteType::Archived))?;
    Ok(Message::Archive(archived_notes))
}

fn handle_archive_open(config: &Config, args: cli::ArchiveOpenArgs) -> Result<Message> {
    let mut config = config.clone();
    if let Some(editor) = args.editor {
        config.editor = editor;
    }
    open_note(&config, args.name, NoteType::Archived)?;
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
    let path = get_note_path(config, new_name.as_str(), NoteType::Active);
    let name_taken = fs::exists(&path)?;
    if name_taken && !args.force {
        return Err(Error::RestoreAlreadyExists(new_name));
    }
    let archived_path = get_note_path(config, args.archive_name.as_str(), NoteType::Archived);
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
    let path = get_note_path(config, name.as_str(), NoteType::Archived);
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

pub fn handle_command(config: &Config, command: cli::Cli) -> Result<Message> {
    check_dir_structure(config)?;
    match command.command {
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
