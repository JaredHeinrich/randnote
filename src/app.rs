use std::path::PathBuf;

use anyhow::{Ok, Result};
use chrono::Local;

use crate::cli;
use crate::config;
use crate::config::{Config, PartialConfig};
use crate::error::AppError;
use crate::error::SystemError;
use crate::file_operations::FileOperations;
use crate::message::Message;

const RN_ROOT_DIR: &str = ".rn";
const NOTEBOOK_DIR_NAME: &str = "notebook";
const ARCHIVE_DIR_NAME: &str = "archive";

#[derive(Clone, Copy)]
enum NoteType {
    Active,
    Archived,
}

pub struct App<FS: FileOperations> {
    pub config: config::Config,
    pub rn_root_dir: PathBuf,
    fs: FS,
}

impl<FS: FileOperations> App<FS> {
    pub fn new(config: config::Config, fs: FS) -> Result<Self> {
        let Some(mut rn_root_dir) = std::env::home_dir() else {
            return Err(SystemError::NoHomeDir.into());
        };
        rn_root_dir.push(RN_ROOT_DIR);
        Ok(Self {
            config,
            rn_root_dir,
            fs,
        })
    }

    fn notebook_dir(&self) -> PathBuf {
        let mut notebook_dir = self.rn_root_dir.clone();
        notebook_dir.push(NOTEBOOK_DIR_NAME);
        notebook_dir
    }

    fn archive_dir(&self) -> PathBuf {
        let mut archive_dir = self.rn_root_dir.clone();
        archive_dir.push(ARCHIVE_DIR_NAME);
        archive_dir
    }

    fn get_dir_path(&self, note_type: NoteType) -> PathBuf {
        match note_type {
            NoteType::Active => self.notebook_dir(),
            NoteType::Archived => self.archive_dir(),
        }
    }

    fn get_note_path(&self, name: &str, note_type: NoteType) -> PathBuf {
        let mut path = self.get_dir_path(note_type);
        path.push(name);
        path
    }

    fn check_dir_structure(&mut self) -> Result<()> {
        let rn_root_dir = &self.rn_root_dir;
        if !self.fs.exists(rn_root_dir)? {
            self.fs.create_dir(rn_root_dir)?;
        }
        let active_dir = self.get_dir_path(NoteType::Active);
        if !self.fs.exists(&active_dir)? {
            self.fs.create_dir(&active_dir)?;
        }
        let archive_dir = self.get_dir_path(NoteType::Archived);
        if !self.fs.exists(&archive_dir)? {
            self.fs.create_dir(&archive_dir)?;
        }
        Ok(())
    }

    fn open_note(&mut self, name: String, note_type: NoteType) -> Result<Message> {
        let path = self.get_note_path(name.as_str(), note_type);
        if !self.fs.exists(&path)? {
            return Err(AppError::NotFound(name).into());
        }
        self.fs.open_file(&self.config.editor, &path)?;
        Ok(Message::Empty)
    }

