pub enum LogCategory {
    Download,
    Load,
    Parse,
    Error,
    Command,
    File,
}

impl LogCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Download => "Download",
            Self::Load => "Load",
            Self::Parse => "Parse",
            Self::Error => "Error",
            Self::Command => "Command",
            Self::File => "File",
        }
    }
}

pub struct LogMessage {
    category: LogCategory,
    content: String
}

impl LogMessage {
    fn new(category: LogCategory, content: String) -> Self {
        Self {
            category,
            content
        }
    }

    pub fn category(&self) -> &'static str {
        self.category.as_str()
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

pub struct LogContext {
    log_messages: Vec<LogMessage>,
    current_written_log: Option<LogMessage>,
}

impl LogContext {
    pub fn new() -> Self {
        Self {
            log_messages: Vec::new(),
            current_written_log: None
        }
    }

    pub fn logs(&self) -> std::slice::Iter<'_, LogMessage> {
        self.log_messages.iter()
    }

    /// Push a completed message.
    fn push_log(&mut self, log: LogMessage) {
        self.log_messages.push(log);
    }

    pub fn log(&mut self, category: LogCategory, message: String) {
        self.push_log(LogMessage::new(category, message));
    }

    pub fn log_error(&mut self, message: String) {
        self.log(LogCategory::Error, message);
    }

    // /// Remove a completed message.
    // pub fn remove_log(&mut self, index: usize) {
    //     self.log_messages.remove(index);
    // }

    /// Clear all completed messages.
    pub fn clear_logs(&mut self) {
        self.log_messages.clear();
    }

    /// Begin writing a message.
    pub fn begin_log(&mut self, category: LogCategory, message: String) {
        if self.current_written_log.is_some() {
            return; // don't override the current one
        }
        
        self.current_written_log = Some(LogMessage::new(category, message));
    }

    /// Update the current message.
    pub fn update_log(&mut self, message: String) {
        if let Some(log) = self.current_written_log.as_mut() {
            log.set_content(message);
        }
    }

    /// Finishes writing the current message with a final message.
    pub fn finish_log(&mut self, message: String) {
        self.update_log(message);
        self.end_log();
    }

    fn end_log(&mut self) {
        if let Some(log) = self.current_written_log.take() {
            self.push_log(log);
        }
    }

    pub fn log_started(&self) -> bool {
        self.current_written_log.is_some()
    }
    
    pub fn current_written_log(&self) -> Option<&LogMessage> {
        self.current_written_log.as_ref()
    }
}
