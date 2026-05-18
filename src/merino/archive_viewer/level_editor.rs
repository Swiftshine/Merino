mod add_object;
mod canvas;
mod canvas_settings;
mod contexts;
pub(crate) mod docking;
mod download;
mod editable;
mod input;
mod interact_object;
mod log;
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
            canvas_context::CanvasContext,
            download_context::DownloadContext,
            log_context::{LogCategory, LogContext},
            message_context::MessageContext,
            parameter_context::ParameterContext,
        },
        docking::LevelEditorTab,
    },
    game::mapbin::Mapdata,
    util::res_folder::get_subfolder,
};
use anyhow::Result;
use tokio::runtime::Runtime;

pub struct LevelEditor {
    // data
    mapdata: Option<Mapdata>,
    writable_data: Option<Vec<u8>>,

    // contexts
    canvas_context: CanvasContext,
    message_context: MessageContext,
    parameter_context: ParameterContext,
    download_context: Option<DownloadContext>,
    log_context: LogContext,

    // docking
    pub(crate) dock_state: Option<egui_dock::DockState<LevelEditorTab>>,
    tab_to_open: Option<LevelEditorTab>,

    // other
    runtime: tokio::runtime::Runtime,
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
            writable_data: None,
            canvas_context: CanvasContext::new(),
            message_context: MessageContext::new(),
            parameter_context: ParameterContext::new(),
            download_context: None,
            log_context: LogContext::new(),
            dock_state,
            runtime: Runtime::new().unwrap(),
            tab_to_open: None,
        }
    }

    pub fn load_mapdata(&mut self, bytes: &[u8]) -> Result<()> {
        match Mapdata::read(bytes) {
            Ok(mapdata) => {
                self.mapdata = Some(mapdata);
                self.log_context
                    .log(LogCategory::File, "Successfully read mapdata.".to_string());
            }

            Err(e) => {
                self.log_context
                    .log_error(format!("Could not read mapdata: {e}"));
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn write_mapdata(&mut self) {
        match self.mapdata.as_ref().unwrap().write() {
            Ok(data) => {
                self.log_context
                    .log(LogCategory::File, "Successfully wrote mapdata.".to_string());
                self.writable_data = Some(data);
            }

            Err(e) => {
                self.log_context
                    .log_error(format!("Could not write mapdata: {e}"));
            }
        }
    }

    pub fn has_mapdata(&self) -> bool {
        self.mapdata.is_some()
    }

    pub fn on_start(&mut self) {
        // no need to log a "failure"

        // load parameter data
        if let Ok(json) = Self::load_params() {
            let _ = self.parse_params(json);
        }

        // load image data
        if let Ok(json) = Self::load_image_data() {
            let _ = self.parse_image_data(json);
        }
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

    pub fn has_writable_data(&self) -> bool {
        self.writable_data.is_some()
    }

    pub fn take_writable_data(&mut self) -> Option<Vec<u8>> {
        self.writable_data.take()
    }
}
