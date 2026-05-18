use std::{fs, path::PathBuf};

use crate::merino::{
    archive_viewer::level_editor::{
        LevelEditor, object_image::ImageBank, settings::NodeEditSettings,
    },
    game::mapbin::{MapNodeType, NodeChildType, NodePath},
    util::camera::CanvasCamera,
};
use anyhow::Result;
use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

const CANVAS_SETTINGS_FILE: &str = "canvas_settings.json";

#[derive(Default, Serialize, Deserialize)]
pub struct CanvasSettings {
    node_edit_settings: EnumMap<MapNodeType, NodeEditSettings>,
    display_grid: bool,
    snap_to_grid: bool,
    display_squares_for_images: bool,
    snap_to_start: bool,
    show_dummy_terrain: bool,
}

impl CanvasSettings {
    fn new() -> Self {
        if let Some(loaded) = Self::read() {
            loaded
        } else {
            Self {
                snap_to_grid: true,
                display_grid: true,
                snap_to_start: true,
                ..Default::default()
            }
        }
    }

    pub fn node_edit_settings_mut(&mut self) -> &mut EnumMap<MapNodeType, NodeEditSettings> {
        &mut self.node_edit_settings
    }

    pub fn display_grid(&self) -> bool {
        self.display_grid
    }

    pub fn display_grid_mut(&mut self) -> &mut bool {
        &mut self.display_grid
    }

    pub fn snap_to_grid(&self) -> bool {
        self.snap_to_grid
    }

    pub fn snap_to_grid_mut(&mut self) -> &mut bool {
        &mut self.snap_to_grid
    }

    pub fn display_squares_for_images(&self) -> bool {
        self.display_squares_for_images
    }

    pub fn display_squares_for_images_mut(&mut self) -> &mut bool {
        &mut self.display_squares_for_images
    }

    fn get_file_path() -> Result<PathBuf> {
        Ok(LevelEditor::get_level_editor_folder()?.join(CANVAS_SETTINGS_FILE))
    }

    pub fn read() -> Option<Self> {
        let path = Self::get_file_path().ok()?;
        let json = fs::read_to_string(path).ok()?;

        serde_json::from_str::<Self>(&json).ok()
    }

    pub fn write(&self) -> Result<()> {
        let path = Self::get_file_path()?;
        let json = serde_json::to_string_pretty(&self)?;

        fs::write(path, json)?;
        Ok(())
    }

    pub fn snap_to_start_mut(&mut self) -> &mut bool {
        &mut self.snap_to_start
    }

    pub fn snap_to_start(&self) -> bool {
        self.snap_to_start
    }
    
    pub fn show_dummy_terrain_mut(&mut self) -> &mut bool {
        &mut self.show_dummy_terrain
    }
    
    pub fn show_dummy_terrain(&self) -> bool {
        self.show_dummy_terrain
    }
}
pub enum CanvasTarget {
    /// Create a new child of this type and attach it to this parent node.
    NewNode(NodeChildType, NodePath),
    /// Create a new child of this type with this name and attach it to this parent node.
    NewNamedNode(NodeChildType, String, NodePath),
    /// Search for an existing node to attach it to this parent node.
    Search(NodePath),
}

impl CanvasTarget {
    pub fn new_to_root(child_type: NodeChildType) -> Self {
        Self::NewNode(child_type, NodePath::root())
    }

    pub fn new_to_node(child_type: NodeChildType, parent: NodePath) -> Self {
        Self::NewNode(child_type, parent)
    }

    // pub fn new_named_to_node(child_type: NodeChildType, name: String, parent: NodePath) -> Self {
    //     Self::NewNamedNode(child_type, name, parent)
    // }

    pub fn new_named_to_root(child_type: NodeChildType, name: String) -> Self {
        Self::NewNamedNode(child_type, name, NodePath::root())
    }

    pub fn search(parent: NodePath) -> Self {
        Self::Search(parent)
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::NewNode(child_type, _) => child_type.to_string(),
            Self::Search(_) => "Searching".to_string(),
            Self::NewNamedNode(child_type, name, _) => format!("{name} ({child_type})"),
        }
    }
}

pub struct CanvasContext {
    camera: CanvasCamera,
    selected_node_paths: Vec<NodePath>,
    settings: CanvasSettings,
    target: Option<CanvasTarget>,
    image_bank: ImageBank,
    canvas_rect: egui::Rect,
}

impl CanvasContext {
    pub fn new() -> Self {
        Self {
            camera: CanvasCamera::default(),
            selected_node_paths: Vec::new(),
            settings: CanvasSettings::new(),
            target: None,
            image_bank: ImageBank::default(),
            canvas_rect: egui::Rect::NOTHING,
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

    pub fn world_to_camera(&self, pos: egui::Vec2) -> egui::Vec2 {
        self.camera.world_to_camera(pos)
    }

    pub fn camera_to_world(&self, pos: egui::Vec2) -> egui::Vec2 {
        self.camera.camera_to_world(pos)
    }

    pub fn camera_pan(&mut self, delta: egui::Vec2) {
        self.camera.pan(delta / self.camera.zoom);
    }

    pub fn camera_zoom(&self) -> f32 {
        self.camera.zoom
    }

    pub fn set_camera_zoom(&mut self, zoom: f32) {
        self.camera.zoom = zoom;
    }

    pub fn camera_focus(&mut self, world_pos: egui::Vec2) {
        self.camera.center(world_pos, self.canvas_rect);
    }

    // pub fn canvas_rect(&self) -> egui::Rect {
    //     self.canvas_rect
    // }

    pub fn set_canvas_rect(&mut self, canvas_rect: egui::Rect) {
        self.canvas_rect = canvas_rect;
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
        self.target = None;
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
        self.target.as_ref().is_some_and(|t| match t {
            CanvasTarget::NewNode(_, _) => true,
            CanvasTarget::NewNamedNode(_, _, _) => true,
            _ => false,
        })
    }

    pub fn is_target_search(&self) -> bool {
        self.target.as_ref().is_some_and(|t| match t {
            CanvasTarget::Search(_) => true,
            _ => false,
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

    pub fn prune_invalid_selections(&mut self) {
        self.selected_node_paths.retain(|path| {
            let root_settings = &self.settings.node_edit_settings[MapNodeType::MapSet];

            if !root_settings.visible || !root_settings.editable {
                return false;
            }

            // every node in the path must be visible + editable

            path.iter().all(|step| {
                let node_type = MapNodeType::from(step.node_type());
                let settings = &self.settings.node_edit_settings[node_type];

                settings.visible && settings.editable
            })
        });
    }

    pub fn settings_mut(&mut self) -> &mut CanvasSettings {
        &mut self.settings
    }

    pub fn settings(&self) -> &CanvasSettings {
        &self.settings
    }

    pub fn image_bank_mut(&mut self) -> &mut ImageBank {
        &mut self.image_bank
    }
}
