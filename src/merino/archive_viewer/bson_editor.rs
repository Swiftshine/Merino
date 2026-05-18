mod ui;

use anyhow::Result;

pub struct BSONEditor {
    pub json_string: String,
    pub writable_data: Option<Vec<u8>>,
    pub error_message: Option<String>,
    pub show_error_popup: bool,
}

impl BSONEditor {
    pub fn new() -> Self {
        Self {
            json_string: String::new(),
            writable_data: None,
            error_message: None,
            show_error_popup: false,
        }
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

    pub fn has_writable_data(&self) -> bool {
        self.writable_data.is_some()
    }

    pub fn take_writable_data(&mut self) -> Option<Vec<u8>> {
        self.writable_data.take()
    }
}
