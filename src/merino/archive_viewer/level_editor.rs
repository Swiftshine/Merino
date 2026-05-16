mod add_object;
mod canvas;
mod canvas_settings;
mod contexts;
pub(crate) mod docking;
mod editable;
mod input;
mod interact_object;
mod message;
mod object_image;
mod object_properties;
mod params;
mod settings;
mod ui;

use std::path::PathBuf;

use crate::merino::{
    archive_viewer::level_editor::{
        contexts::{
            canvas_context::CanvasContext, message_context::MessageContext,
            parameter_context::ParameterContext,
        },
        docking::LevelEditorTab,
    },
    game::mapbin::Mapdata, util::res_folder::get_subfolder,
};
use anyhow::Result;

pub struct LevelEditor {
    // data
    mapdata: Option<Mapdata>,

    // contexts
    canvas_context: CanvasContext,
    message_context: MessageContext,
    parameter_context: ParameterContext,

    // docking
    pub(crate) dock_state: Option<egui_dock::DockState<LevelEditorTab>>,
    tab_to_open: Option<LevelEditorTab>,
}

impl LevelEditor {
    pub fn new() -> Self {
        let dock_state = if let Ok(state) = Self::read_dock_state() {
            state
        } else {
            Self::default_dock()
        };

        let dock_state = Some(dock_state);

        Self {
            mapdata: None,
            canvas_context: CanvasContext::new(),
            message_context: MessageContext::new(),
            parameter_context: ParameterContext::new(),
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

    pub fn on_exit(&mut self) {
        // save settings
        let _ = self.canvas_context.settings().write();
        // save dock data
        let _ = self.write_dock_state();
    }

    pub fn get_level_editor_folder() -> Result<PathBuf> {
        get_subfolder("level_editor")
    }
}
