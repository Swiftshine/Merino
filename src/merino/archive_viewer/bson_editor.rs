mod ui;

use std::fs;

use anyhow::Result;
use rfd::FileDialog;

pub struct BSONEditor {
    pub json_string: String,
    pub writable_data: Option<Vec<u8>>,
    pub error_message: Option<String>,
    pub show_error_popup: bool,
    pub is_individual_file: bool,
}

impl BSONEditor {
    pub fn new() -> Self {
        Self {
            json_string: String::new(),
            writable_data: None,
            error_message: None,
            show_error_popup: false,
            is_individual_file: false,
        }
    }

    pub fn clear(&mut self) {
        self.json_string.clear();
    }

    pub fn load_bson(&mut self, data: &[u8]) -> Result<()> {
        let bson = gfbson::read(data, gfbson::Endianness::Auto)?;
        self.json_string = gfbson::to_json(&bson, true)?;
        Ok(())
    }

    pub fn write_bson(&self) -> Result<Vec<u8>> {
        let bson = gfbson::from_json(&self.json_string)?;
        let bytes = gfbson::write(&bson, 3, gfbson::Endianness::Big)?;
        Ok(bytes)
    }

    pub fn write_bson_to_file(&self) -> Result<()> {
        let Some(path) = FileDialog::new()
            .add_filter("Good-Feel BSON File", &["bson", "mappath", "MapScene"])
            .save_file()
        else {
            return Ok(());
        };

        let bytes = self.write_bson()?;
        fs::write(path, bytes)?;

        Ok(())
    }

    pub fn import_json(&mut self) -> Result<()> {
        let Some(path) = FileDialog::new()
            .add_filter("JSON File", &["json"])
            .pick_file()
        else {
            return Ok(());
        };

        self.json_string = fs::read_to_string(path)?;

        Ok(())
    }

    pub fn export_json(&self) -> Result<()> {
        let Some(path) = FileDialog::new()
            .add_filter("JSON File", &["json"])
            .save_file()
        else {
            return Ok(());
        };

        fs::write(path, self.json_string.as_bytes())?;

        Ok(())
    }

    pub fn has_writable_data(&self) -> bool {
        self.writable_data.is_some()
    }

    pub fn take_writable_data(&mut self) -> Option<Vec<u8>> {
        self.writable_data.take()
    }

    pub fn set_is_individual_file(&mut self, individual_file: bool) {
        self.is_individual_file = individual_file;
    }
}
