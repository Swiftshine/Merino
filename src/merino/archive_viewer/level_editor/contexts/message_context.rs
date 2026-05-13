/// Tell the editor to do something at the next opportunity.
pub enum Command { }
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
    
    /// Take ownership of all commands.
    pub fn take_commands(&mut self) -> Vec<Command> {
        std::mem::take(&mut self.commands)
    }

    /// Add command to command list.
    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }
}
