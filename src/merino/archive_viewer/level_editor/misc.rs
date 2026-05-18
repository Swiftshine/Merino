use crate::merino::{archive_viewer::level_editor::LevelEditor, game::mapbin::{NodeChildType, NodeData}};

/// Delay doing something
pub enum WaitState {
    /// Not waiting for anything
    Idle,
    // Waiting for something
    Waiting,
    /// Done waiting for something
    Done
}

impl LevelEditor {
    pub fn snap_to_start(&mut self) -> Option<()> {
        // find the "START" position. it should be in the root node

        let root = &self.mapdata.as_ref()?.root;
        let locators = root.children_vec(NodeChildType::MapLocator)?;

        let start_node = locators.iter().find(|node|{
            let NodeData::MapLocator { name, .. } = &node.node_data else {
                return false;
            };

            name.as_str() == "START"
        })?;


        // snap to position
        let position = start_node.node_data.position();
        self.canvas_context.set_camera_zoom(10.0);
        self.canvas_context.camera_focus(position.into());
        Some(())
    }
}
