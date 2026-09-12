use clap::Subcommand as ClapSubcommand;
use clap::{Args, Parser, ValueEnum};

#[derive(Parser)]
#[command(version)]
#[command(name = "rn")]
#[command(about = "CLI notes manager")]
#[command(disable_help_subcommand = true)]
#[command(flatten_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(ClapSubcommand, Debug)]
pub enum Subcommand {
    #[command(about = "Create a new note")]
    New(NewArgs),

    #[command(about = "Open a note")]
    Open(OpenArgs),

    #[command(about = "Delete a note")]
    #[clap(visible_alias = "rm")]
    Remove(RemoveArgs),

    #[command(about = "List existing notes")]
    #[clap(visible_alias = "ls")]
    List,

    #[command(about = "Rename a note")]
    Rename(RenameArgs),

    #[command(about = "Access config via cli")]
    Config(ConfigArgs),

    #[command(about = "Completion script for specific shell")]
    Completions(CompletionArgs),

    #[command(about = "View and manage archive")]
    Archive(ArchiveArgs),
}

#[derive(Args, Debug)]
pub struct NewArgs {
    #[arg(help = "Name of the note to be created")]
    #[arg(value_parser=non_empty_trimmed)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct OpenArgs {
    #[arg(help = "Name of the note to open")]
    pub name: String,

    #[arg(help = "Editor command used to open the note")]
    #[arg(short, long)]
    pub editor: Option<String>,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    #[arg(help = "Name of the note to be deleted")]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct RenameArgs {
    #[arg(help = "Name of the note to rename")]
    pub name: String,

    #[arg(help = "New name of the note")]
    pub new_name: String,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(ClapSubcommand, Debug)]
pub enum ConfigSubcommand {
    #[command(about = "Generate a default config file")]
    Generate(ConfigGenerateArgs),

    #[command(about = "Get specific config values")]
    Get(ConfigGetArgs),

    #[command(about = "List all config values")]
    #[clap(visible_alias = "ls")]
    List,
}

#[derive(Args, Debug)]
pub struct ConfigGenerateArgs {
    #[arg(help = "Overwrite the config file if one already exists")]
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ConfigGetArgs {
    #[arg(help = "Values to get from the config")]
    #[arg(value_name = "VALUE_NAME")]
    #[arg(required = true)]
    pub value_names: Vec<String>,
}

#[derive(Args, Debug)]
pub struct CompletionArgs {
    #[arg(help = "Shell for which to return the completion script")]
    #[arg(short, long)]
    pub shell: Shell,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum Shell {
    Zsh,
}

#[derive(Args, Debug)]
pub struct ArchiveArgs {
    #[command(subcommand)]
    pub subcommand: ArchiveSubcommand,
}

#[derive(ClapSubcommand, Debug)]
pub enum ArchiveSubcommand {
    #[command(about = "Archive a specific note")]
    Save(ArchiveSaveArgs),

    #[command(about = "List all archived notes")]
    #[clap(visible_alias = "ls")]
    List,

    #[command(about = "Open a archived note")]
    Open(ArchiveOpenArgs),

    #[command(about = "Restore a note from the archive")]
    Restore(ArchiveRestoreArgs),

    #[command(about = "Delete a archived note permanently")]
    #[clap(visible_alias = "rm")]
    Remove(ArchiveRemoveArgs),
}

#[derive(Args, Debug)]
pub struct ArchiveSaveArgs {
    #[arg(help = "Name of the note to archive")]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ArchiveOpenArgs {
    #[arg(help = "Name of the note to open")]
    pub name: String,

    #[arg(help = "Editor command used to open the note")]
    #[arg(short, long)]
    pub editor: Option<String>,
}

#[derive(Args, Debug)]
pub struct ArchiveRestoreArgs {
    #[arg(help = "Name of the note to restore from archive")]
    pub archive_name: String,

