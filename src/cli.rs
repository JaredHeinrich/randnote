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

#[derive(ClapSubcommand)]
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

    #[command(about = "Access config via cli")]
    Config(ConfigArgs),

    #[command(about = "Completion script for specific shell")]
    Completions(CompletionArgs),

    #[command(about = "View and manage archive")]
    Archive(ArchiveArgs),
}

#[derive(Args)]
pub struct NewArgs {
    #[arg(help = "Name of the note to be created")]
    #[arg(value_parser=non_empty_trimmed)]
    pub name: String,
}

#[derive(Args)]
pub struct OpenArgs {
    #[arg(help = "Name of the note to open")]
    pub name: String,

    #[arg(help = "Editor command used to open the note")]
    #[arg(short, long)]
    pub editor: Option<String>,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[arg(help = "Name of the note to be deleted")]
    pub name: String,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(ClapSubcommand)]
pub enum ConfigSubcommand {
    #[command(about = "Generate a default config file")]
    Generate(ConfigGenerateArgs),

    #[command(about = "Get specific config values")]
    Get(ConfigGetArgs),

    #[command(about = "List all config values")]
    #[clap(visible_alias = "ls")]
    List,
}

#[derive(Args)]
pub struct ConfigGenerateArgs {
    #[arg(help = "Overwrite the config file if one already exists")]
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args)]
pub struct ConfigGetArgs {
    #[arg(help = "Values to get from the config")]
    #[arg(value_name = "VALUE_NAME")]
    #[arg(required = true)]
    pub value_names: Vec<String>,
}

#[derive(Args)]
pub struct CompletionArgs {
    #[arg(help = "Shell for which to return the completion script")]
    #[arg(short, long)]
    pub shell: Shell,
}

#[derive(ValueEnum, Clone, PartialEq, Debug)]
pub enum Shell {
    Zsh,
}

#[derive(Args)]
pub struct ArchiveArgs {
    #[command(subcommand)]
    pub subcommand: ArchiveSubcommand,
}

#[derive(ClapSubcommand)]
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

#[derive(Args)]
pub struct ArchiveSaveArgs {
    #[arg(help = "Name of the note to archive")]
    pub name: String,
}

#[derive(Args)]
pub struct ArchiveOpenArgs {
    #[arg(help = "Name of the note to open")]
    pub name: String,

    #[arg(help = "Editor command used to open the note")]
    #[arg(short, long)]
    pub editor: Option<String>,
}

#[derive(Args)]
pub struct ArchiveRestoreArgs {
    #[arg(help = "Name of the note to restore from archive")]
    pub archive_name: String,

    #[arg(help = "New name of the note after its restored")]
    #[arg(short, long)]
    pub new_name: Option<String>,
}

