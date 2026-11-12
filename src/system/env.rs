#[cfg(test)]
use std::path::Path;

use std::path::PathBuf;

use crate::system::error::Error;
use crate::system::error::Result;

/// Gets the home directory path from environment variable.
///
/// Calls [`std::env::home_dir`].
///
/// # Errors
///
/// Returns [`super::error::Error::NoHomeDir`] if call to [`std::env::home_dir`] fails.
pub fn home_dir() -> Result<PathBuf> {
    std::env::home_dir().ok_or(Error::NoHomeDir)
}

/// Set home directory to given `path`.
///
/// Currently only for tests.
#[cfg(test)]
pub fn set_home_dir(path: &Path) {
    std::env::set_var("HOME", path);
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)] // ok for tests

    use std::{path::PathBuf, str::FromStr};

    use crate::system::env::{home_dir, set_home_dir};

    #[test]
    fn test_set_and_get() {
        let expected_home_dir = PathBuf::from_str("/tmp").unwrap();
        set_home_dir(&expected_home_dir);
        let home_dir = home_dir().unwrap();
        assert_eq!(expected_home_dir, home_dir);
    }
}
