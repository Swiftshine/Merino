use crate::merino::game::mapbin::NodePath;

/// Tell the editor to do something at the next opportunity.
pub enum Command {
    /// Clear selections and select this node
    SelectNode(NodePath),
    /// Add this node to the list of selections
    AddToSelection(NodePath),
}

impl Command {
    pub fn select_node(path: NodePath) -> Self {
        Self::SelectNode(path)
    }

    pub fn add_to_selection(path: NodePath) -> Self {
        Self::AddToSelection(path)
    }
}
// /// Tell the editor to retrieve data immediately.
// enum Request { }
pub struct MessageContext {
    commands: Vec<Command>,
    // requests: Vec<Request>,
}

impl MessageContext {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// The caller takes ownership of all commands.
    pub fn take_commands(&mut self) -> Vec<Command> {
        std::mem::take(&mut self.commands)
    }

    /// Add command to command list.
    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    // /// Add multiple commands to the command list.
    // pub fn push_commands(&mut self, commands: impl IntoIterator<Item = Command>) {
    //     self.commands.extend(commands);
    // }
}