    fn get_config_values<T: AsRef<str>>(&self, value_names: &[T]) -> Result<Vec<(String, String)>> {
        let mut config_values: Vec<(String, String)> = Vec::new();
        let config_file_path = config::config_file()?;
        let config_exists = self.fs.exists(&config_file_path)?;
        if !config_exists {
            return Ok(config_values);
        }
        let config = PartialConfig::from_config_file(&self.fs)?;
        for value_name in value_names {
            let value_name = value_name.as_ref();
            let value = match value_name {
                config::value_names::EDITOR => config.editor.as_ref(),
                _ => continue,
            };
            if let Some(value) = value {
                config_values.push((value_name.to_owned(), value.to_owned()));
            }
        }
        Ok(config_values)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_new(&mut self, args: cli::NewArgs) -> Result<Message> {
        let name = args.name;
        let path = self.get_note_path(&name, NoteType::Active);
        if self.fs.exists(&path)? {
            return Err(AppError::AlreadyExists(name).into());
        }
        self.fs.create_file(&path)?;
        Ok(Message::CreatedNote)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_remove(&mut self, args: cli::RemoveArgs) -> Result<Message> {
        let name = args.name;
        let path = self.get_note_path(&name, NoteType::Active);
        if !self.fs.exists(&path)? {
            return Err(AppError::NotFound(name).into());
        }
        self.fs.delete_file(&path)?;
        Ok(Message::DeletedNote)
    }

    fn handle_list(&self) -> Result<Message> {
        let notes = self.fs.get_files(&self.get_dir_path(NoteType::Active))?;
        Ok(Message::Notebook(notes))
    }

    fn handle_open(&mut self, args: cli::OpenArgs) -> Result<Message> {
        if let Some(editor) = args.editor {
            self.config.editor = editor;
        }
        self.open_note(args.name, NoteType::Active)
    }

    #[allow(clippy::unused_self)]
    #[allow(clippy::needless_pass_by_value)]
    fn handle_completions(&self, args: cli::CompletionArgs) -> Result<Message> {
        let script = match args.shell {
            cli::Shell::Zsh => include_str!("../completions/_rn").to_owned(),
        };
        Ok(Message::CompletionScript(script))
    }

    fn handle_config(&mut self, args: cli::ConfigArgs) -> Result<Message> {
        match args.subcommand {
            cli::ConfigSubcommand::Generate(args) => self.handle_config_generate(args),
            cli::ConfigSubcommand::Get(args) => self.handle_config_get(args),
            cli::ConfigSubcommand::List => self.handle_config_list(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_config_generate(&mut self, args: cli::ConfigGenerateArgs) -> Result<Message> {
        let config_file_path = config::config_file()?;
        let config_exists = self.fs.exists(&config_file_path)?;
        if config_exists && !args.force {
            return Err(AppError::ConfigAlreadyExists(config_file_path).into());
        }
        let config = Config::default();
        let config_string = config.to_toml()?;
        #[allow(clippy::unwrap_used)] // `config_file_path` is never empty
        let dir_path = config_file_path.parent().unwrap();
        self.fs.create_dir(dir_path)?;
        self.fs.write_file(&config_file_path, &config_string)?;
        Ok(Message::GeneratedConfig(config_file_path))
    }

    #[allow(clippy::needless_pass_by_value)]
    fn handle_config_get(&self, args: cli::ConfigGetArgs) -> Result<Message> {
        Ok(Message::ConfigValues(
            self.get_config_values(&args.value_names)?,
        ))
    }

    fn handle_config_list(&self) -> Result<Message> {
        Ok(Message::ConfigValues(
            self.get_config_values(&config::value_names::ALL)?,
        ))
    }

    fn handle_archive(&mut self, args: cli::ArchiveArgs) -> Result<Message> {
        match args.subcommand {
            cli::ArchiveSubcommand::Save(args) => self.handle_archive_save(args),
            cli::ArchiveSubcommand::List => self.handle_archive_list(),
            cli::ArchiveSubcommand::Open(args) => self.handle_archive_open(args),
            cli::ArchiveSubcommand::Restore(args) => self.handle_archive_restore(args),
            cli::ArchiveSubcommand::Remove(args) => self.handle_archive_remove(args),
        }
    }

    fn handle_archive_save(&mut self, args: cli::ArchiveSaveArgs) -> Result<Message> {
        let name = args.name;
        let active_path = self.get_note_path(name.as_str(), NoteType::Active);
        if !self.fs.exists(&active_path)? {
            return Err(AppError::NotFound(name).into());
        }
        let time_stamp = Local::now().format("%d-%m-%Y-%H:%M:%S").to_string();
        let archived_name = format!("{name}_{time_stamp}");
        let archived_path = self.get_note_path(&archived_name, NoteType::Archived);
        if self.fs.exists(&archived_path)? {
            return Err(AppError::ArchiveAlreadyExists(archived_name).into());
        }
        self.fs.copy(&active_path, &archived_path)?;
        self.fs.delete_file(&active_path)?;
        Ok(Message::ArchivedNote((name, archived_name)))
    }

    fn handle_archive_list(&self) -> Result<Message> {
        let archived_notes = self.fs.get_files(&self.get_dir_path(NoteType::Archived))?;
        Ok(Message::Archive(archived_notes))
    }

    fn handle_archive_open(&mut self, args: cli::ArchiveOpenArgs) -> Result<Message> {
        if let Some(editor) = args.editor {
            self.config.editor = editor;
        }
        self.open_note(args.name, NoteType::Archived)
    }

    fn handle_archive_restore(&mut self, args: cli::ArchiveRestoreArgs) -> Result<Message> {
        let new_name = args.new_name.unwrap_or_else(|| {
            match args.archive_name.rsplit_once('_') {
                Some((name, _time_stamp)) => name,
                None => args.archive_name.as_str(),
            }
            .to_owned()
        });
        let path = self.get_note_path(new_name.as_str(), NoteType::Active);
        if self.fs.exists(&path)? {
            return Err(AppError::RestoreAlreadyExists(new_name).into());
        }
        let archived_path = self.get_note_path(args.archive_name.as_str(), NoteType::Archived);
        self.fs.copy(&archived_path, &path)?;
        Ok(Message::RestoredNote((args.archive_name, new_name)))
    }

    fn handle_archive_remove(&mut self, args: cli::ArchiveRemoveArgs) -> Result<Message> {
        let name = args.name;
        let path = self.get_note_path(name.as_str(), NoteType::Archived);
        if !self.fs.exists(&path)? {
            return Err(AppError::NotFound(name).into());
        }
        self.fs.delete_file(&path)?;
        Ok(Message::DeletedNote)
    }

    pub fn handle_command(&mut self, command: cli::Cli) -> Result<Message> {
        self.check_dir_structure()?;
        match command.subcommand {
            cli::Subcommand::New(args) => self.handle_new(args),
            cli::Subcommand::Open(args) => self.handle_open(args),
            cli::Subcommand::Remove(args) => self.handle_remove(args),
            cli::Subcommand::List => self.handle_list(),
            cli::Subcommand::Completions(args) => self.handle_completions(args),
            cli::Subcommand::Config(args) => self.handle_config(args),
            cli::Subcommand::Rename(_) => todo!("Not implemented yet"),
            cli::Subcommand::Archive(args) => self.handle_archive(args),
        }
    }
}
