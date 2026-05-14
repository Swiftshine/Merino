use crate::merino::archive_viewer::level_editor::{
    LevelEditor, contexts::message_context::Command, docking::LevelEditorTab,
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
                    self.mapdata.as_mut().unwrap().remove_node_at_path(path);
                }

                Command::SelectParentOf(path) => {
                    let mut parent_path = path.clone();
                    parent_path.pop();

                    if !parent_path.is_root() {
                        additional_commands.push(Command::SelectNode(parent_path));
                    }
                }
            }
        }

        // handle any additional commands
        self.message_context.push_commands(additional_commands);
    }
}