    #[arg(help = "New name of the note after its restored")]
    #[arg(short, long)]
    pub new_name: Option<String>,
}

#[derive(Args, Debug)]
pub struct ArchiveRemoveArgs {
    #[arg(help = "Name of the note to delete from archive")]
    pub name: String,
}

fn non_empty_trimmed(s: &str) -> Result<String, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        Err("Name must not be empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
#[allow(clippy::panic)] // tests
#[allow(clippy::unwrap_used)] // tests
mod tests {
    use core::panic;
    use super::*;

    macro_rules! unwrap_variant {
        ($val:expr, $variant:path) => {
            match $val {
                $variant(inner) => inner,
                other => panic!(
                    "expected variant {}, but found: {:?}",
                    stringify!($variant),
                    other
            ),
            }
        };
    }

    #[test]
    fn test_rn_no_subcommand() {
        assert!(Cli::try_parse_from(["rn"]).is_err());
    }

    #[test]
    fn test_rn_invalid_subcommand() {
        assert!(Cli::try_parse_from(["rn", " "]).is_err());
        assert!(Cli::try_parse_from(["rn", "test"]).is_err());
    }
    #[test]
    fn test_rn_invalid_arguments() {
        assert!(Cli::try_parse_from(["rn", "-t"]).is_err());
        assert!(Cli::try_parse_from(["rn", "--test"]).is_err());
    }

    #[test]
    fn test_new_no_name() {
        assert!(Cli::try_parse_from(["rn", "new"]).is_err());
        assert!(Cli::try_parse_from(["rn", "new", ""]).is_err());
        assert!(Cli::try_parse_from(["rn", "new", " "]).is_err());
        assert!(Cli::try_parse_from(["rn", "new", "\r"]).is_err());
        assert!(Cli::try_parse_from(["rn", "new", "\n"]).is_err());
        assert!(Cli::try_parse_from(["rn", "new", "\t"]).is_err());
    }

    #[test]
    fn test_new_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "new", "a", "b"]).is_err());
    }

    #[test]
    fn test_new() {
        let cli = Cli::parse_from(["rn", "new", "my_note"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::New);
        assert_eq!(args.name, "my_note");
    }

    #[test]
    fn test_open_no_name() {
        assert!(Cli::try_parse_from(["rn", "open"]).is_err());
    }

    #[test]
    fn test_open_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "open", "a", "b"]).is_err());
    }

    #[test]
    fn test_open() {
        let cli = Cli::parse_from(["rn", "open", "my_note"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::Open);
        assert_eq!(args.name, "my_note");
    }

    #[test]
    fn test_open_with_editor_short() {
        let cli = Cli::parse_from(["rn", "open", "my_note", "-e", "nvim"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::Open);
        assert_eq!(args.name, "my_note");
        assert_eq!(args.editor.unwrap(), "nvim");
    }

    #[test]
    fn test_open_with_editor_long() {
        let cli = Cli::parse_from(["rn", "open", "my_note", "--editor", "nvim"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::Open);
        assert_eq!(args.name, "my_note");
        assert_eq!(args.editor.unwrap(), "nvim");
    }

    #[test]
    fn test_remove_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "remove", "a", "b"]).is_err());
    }

    #[test]
    fn test_remove() {
        let cli = Cli::parse_from(["rn", "remove", "my_note"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::Remove);
        assert_eq!(&args.name, "my_note");
    }

    #[test]
    fn test_remove_alias() {
        let cli = Cli::parse_from(["rn", "rm", "my_note"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::Remove);
        assert_eq!(&args.name, "my_note");
    }

    #[test]
    fn test_list_additional_argument() {
        assert!(Cli::try_parse_from(["rn", "list", "my_note"]).is_err());
    }

    #[test]
    fn test_list() {
        let cli = Cli::parse_from(["rn", "list"]);
        assert!(matches!(cli.subcommand, Subcommand::List));
    }

    #[test]
    fn test_list_alias() {
        let cli = Cli::parse_from(["rn", "ls"]);
        assert!(matches!(cli.subcommand, Subcommand::List));
    }

    #[test]
    fn test_completions_no_shell() {
        assert!(Cli::try_parse_from(["rn", "completions"]).is_err());
    }

    #[test]
    fn test_completions_no_argument_name() {
        assert!(Cli::try_parse_from(["rn", "completions", "zsh"]).is_err());
    }

    #[test]
    fn test_completions_with_shell_short() {
        let cli = Cli::parse_from(["rn", "completions", "-s", "zsh"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::Completions);
        assert_eq!(args.shell, Shell::Zsh);
    }

    #[test]
    fn test_completions_with_shell_long() {
        let cli = Cli::parse_from(["rn", "completions", "--shell", "zsh"]);
        let args = unwrap_variant!(cli.subcommand, Subcommand::Completions);
        assert_eq!(args.shell, Shell::Zsh);
    }

    #[test]
    fn test_config_wrong_subcommand() {
        assert!(Cli::try_parse_from(["rn", "config"]).is_err());
        assert!(Cli::try_parse_from(["rn", "config", "test"]).is_err());
    }

    #[test]
    fn test_config_generate_invalid_argument() {
        assert!(Cli::try_parse_from(["rn", "config", "generate", "--test"]).is_err());
    }

    #[test]
    fn test_config_generate() {
        let cli = Cli::parse_from(["rn", "config", "generate"]);
        let config_args = unwrap_variant!(cli.subcommand, Subcommand::Config);
        let generate_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Generate);
        assert!(!generate_args.force);
    }

    #[test]
    fn test_config_generate_force_short() {
        let cli = Cli::parse_from(["rn", "config", "generate", "-f"]);
        let config_args = unwrap_variant!(cli.subcommand, Subcommand::Config);
        let generate_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Generate);
        assert!(generate_args.force);
    }

    #[test]
    fn test_config_generate_force_long() {
        let cli = Cli::parse_from(["rn", "config", "generate", "--force"]);
        let config_args = unwrap_variant!(cli.subcommand, Subcommand::Config);
        let generate_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Generate);
        assert!(generate_args.force);
    }

    #[test]
    fn test_config_get_no_value() {
        assert!(Cli::try_parse_from(["rn", "config", "get"]).is_err());
    }

    #[test]
    fn test_config_get_one_value() {
        let cli = Cli::parse_from(["rn", "config", "get", "value_name"]);
        let config_args = unwrap_variant!(cli.subcommand, Subcommand::Config);
        let get_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Get);
        assert_eq!(get_args.value_names, ["value_name"]);

    }

    #[test]
    fn test_config_get_multiple_values() {
        let cli = Cli::parse_from(["rn", "config", "get", "value_name_1", "value_name_2"]);
        let config_args = unwrap_variant!(cli.subcommand, Subcommand::Config);
        let get_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Get);
        assert_eq!(get_args.value_names, ["value_name_1", "value_name_2"]);
    }

    #[test]
    fn test_config_list_additional_argument() {
        assert!(Cli::try_parse_from(["rn", "config", "list", "test"]).is_err());
    }

    #[test]
    fn test_config_list() {
        let cli = Cli::parse_from(["rn", "config", "list"]);
        let config_args = unwrap_variant!(cli.subcommand, Subcommand::Config);
        assert!(matches!(config_args.subcommand, ConfigSubcommand::List));
    }

    #[test]
    fn test_rename_no_arguments() {
        assert!(Cli::try_parse_from(["rn", "rename"]).is_err());
    }

    #[test]
    fn test_rename_only_missing_argument() {
        assert!(Cli::try_parse_from(["rn", "rename", "note_1"]).is_err());
    }

    #[test]
    fn test_rename_to_many_arguments() {
        assert!(Cli::try_parse_from(["rn", "rename", "note_1", "note_2", "note_3"]).is_err());
    }

    #[test]
    fn test_rename() {
        let cli = Cli::parse_from(["rn", "rename", "note_1", "note_2"]);
        let rename_args = unwrap_variant!(cli.subcommand, Subcommand::Rename);
        assert_eq!(rename_args.name, "note_1");
        assert_eq!(rename_args.new_name, "note_2");
    }

    #[test]
    fn test_archive_no_subcommand() {
        assert!(Cli::try_parse_from(["rn", "archive"]).is_err());
    }

    #[test]
    fn test_archive_wrong_subcommand() {
        assert!(Cli::try_parse_from(["rn", "archive", "test"]).is_err());
    }

    #[test]
    fn test_archive_save_no_name() {
        assert!(Cli::try_parse_from(["rn", "archive", "save"]).is_err());
    }

    #[test]
    fn test_archive_save_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "archive", "save", "note_1", "note_2"]).is_err());
    }

    #[test]
    fn test_archive_save() {
        let cli = Cli::parse_from(["rn", "archive", "save", "note_1"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let save_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Save);
        assert_eq!(save_args.name, "note_1");
    }

    #[test]
    fn test_archive_list_additional_argument() {
        assert!(Cli::try_parse_from(["rn", "archive", "list", "test"]).is_err());
    }

    #[test]
    fn test_archive_list() {
        let cli = Cli::parse_from(["rn", "archive", "list"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        assert!(matches!(archive_args.subcommand, ArchiveSubcommand::List));
    }

    #[test]
    fn test_archive_open_no_name() {
        assert!(Cli::try_parse_from(["rn", "archive", "open"]).is_err());
    }

    #[test]
    fn test_archive_open_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "archive", "open", "note_1", "note_2"]).is_err());
    }

    #[test]
    fn test_archive_open() {
        let cli = Cli::parse_from(["rn", "archive", "open", "note_1"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let open_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Open);
        assert_eq!(open_args.name, "note_1");

    }

    #[test]
    fn test_archive_open_with_editor_short() {
        let cli = Cli::parse_from(["rn", "archive", "open", "-e", "nvim", "note_1"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let open_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Open);
        assert_eq!(open_args.editor.unwrap(), "nvim");
        assert_eq!(open_args.name, "note_1");

        let cli = Cli::parse_from(["rn", "archive", "open", "note_1", "-e", "nvim"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let open_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Open);
        assert_eq!(open_args.editor.unwrap(), "nvim");
        assert_eq!(open_args.name, "note_1");
    }

    #[test]
    fn test_archive_open_with_editor_long() {
        let cli = Cli::parse_from(["rn", "archive", "open", "--editor", "nvim", "note_1"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let open_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Open);
        assert_eq!(open_args.editor.unwrap(), "nvim");
        assert_eq!(open_args.name, "note_1");
    }


    #[test]
    fn test_archive_restore_no_name() {
        assert!(Cli::try_parse_from(["rn", "archive", "restore"]).is_err());
    }

    #[test]
    fn test_archive_restore_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "archive", "restore", "note_1", "note_2"]).is_err());
    }

    #[test]
    fn test_archive_restore() {
        let cli = Cli::parse_from(["rn", "archive", "restore", "note_1"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let restore_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Restore);
        assert_eq!(restore_args.archive_name, "note_1");
        assert_eq!(restore_args.new_name, None);
    }

    #[test]
    fn test_archive_restore_with_new_name() {
        let cli = Cli::parse_from(["rn", "archive", "restore", "note_1", "--new-name", "note"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let restore_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Restore);
        assert_eq!(restore_args.archive_name, "note_1");
        assert_eq!(restore_args.new_name.unwrap(), "note");
    }

    #[test]
    fn test_archive_remove_no_name() {
        assert!(Cli::try_parse_from(["rn", "archive", "remove"]).is_err());
    }

    #[test]
    fn test_archive_remove_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "archive", "remove", "note_1", "note_2"]).is_err());
    }

    #[test]
    fn test_archive_remove() {
        let cli = Cli::parse_from(["rn", "archive", "remove", "note_1"]);
        let archive_args = unwrap_variant!(cli.subcommand, Subcommand::Archive);
        let remove_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Remove);
        assert_eq!(remove_args.name, "note_1");
    }
}
