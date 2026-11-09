use core::fmt;
use std::path::{Path, PathBuf};

use sim_core::SimError;

/// Why data could not be loaded or a scenario could not run.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataError {
    /// The file the problem is in, when there is one.
    pub path: Option<PathBuf>,
    pub message: String,
}

impl DataError {
    pub(crate) fn in_file(path: &Path, message: impl Into<String>) -> DataError {
        DataError {
            path: Some(path.to_path_buf()),
            message: message.into(),
        }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.path {
            Some(path) => write!(f, "{}: {}", path.display(), self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for DataError {}

impl From<SimError> for DataError {
    fn from(error: SimError) -> DataError {
        DataError {
            path: None,
            message: error.to_string(),
        }
    }
}
