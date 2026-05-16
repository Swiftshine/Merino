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

        self.selected_file = None;

        Ok(true)
    }

    pub fn save_archive(&self) -> Result<()> {
        let Some(path) = FileDialog::new()
            .add_filter("Good-Feel ARchive", &["gfa"])
            .save_file()
        else {
            return Ok(());
        };

        let mut archive: Vec<(String, Vec<u8>)> =
            self.archive_contents.clone().into_iter().collect();
        archive.sort_by_key(|(name, _)| name.to_lowercase());

        let file = gfarch::pack_from_files(
            &archive,
            gfarch::Version::V3_1,
            gfarch::CompressionType::BPE,
            gfarch::GFCPOffset::Default,
        );

        fs::write(path, file)?;

        Ok(())
    }

    pub fn replace_current_file_contents(&mut self, new_contents: Vec<u8>) {
        let selected = self.selected_file().unwrap().clone();

        *self.archive_contents.get_mut(&selected).unwrap() = new_contents;
    }
}
