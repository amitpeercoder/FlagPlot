//! Where `sim-data` reads a file from (decision 0229, rulings 8 and 9).
//!
//! Every reader in this crate takes its text from a [`Source`]. There are
//! three kinds: the [`Disk`], which every `load` function uses and goes on
//! using; a [`Pack`], a set of files held in memory by the paths they were
//! asked for; and [`Noting`], which reads through another source and keeps
//! every file it was asked for, by the path asked, with its text. A pack is
//! made by loading a scenario through a noting source, so it holds exactly
//! the files that load read and no list is kept by hand.
//!
//! Paths are used as they are asked for, with their `..` parts left in: a
//! pack holds `data/scenarios/table-boats/../../map/midway.csv` under that
//! name. Every file this crate reads is text.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Where a file's bytes come from. A failure is its words, as the operating
/// system gives them for a file on a disk.
pub trait Source {
    /// The text of the file at `path`.
    fn read_text(&self, path: &Path) -> Result<String, String>;

    /// The bytes of the file at `path`, as they are hashed.
    fn read_bytes(&self, path: &Path) -> Result<Vec<u8>, String>;

    /// Whether there is a file at `path`.
    fn is_file(&self, path: &Path) -> bool;
}

/// The disk: what every `load` function reads from.
#[derive(Clone, Copy, Debug, Default)]
pub struct Disk;

impl Source for Disk {
    fn read_text(&self, path: &Path) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    fn read_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        fs::read(path).map_err(|e| e.to_string())
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }
}

/// A set of files held in memory, by the paths they were asked for.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pack {
    files: BTreeMap<PathBuf, String>,
}

impl Pack {
    /// A pack with no file.
    pub fn new() -> Pack {
        Pack::default()
    }

    /// A pack of these files: each path with its text.
    pub fn from_files(files: BTreeMap<PathBuf, String>) -> Pack {
        Pack { files }
    }

    /// Adds a file, replacing a file of the same path.
    pub fn insert(&mut self, path: PathBuf, text: String) {
        self.files.insert(path, text);
    }

    /// Takes a file out, giving its text if it was there.
    pub fn remove(&mut self, path: &Path) -> Option<String> {
        self.files.remove(path)
    }

    /// Adds every file of `other`. A path in both must hold the same text,
    /// or the path is the error.
    pub fn merge(&mut self, other: &Pack) -> Result<(), PathBuf> {
        for (path, text) in &other.files {
            match self.files.get(path) {
                Some(held) if held != text => return Err(path.clone()),
                Some(_) => {}
                None => {
                    self.files.insert(path.clone(), text.clone());
                }
            }
        }
        Ok(())
    }

    /// Every file, by path.
    pub fn files(&self) -> &BTreeMap<PathBuf, String> {
        &self.files
    }

    /// Whether there is a file at `path`.
    pub fn contains(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }
}

impl Source for Pack {
    fn read_text(&self, path: &Path) -> Result<String, String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| "the file is not in the pack".to_string())
    }

    fn read_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        self.read_text(path).map(String::into_bytes)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.contains(path)
    }
}

/// A source that reads through another and notes every file it is asked
/// for, by the path asked, with its text.
pub struct Noting<'a> {
    inner: &'a dyn Source,
    seen: RefCell<BTreeMap<PathBuf, String>>,
}

impl<'a> Noting<'a> {
    /// Reads through `inner`, noting what it is asked for.
    pub fn new(inner: &'a dyn Source) -> Noting<'a> {
        Noting {
            inner,
            seen: RefCell::new(BTreeMap::new()),
        }
    }

    /// The files read so far, as a pack.
    pub fn pack(&self) -> Pack {
        Pack {
            files: self.seen.borrow().clone(),
        }
    }
}

impl Source for Noting<'_> {
    fn read_text(&self, path: &Path) -> Result<String, String> {
        let text = self.inner.read_text(path)?;
        self.seen
            .borrow_mut()
            .insert(path.to_path_buf(), text.clone());
        Ok(text)
    }

    fn read_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        let bytes = self.inner.read_bytes(path)?;
        let text = String::from_utf8(bytes.clone())
            .map_err(|_| "the file is not text, and a pack holds text only".to_string())?;
        self.seen.borrow_mut().insert(path.to_path_buf(), text);
        Ok(bytes)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.inner.is_file(path)
    }
}

/// The text of a file, or the error that names the file.
pub(crate) fn read_text(
    source: &dyn Source,
    path: &Path,
) -> Result<String, crate::error::DataError> {
    source
        .read_text(path)
        .map_err(|message| crate::error::DataError::in_file(path, message))
}