#[derive(Args)]
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

    #[test]
    fn test_rn_no_subcommand() {
        assert!(Cli::try_parse_from(["rn"]).is_err());
    }

    #[test]
    fn test_rn_invalid_subcommands() {
        assert!(Cli::try_parse_from(["rn", " "]).is_err());
        assert!(Cli::try_parse_from(["rn", "test"]).is_err());
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
        let Subcommand::New(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.name, "my_note");
    }

    #[test]
    fn test_open_multiple_names() {
        assert!(Cli::try_parse_from(["rn", "open", "a", "b"]).is_err());
    }

    #[test]
    fn test_open() {
        let cli = Cli::parse_from(["rn", "open", "my_note"]);
        let Subcommand::Open(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.name, "my_note");
    }

    #[test]
    fn test_open_with_editor() {
        let cli = Cli::parse_from(["rn", "open", "my_note", "-e", "nvim"]);
        let Subcommand::Open(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.name, "my_note");
        assert_eq!(args.editor.unwrap(), "nvim");

        let cli = Cli::parse_from(["rn", "open", "my_note", "--editor", "nvim"]);
        let Subcommand::Open(args) = cli.subcommand else {
            panic!()
        };
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
        let Subcommand::Remove(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(&args.name, "my_note");
        let cli = Cli::parse_from(["rn", "rm", "my_note"]);
        let Subcommand::Remove(args) = cli.subcommand else {
            panic!()
        };
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
    fn test_completions_no_shell() {
        assert!(Cli::try_parse_from(["rn", "completions"]).is_err());
    }

    #[test]
    fn test_completions_no_argument_name() {
        assert!(Cli::try_parse_from(["rn", "completions", "zsh"]).is_err());
    }

    #[test]
    fn test_completions() {
        let cli = Cli::parse_from(["rn", "completions", "-s", "zsh"]);
        let Subcommand::Completions(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.shell, Shell::Zsh);

        let cli = Cli::parse_from(["rn", "completions", "--shell", "zsh"]);
        let Subcommand::Completions(args) = cli.subcommand else {
            panic!()
        };
        assert_eq!(args.shell, Shell::Zsh);
    }

    #[test]
    fn test_config_wrong_subcommand() {
        assert!(Cli::try_parse_from(["rn", "config"]).is_err());
        assert!(Cli::try_parse_from(["rn", "config", "test"]).is_err());
    }

    #[test]
    fn test_config_generate() {
        assert!(Cli::try_parse_from(["rn", "config", "generate", "--test"]).is_err());

        let cli = Cli::parse_from(["rn", "config", "generate"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Generate(generate_args) = config_args.subcommand else {
            panic!()
        };
        assert!(!generate_args.force);

        let cli = Cli::parse_from(["rn", "config", "generate", "--force"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Generate(generate_args) = config_args.subcommand else {
            panic!()
        };
        assert!(generate_args.force);

        let cli = Cli::parse_from(["rn", "config", "generate", "-f"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Generate(generate_args) = config_args.subcommand else {
            panic!()
        };
        assert!(generate_args.force);
    }

    #[test]
    fn test_config_get() {
        assert!(Cli::try_parse_from(["rn", "config", "get"]).is_err());

        let cli = Cli::parse_from(["rn", "config", "get", "value_name"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Get(get_args) = config_args.subcommand else {
            panic!()
        };
        assert_eq!(get_args.value_names, ["value_name"]);

        let cli = Cli::parse_from(["rn", "config", "get", "value_name_1", "value_name_2"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        let ConfigSubcommand::Get(get_args) = config_args.subcommand else {
            panic!()
        };
        assert_eq!(get_args.value_names, ["value_name_1", "value_name_2"]);
    }

    #[test]
    fn test_config_list() {
        assert!(Cli::try_parse_from(["rn", "config", "list", "test"]).is_err());

        let cli = Cli::parse_from(["rn", "config", "list"]);
        let Subcommand::Config(config_args) = cli.subcommand else {
            panic!()
        };
        assert!(matches!(config_args.subcommand, ConfigSubcommand::List));
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
    fn test_archive_save() {
        assert!(Cli::try_parse_from(["rn", "archive", "save"]).is_err());

        assert!(Cli::try_parse_from(["rn", "archive", "save", "nb_1", "nb_2"]).is_err());

        let cli = Cli::parse_from(["rn", "archive", "save", "nb_1"]);
        let Subcommand::Archive(archive_args) = cli.subcommand else {
            panic!();
        };
        let ArchiveSubcommand::Save(save_args) = archive_args.subcommand else {
            panic!();
        };
        assert_eq!(save_args.name, "nb_1");
    }

    #[test]
    fn test_archive_list() {
        assert!(Cli::try_parse_from(["rn", "archive", "list", "test"]).is_err());

        let cli = Cli::parse_from(["rn", "archive", "list"]);
        let Subcommand::Archive(archive_args) = cli.subcommand else {
            panic!();
        };
        assert!(matches!(archive_args.subcommand, ArchiveSubcommand::List));
    }

    #[test]
    fn test_archive_open() {
        assert!(Cli::try_parse_from(["rn", "archive", "open"]).is_err());

        assert!(Cli::try_parse_from(["rn", "archive", "open", "nb_1", "nb_2"]).is_err());

        let cli = Cli::parse_from(["rn", "archive", "open", "nb_1"]);
        let Subcommand::Archive(archive_args) = cli.subcommand else {
            panic!();
        };
        let ArchiveSubcommand::Open(open_args) = archive_args.subcommand else {
            panic!();
        };
        assert_eq!(open_args.name, "nb_1");

        let cli = Cli::parse_from(["rn", "archive", "open", "-e", "nvim", "nb_1"]);
        let Subcommand::Archive(archive_args) = cli.subcommand else {
            panic!();
        };
        let ArchiveSubcommand::Open(open_args) = archive_args.subcommand else {
            panic!();
        };
        assert_eq!(open_args.editor.unwrap(), "nvim");
        assert_eq!(open_args.name, "nb_1");
    }

    #[test]
    fn test_archive_restore() {
        assert!(Cli::try_parse_from(["rn", "archive", "restore"]).is_err());

        assert!(Cli::try_parse_from(["rn", "archive", "restore", "nb_1", "nb_2"]).is_err());

        let cli = Cli::parse_from(["rn", "archive", "restore", "nb_1"]);
        let Subcommand::Archive(archive_args) = cli.subcommand else {
            panic!();
        };
        let ArchiveSubcommand::Restore(restore_args) = archive_args.subcommand else {
            panic!();
        };
        assert_eq!(restore_args.archive_name, "nb_1");
        assert_eq!(restore_args.new_name, None);

        let cli = Cli::parse_from(["rn", "archive", "restore", "nb_1", "--new-name", "nb"]);
        let Subcommand::Archive(archive_args) = cli.subcommand else {
            panic!();
        };
        let ArchiveSubcommand::Restore(restore_args) = archive_args.subcommand else {
            panic!();
        };
        assert_eq!(restore_args.archive_name, "nb_1");
        assert_eq!(restore_args.new_name, Some(String::from("nb")));
    }

    #[test]
    fn test_archive_remove() {
        assert!(Cli::try_parse_from(["rn", "archive", "remove"]).is_err());

        assert!(Cli::try_parse_from(["rn", "archive", "remove", "nb_1", "nb_2"]).is_err());

        let cli = Cli::parse_from(["rn", "archive", "remove", "nb_1"]);
        let Subcommand::Archive(archive_args) = cli.subcommand else {
            panic!();
        };
        let ArchiveSubcommand::Remove(remove_args) = archive_args.subcommand else {
            panic!();
        };
        assert_eq!(remove_args.name, "nb_1");
    }
}
