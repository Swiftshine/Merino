mod ui;
mod contexts;
mod canvas;
pub(crate) mod docking;
mod interact;
mod settings;
mod input;
mod message;
mod object_properties;

use crate::merino::{archive_viewer::level_editor::{contexts::{canvas_context::CanvasContext, message_context::MessageContext}, docking::LevelEditorTab}, game::mapbin::Mapdata};
use anyhow::Result;

pub struct LevelEditor {
    // data
    mapdata: Option<Mapdata>,

    // contexts
    canvas_context: CanvasContext,
    message_context: MessageContext,

    // docking
    dock_state: Option<egui_dock::DockState<LevelEditorTab>>,
    tab_to_open: Option<LevelEditorTab>,
}

impl LevelEditor {
    pub fn new() -> Self {
        // todo! load from file
        let dock_state = Some(Self::default_dock());

        Self {
            mapdata: None,
            canvas_context: CanvasContext::new(),
            message_context: MessageContext::new(),
            dock_state,
            tab_to_open: None,
        }
    }

    pub fn load_mapdata(&mut self, bytes: &[u8]) -> Result<()> {
        match Mapdata::read(bytes) {
            Ok(mapdata) => {
                self.mapdata = Some(mapdata);
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

    pub fn has_mapdata(&self) -> bool {
        self.mapdata.is_some()
    }
}
