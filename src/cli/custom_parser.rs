use super::error::InvalidNoteName;
use std::path::Component;
use std::path::Path;

pub fn valid_note_name(s: &str) -> Result<String, InvalidNoteName> {
    if s.ends_with('/') || s.ends_with('\\') {
        return Err(InvalidNoteName::TrailingSeparator);
    }
    let path = Path::new(s);
    let mut components = path.components();
    let (first, None) = (components.next(), components.next()) else {
        return Err(InvalidNoteName::MultiplePathComponents);
    };
    let Some(first) = first else {
        return Err(InvalidNoteName::Empty);
    };
    let Component::Normal(_) = first else {
        return Err(InvalidNoteName::InvalidPathComponent);
    };
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(InvalidNoteName::Empty);
    }
    if s != trimmed {
        return Err(InvalidNoteName::AdditionalWhitespaces);
    }
    if s.starts_with('.') {
        return Err(InvalidNoteName::LeadingDot);
    }
    Ok(s.to_string())
}
