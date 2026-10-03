use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum Message {
    Notebook(Vec<String>),
    Archive(Vec<String>),
    CreatedNotes(usize),
    DeletedNotes(usize),
    CompletionScript(String),
    ConfigValues(Vec<(String, String)>),
    GeneratedConfig(PathBuf),
    Renamed((String, String)),
    ArchivedNotes(usize),
    RestoredNotes(usize),
    Empty,
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreatedNotes(count) => {
                if *count == 1 {
                    writeln!(f, "Created note")
                } else {
                    writeln!(f, "Created {count} notes")
                }
            }
            Self::DeletedNotes(count) => {
                if *count == 1 {
                    writeln!(f, "Deleted note")
                } else {
                    writeln!(f, "Deleted {count} notes")
                }
            }
            Self::ArchivedNotes(count) => {
                if *count == 1 {
                    writeln!(f, "Archived note")
                } else {
                    writeln!(f, "Archived {count} notes")
                }
            }
            Self::RestoredNotes(count) => {
                if *count == 1 {
                    writeln!(f, "Restored note")
                } else {
                    writeln!(f, "Restored {count} notes")
                }
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
            Self::Empty => Ok(()),
        }
    }
}
