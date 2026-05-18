use anyhow::Result;
use gfarch::gfarch;
use rfd::FileDialog;
use std::{collections::BTreeMap, fs};

pub enum FileType {
    None,
    BSON
}

/// Contains files.
pub struct FileContext {
    // archive
    archive_contents: BTreeMap<String, Vec<u8>>,
    selected_file: Option<String>,
    // single file
    file_contents: Option<Vec<u8>>,
}

impl FileContext {
    pub fn new() -> Self {
        Self {
            archive_contents: Default::default(),
            selected_file: None,
            file_contents: None,
        }
    }

    pub fn has_archive_contents(&self) -> bool {
        !self.archive_contents.is_empty()
    }

    // pub fn has_file(&self) -> bool {
    //     self.file_contents.is_some()
    // }

    pub fn archive_contents(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.archive_contents
    }

    pub fn selected_file(&self) -> Option<&String> {
        self.selected_file.as_ref()
    }

    pub fn set_selected_file(&mut self, selected_file: Option<String>) {
        self.selected_file = selected_file;
    }

    pub fn file_contents(&self) -> Option<&Vec<u8>> {
        self.file_contents.as_ref()
    }

    pub fn has_file(&self) -> bool {
        self.file_contents.is_some()
    }
    
    // pub fn is_file_selected(&self) -> bool {
    //     self.selected_file.is_some()
    // }
}

impl FileContext {
    pub fn open_archive(&mut self) -> Result<()> {
        let Some(path) = FileDialog::new()
            .add_filter("Good-Feel Archive", &["gfa"])
            .pick_file()
        else {
            return Ok(());
        };

        let data = fs::read(path)?;

        self.archive_contents = gfarch::extract(&data)?.into_iter().collect();
        self.selected_file = None;
        self.file_contents = None;

        Ok(())
    }

    pub fn open_file(&mut self) -> Result<FileType> {
        let Some(path) = FileDialog::new()
        .add_filter("Good-Feel BSON File", &["bson", "mappath", "MapScene"])
        .pick_file() else {
            return Ok(FileType::None);
        };

        self.file_contents = Some(fs::read(path)?);
        self.archive_contents = Default::default();
        self.selected_file = None;

        // change this if any other individual file types need to be supported
        Ok(FileType::BSON)
    }

    pub fn save_archive(&self) -> Result<()> {
        let Some(path) = FileDialog::new()
            .add_filter("Good-Feel Archive", &["gfa"])
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
        if let Some(selected) = self.selected_file() {
            *self.archive_contents.get_mut(&selected.clone()).unwrap() = new_contents;
        }
    }
}
