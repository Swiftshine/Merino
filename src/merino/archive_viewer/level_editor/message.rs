use crate::merino::archive_viewer::level_editor::{
    LevelEditor, contexts::{log_context::LogCategory, message_context::Command}, docking::LevelEditorTab,
};

impl LevelEditor {
    pub fn process_messages(&mut self) {
        let commands = self.message_context.take_commands();
        let mut additional_commands = Vec::new();

        for command in commands {
            match command {
                Command::SelectNode(path) => {
                    self.canvas_context.select_node(path);
                    self.open_tab(LevelEditorTab::ObjectProperties);
                }

                Command::AddToSelection(path) => {
                    self.canvas_context.add_node_to_selection(path);
                }

                Command::RemoveNode(path) => {
                    if self
                        .mapdata
                        .as_mut()
                        .unwrap()
                        .remove_node_at_path(path)
                        .is_some()
                    {
                        // deselect node
                        self.canvas_context.clear_selections();
                    }
                }

                Command::FocusParentOf(path) => {
                    let parent_path = path.parent();

                    if !parent_path.is_root() {
                        additional_commands.push(Command::Focus(parent_path));
                    } else {
                        self.log_context.log(LogCategory::Command, "The parent node is the root.".to_string());
                    }
                }

                Command::MakeChildOf(child, new_parent) => {
                    self.mapdata.as_mut().unwrap().move_node(child, new_parent);
                }

                Command::Focus(path) => {
                    let world_pos = self.mapdata.as_ref().unwrap().get_node_at_path(&path).unwrap().node_data.position();
                    self.canvas_context.camera_focus(world_pos.into());
                    additional_commands.push(Command::SelectNode(path));
                }
            }
        }

        // handle any additional commands
        self.message_context.push_commands(additional_commands);
    }
}
