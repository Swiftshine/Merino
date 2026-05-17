use crate::merino::game::mapbin::NodePath;

/// Tell the editor to do something at the next opportunity.
pub enum Command {
    /// Clear selections and select this node.
    SelectNode(NodePath),
    /// Add this node to the list of selections.
    AddToSelection(NodePath),
    /// Remove this node from the tree.
    RemoveNode(NodePath),
    /// Select and focus the camera on the parent of the node at this path.
    FocusParentOf(NodePath),
    /// Make one node the child of another.
    MakeChildOf(NodePath, NodePath),
    /// Select and focus the camera on the node at the given path.
    Focus(NodePath)
}

impl Command {
    pub fn select_node(path: NodePath) -> Self {
        Self::SelectNode(path)
    }

    pub fn add_to_selection(path: NodePath) -> Self {
        Self::AddToSelection(path)
    }

    pub fn remove_node(path: NodePath) -> Self {
        Self::RemoveNode(path)
    }

    pub fn focus_parent_of(path: NodePath) -> Self {
        Self::FocusParentOf(path)
    }

    pub fn make_child_of(child: NodePath, new_parent: NodePath) -> Self {
        Self::MakeChildOf(child, new_parent)
    }

    pub fn make_child_of_root(child: NodePath) -> Self {
        Self::MakeChildOf(child, NodePath::root())
    }

    pub fn focus(path: NodePath) -> Self {
        Self::Focus(path)
    }
}

pub struct MessageContext {
    commands: Vec<Command>,
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

    /// Add multiple commands to the command list.
    pub fn push_commands(&mut self, commands: impl IntoIterator<Item = Command>) {
        self.commands.extend(commands);
    }
}
