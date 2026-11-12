mod custom_parser;
pub mod error;

#[cfg(test)]
mod tests;

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use custom_parser::valid_note_name;

#[derive(Parser)]
#[command(version)]
#[command(name = "rn")]
#[command(about = "CLI notes manager")]
#[command(disable_help_subcommand = true)]
#[command(flatten_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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
    #[command(after_help = "To activate completions do the following:\
            \n\nFor `zsh` add the script to a directory in your $FPATH.\
            \ne.g. `rn completions --shell zsh > ~/.local/share/zsh/completions/_rn`")]
    Completions(CompletionArgs),

    #[command(about = "View and manage archive")]
    Archive(ArchiveArgs),
}

#[derive(Args, Debug)]
pub struct NewArgs {
    #[arg(help = "Name of the note to be created")]
    #[arg(value_parser=valid_note_name)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct OpenArgs {
    #[arg(help = "Name of the note to open")]
    #[arg(value_parser=valid_note_name)]
    pub name: String,

    #[arg(help = "Editor command used to open the note")]
    #[arg(short, long)]
    pub editor: Option<String>,

    #[arg(help = "Create note if it does not exist")]
    #[arg(short, long)]
    pub new: bool,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    #[arg(help = "Name of the note to be deleted")]
    #[arg(value_parser=valid_note_name)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct RenameArgs {
    #[arg(help = "Name of the note to rename")]
    #[arg(value_parser=valid_note_name)]
    pub name: String,

    #[arg(help = "New name of the note")]
    #[arg(value_parser=valid_note_name)]
    pub new_name: String,

    #[arg(help = "Overwrite the note, if <NEW_NAME> is already taken")]
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(Subcommand, Debug)]
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

#[derive(Subcommand, Debug)]
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
    #[arg(value_parser=valid_note_name)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ArchiveOpenArgs {
    #[arg(help = "Name of the note to open")]
    #[arg(value_parser=valid_note_name)]
    pub name: String,

    #[arg(help = "Editor command used to open the note")]
    #[arg(short, long)]
    pub editor: Option<String>,
}

#[derive(Args, Debug)]
pub struct ArchiveRestoreArgs {
    #[arg(help = "Name of the note to restore from archive")]
    #[arg(value_parser=valid_note_name)]
    pub archive_name: String,

    #[arg(help = "New name of the note after its restored")]
    #[arg(short, long)]
    #[arg(value_parser=valid_note_name)]
    pub new_name: Option<String>,

    #[arg(help = "Replace existing note if name is already taken")]
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Debug)]
pub struct ArchiveRemoveArgs {
    #[arg(help = "Name of the note to delete from archive")]
    #[arg(value_parser=valid_note_name)]
    pub name: String,
}
