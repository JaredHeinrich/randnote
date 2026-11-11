use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InvalidNoteName {
    #[error("Note name mustn't be empty.")]
    Empty,
    #[error("Note name mustn't end with a seperator.")]
    TrailingSeparator,
    #[error("Note name mustn't be a path")]
    MultiplePathComponents,
    #[error("Note name mustn't be a path")]
    InvalidPathComponent,
    #[error("Note name mustn't have leading or trailing whitespaces.")]
    AdditionalWhitespaces,
    #[error("Note name mustn't start with a dot.")]
    LeadingDot,
}
