mod ui;

use crate::merino::game::mapbin::Mapdata;
use anyhow::Result;

pub struct LevelEditor {
    mapdata: Option<Mapdata>
}

impl LevelEditor {
    pub fn new() -> Self {
        Self {
            mapdata: None,
        }
    }

    pub fn load_mapdata(&mut self, bytes: &[u8]) -> Result<()> {
        match Mapdata::read(bytes) {
            Ok(mapdata) => {
                self.mapdata = Some(mapdata);
                println!("ok!");
            }

            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    // pub fn save_mapdata() -> Result<Vec<u8>> {
    //     todo!()
    // }
}
