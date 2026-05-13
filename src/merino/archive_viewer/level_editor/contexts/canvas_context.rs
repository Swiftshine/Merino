use enum_map::EnumMap;

use crate::merino::{archive_viewer::level_editor::settings::NodeEditSettings, game::mapbin::MapNodeType, util::camera::CanvasCamera};

#[derive(Default)]
struct CanvasSettings {
    node_edit_settings: EnumMap<MapNodeType, NodeEditSettings>,
}

pub struct CanvasContext {
    camera: CanvasCamera,
    settings: CanvasSettings,
}

impl CanvasContext {
    pub fn new() -> Self {
        Self {
            camera: CanvasCamera::default(),
            settings: CanvasSettings::default(),
        }
    }
    
    pub fn camera_mut(&mut self) -> &mut CanvasCamera {
        &mut self.camera
    }

    pub fn can_view(&self, node_type: MapNodeType) -> bool {
        self.settings.node_edit_settings[node_type].visible
    }

    pub fn can_edit(&self, node_type: MapNodeType) -> bool {
        self.settings.node_edit_settings[node_type].editable
    }

    pub fn convert_to_camera(&mut self, pos: egui::Vec2) -> egui::Vec2 {
        self.camera.convert_to_camera(pos)
    }

    pub fn camera_pan(&mut self, delta: egui::Vec2) {
        self.camera.pan(delta / self.camera.zoom);
    }
    
    // pub fn convert_from_camera(&mut self, pos: egui::Vec2) -> egui::Vec2 {
    //     self.camera.convert_from_camera(pos)
    // }
}
