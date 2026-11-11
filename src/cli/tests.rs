#![allow(clippy::panic)] // tests
#![allow(clippy::unwrap_used)] // tests

use super::*;
use super::custom_parser::*;
use super::error::*;
use clap::Parser;
use core::panic;

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
fn test_valid_note_name() {
    assert!(valid_note_name("test").is_ok());
    assert!(valid_note_name("test.md").is_ok());
    assert_eq!(valid_note_name(" ").unwrap_err(), InvalidNoteName::Empty);
    assert_eq!(valid_note_name("\t").unwrap_err(), InvalidNoteName::Empty);
    assert_eq!(valid_note_name("").unwrap_err(), InvalidNoteName::Empty);
    assert_eq!(
        valid_note_name("..").unwrap_err(),
        InvalidNoteName::InvalidPathComponent
    );
    assert_eq!(
        valid_note_name("../test.md").unwrap_err(),
        InvalidNoteName::MultiplePathComponents
    );
    assert_eq!(
        valid_note_name("test/").unwrap_err(),
        InvalidNoteName::TrailingSeparator
    );
    assert_eq!(
        valid_note_name(" test").unwrap_err(),
        InvalidNoteName::AdditionalWhitespaces
    );
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
    let args = unwrap_variant!(cli.subcommand, Command::New);
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
    let args = unwrap_variant!(cli.subcommand, Command::Open);
    assert_eq!(args.name, "my_note");
    assert!(!args.new);
}

#[test]
fn test_open_with_editor_short() {
    let cli = Cli::parse_from(["rn", "open", "my_note", "-e", "nvim"]);
    let args = unwrap_variant!(cli.subcommand, Command::Open);
    assert_eq!(args.name, "my_note");
    assert_eq!(args.editor.unwrap(), "nvim");
    assert!(!args.new);
}

#[test]
fn test_open_with_editor_long() {
    let cli = Cli::parse_from(["rn", "open", "my_note", "--editor", "nvim"]);
    let args = unwrap_variant!(cli.subcommand, Command::Open);
    assert_eq!(args.name, "my_note");
    assert_eq!(args.editor.unwrap(), "nvim");
    assert!(!args.new);
}

#[test]
fn test_open_with_new_short() {
    let cli = Cli::parse_from(["rn", "open", "my_note", "-n"]);
    let args = unwrap_variant!(cli.subcommand, Command::Open);
    assert_eq!(args.name, "my_note");
    assert!(args.new);
}

#[test]
fn test_open_with_new_long() {
    let cli = Cli::parse_from(["rn", "open", "my_note", "--new"]);
    let args = unwrap_variant!(cli.subcommand, Command::Open);
    assert_eq!(args.name, "my_note");
    assert!(args.new);
}

#[test]
fn test_remove_multiple_names() {
    assert!(Cli::try_parse_from(["rn", "remove", "a", "b"]).is_err());
}

#[test]
fn test_remove() {
    let cli = Cli::parse_from(["rn", "remove", "my_note"]);
    let args = unwrap_variant!(cli.subcommand, Command::Remove);
    assert_eq!(&args.name, "my_note");
}

#[test]
fn test_remove_alias() {
    let cli = Cli::parse_from(["rn", "rm", "my_note"]);
    let args = unwrap_variant!(cli.subcommand, Command::Remove);
    assert_eq!(&args.name, "my_note");
}

#[test]
fn test_list_additional_argument() {
    assert!(Cli::try_parse_from(["rn", "list", "my_note"]).is_err());
}

#[test]
fn test_list() {
    let cli = Cli::parse_from(["rn", "list"]);
    assert!(matches!(cli.subcommand, Command::List));
}

#[test]
fn test_list_alias() {
    let cli = Cli::parse_from(["rn", "ls"]);
    assert!(matches!(cli.subcommand, Command::List));
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
    let args = unwrap_variant!(cli.subcommand, Command::Completions);
    assert_eq!(args.shell, Shell::Zsh);
}

#[test]
fn test_completions_with_shell_long() {
    let cli = Cli::parse_from(["rn", "completions", "--shell", "zsh"]);
    let args = unwrap_variant!(cli.subcommand, Command::Completions);
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
    let config_args = unwrap_variant!(cli.subcommand, Command::Config);
    let generate_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Generate);
    assert!(!generate_args.force);
}

#[test]
fn test_config_generate_force_short() {
    let cli = Cli::parse_from(["rn", "config", "generate", "-f"]);
    let config_args = unwrap_variant!(cli.subcommand, Command::Config);
    let generate_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Generate);
    assert!(generate_args.force);
}

#[test]
fn test_config_generate_force_long() {
    let cli = Cli::parse_from(["rn", "config", "generate", "--force"]);
    let config_args = unwrap_variant!(cli.subcommand, Command::Config);
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
    let config_args = unwrap_variant!(cli.subcommand, Command::Config);
    let get_args = unwrap_variant!(config_args.subcommand, ConfigSubcommand::Get);
    assert_eq!(get_args.value_names, ["value_name"]);
}

#[test]
fn test_config_get_multiple_values() {
    let cli = Cli::parse_from(["rn", "config", "get", "value_name_1", "value_name_2"]);
    let config_args = unwrap_variant!(cli.subcommand, Command::Config);
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
    let config_args = unwrap_variant!(cli.subcommand, Command::Config);
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
    let rename_args = unwrap_variant!(cli.subcommand, Command::Rename);
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
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
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
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
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
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
    let open_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Open);
    assert_eq!(open_args.name, "note_1");
}

#[test]
fn test_archive_open_with_editor_short() {
    let cli = Cli::parse_from(["rn", "archive", "open", "-e", "nvim", "note_1"]);
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
    let open_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Open);
    assert_eq!(open_args.editor.unwrap(), "nvim");
    assert_eq!(open_args.name, "note_1");

    let cli = Cli::parse_from(["rn", "archive", "open", "note_1", "-e", "nvim"]);
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
    let open_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Open);
    assert_eq!(open_args.editor.unwrap(), "nvim");
    assert_eq!(open_args.name, "note_1");
}

#[test]
fn test_archive_open_with_editor_long() {
    let cli = Cli::parse_from(["rn", "archive", "open", "--editor", "nvim", "note_1"]);
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
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
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
    let restore_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Restore);
    assert_eq!(restore_args.archive_name, "note_1");
    assert_eq!(restore_args.new_name, None);
    assert!(!restore_args.force);
}

#[test]
fn test_archive_restore_with_new_name() {
    let cli = Cli::parse_from(["rn", "archive", "restore", "note_1", "--new-name", "note"]);
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
    let restore_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Restore);
    assert_eq!(restore_args.archive_name, "note_1");
    assert_eq!(restore_args.new_name.unwrap(), "note");
    assert!(!restore_args.force);
}

#[test]
fn test_archive_restore_with_force() {
    let cli = Cli::parse_from(["rn", "archive", "restore", "note_1", "--force"]);
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
    let restore_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Restore);
    assert_eq!(restore_args.archive_name, "note_1");
    assert_eq!(restore_args.new_name, None);
    assert!(restore_args.force);
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
    let archive_args = unwrap_variant!(cli.subcommand, Command::Archive);
    let remove_args = unwrap_variant!(archive_args.subcommand, ArchiveSubcommand::Remove);
    assert_eq!(remove_args.name, "note_1");
}
