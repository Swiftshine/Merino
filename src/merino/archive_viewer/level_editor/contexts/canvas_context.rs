use enum_map::EnumMap;

use crate::merino::{
    archive_viewer::level_editor::settings::NodeEditSettings,
    game::mapbin::{MapNodeType, NodeChildType, NodePath},
    util::camera::CanvasCamera,
};

#[derive(Default)]
struct CanvasSettings {
    node_edit_settings: EnumMap<MapNodeType, NodeEditSettings>,
    display_grid: bool,
    snap_to_grid: bool,
}

impl CanvasSettings {
    // todo! load from file
    fn new() -> Self {
        Self {
            snap_to_grid: true,
            display_grid: true,
            ..Default::default()
        }
    }
}
pub enum CanvasTarget {
    /// Create a new child of this type and attach it this parent node.
    NewNode(NodeChildType, NodePath),
}

impl CanvasTarget {
    pub fn new_to_root(child_type: NodeChildType) -> Self {
        Self::NewNode(child_type, NodePath::root())
    }

    pub fn new_to_node(child_type: NodeChildType, parent: NodePath) -> Self {
        Self::NewNode(child_type, parent)
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::NewNode(child_type, _) => child_type.to_string(),
        }
    }
}

pub struct CanvasContext {
    camera: CanvasCamera,
    selected_node_paths: Vec<NodePath>,
    settings: CanvasSettings,
    target: Option<CanvasTarget>,
}

impl CanvasContext {
    pub fn new() -> Self {
        Self {
            camera: CanvasCamera::default(),
            selected_node_paths: Vec::new(),
            settings: CanvasSettings::new(),
            target: None,
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

    pub fn convert_to_camera(&self, pos: egui::Vec2) -> egui::Vec2 {
        self.camera.convert_to_camera(pos)
    }

    pub fn convert_from_camera(&self, pos: egui::Vec2) -> egui::Vec2 {
        self.camera.convert_from_camera(pos)
    }

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

    /// Only one node can have its properties edited.
    pub fn can_edit_node_properties(&mut self) -> bool {
        self.selected_node_paths.len() == 1
    }

    pub fn selected_node_paths(&self) -> &Vec<NodePath> {
        &self.selected_node_paths
    }

    pub fn set_target(&mut self, target: Option<CanvasTarget>) {
        self.target = target;
    }

    pub fn is_target_new(&self) -> bool {
        self.target.as_ref().is_some_and(|t| {
            match t {
                CanvasTarget::NewNode(_, _) => true,
                // _ => false
            }
        })
    }

    pub fn take_target(&mut self) -> Option<CanvasTarget> {
        self.target.take()
    }

    pub fn draw_grid(
        &self,
        painter: &egui::Painter,
        rect: egui::Rect,
        size: f32,
        color: egui::Color32,
    ) {
        self.camera.draw_grid(painter, rect, size, color);
    }

    pub fn snap_to_grid(&self) -> bool {
        self.settings.snap_to_grid
    }
}
