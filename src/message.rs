use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Message {
    Notebook(Vec<String>),
    Archive(Vec<String>),
    CreatedNote,
    DeletedNote,
    CompletionScript(String),
    ConfigValues(Vec<(String, String)>),
    GeneratedConfig(PathBuf),
    Renamed((String, String)),
    ArchivedNote((String, String)),
    RestoredNote((String, String)),
    Empty,
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreatedNote => {
                writeln!(f, "Created note")
            }
            Self::DeletedNote => {
                writeln!(f, "Deleted note")
            }
            Self::Notebook(notes) | Self::Archive(notes) => {
                for name in notes {
                    writeln!(f, "{name}")?;
                }
                Ok(())
            }
            Self::CompletionScript(script) => writeln!(f, "{script}"),
            Self::ConfigValues(config_values) => {
                let name_col_width = config_values
                    .iter()
                    .map(|(n, _)| n.len())
                    .max()
                    .unwrap_or(0);
                for (name, value) in config_values {
                    writeln!(f, "{name:<name_col_width$} : {value}")?;
                }
                Ok(())
            }
            Self::GeneratedConfig(path) => writeln!(f, "Generated config file {}", path.display()),
            Self::Renamed((original, new)) => writeln!(f, "Renamed note {original} to {new}"),
            Self::ArchivedNote((original, archived)) => {
                writeln!(f, "Archived note {original} to {archived}")
            }
            Self::RestoredNote((archived, new)) => writeln!(f, "Restored note {archived} to {new}"),
            Self::Empty => Ok(()),
        }
    }
}
