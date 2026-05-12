use anyhow::Result;
use gfarch::gfarch;
use rfd::FileDialog;
use std::{collections::BTreeMap, fs};

/// Contains files.
pub struct FileContext {
    archive_contents: BTreeMap<String, Vec<u8>>,
    selected_file: Option<String>,
}

impl FileContext {
    pub fn new() -> Self {
        Self {
            archive_contents: Default::default(),
            selected_file: None,
        }
    }

    pub fn has_files(&self) -> bool {
        !self.archive_contents.is_empty()
    }

    pub fn archive_contents(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.archive_contents
    }

    pub fn selected_file(&self) -> Option<&String> {
        self.selected_file.as_ref()
    }

    pub fn set_selected_file(&mut self, selected_file: Option<String>) {
        self.selected_file = selected_file;
    }
}

impl FileContext {
    pub fn open_archive(&mut self) -> Result<bool> {
        let Some(path) = FileDialog::new()
            .add_filter("Good-Feel Archive", &["gfa"])
            .pick_file()
        else {
            return Ok(false); // user exited
        };

        let data = fs::read(path)?;

        self.archive_contents = gfarch::extract(&data)?.into_iter().collect();

        // if we have archive contents then clear everything
        self.clear_all();

        Ok(true)
    }

    /// Clears selections, etc.
    fn clear_all(&mut self) {
        self.selected_file = None;
        // archive contents were rewritten anyway
    }
}
