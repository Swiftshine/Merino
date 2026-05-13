use crate::merino::archive_viewer::level_editor::{
    LevelEditor, contexts::message_context::Command, docking::LevelEditorTab,
};

impl LevelEditor {
    pub fn process_messages(&mut self) {
        let commands = self.message_context.take_commands();
        // let mut additional_commands = Vec::new();

        for command in commands {
            match command {
                Command::SelectNode(path) => {
                    self.canvas_context.select_node(path);
                    self.open_tab(LevelEditorTab::ObjectProperties);
                }

                Command::AddToSelection(path) => {
                    self.canvas_context.add_node_to_selection(path);
                }
            }
        }

        // // handle any additional commands
        // self.message_context.push_commands(additional_commands);
    }
}
