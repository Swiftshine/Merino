use enum_map::EnumMap;

use crate::merino::{archive_viewer::level_editor::settings::NodeEditSettings, game::mapbin::{MapNodeType, NodePath}, util::camera::CanvasCamera};

#[derive(Default)]
struct CanvasSettings {
    node_edit_settings: EnumMap<MapNodeType, NodeEditSettings>,
}

pub struct CanvasContext {
    camera: CanvasCamera,
    selected_node_paths: Vec<NodePath>,
    settings: CanvasSettings,
}

impl CanvasContext {
    pub fn new() -> Self {
        Self {
            camera: CanvasCamera::default(),
            selected_node_paths: Vec::new(),
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

    // pub fn convert_from_camera(&mut self, pos: egui::Vec2) -> egui::Vec2 {
    //     self.camera.convert_from_camera(pos)
    // }

    pub fn camera_pan(&mut self, delta: egui::Vec2) {
        self.camera.pan(delta / self.camera.zoom);
    }

    pub fn camera_zoom(&self) -> f32 {
        self.camera.zoom
    }
    
    pub fn is_node_selected(&self, path: &NodePath) -> bool {
        self.selected_node_paths.contains(path)
    }

    /// Replace all selections with the given path.
    pub fn select_node(&mut self, path: NodePath) {
        self.clear_selections();
        self.add_node_to_selection(path);
    }

    /// Add given path to the list of selections.
    pub fn add_node_to_selection(&mut self, path: NodePath) {
        if !self.selected_node_paths.contains(&path) {
            self.selected_node_paths.push(path);
        }
    }

    /// Removes all selections
    pub fn clear_selections(&mut self) {
        self.selected_node_paths.clear();
    }
}
