use std::sync::mpsc::Receiver;

pub enum DownloadMessage {
    Progress(f32),
    Finished,
    Error(String),
}

pub struct DownloadContext {
    receiver: Receiver<DownloadMessage>,
}

impl DownloadContext {
    pub fn new(receiver: Receiver<DownloadMessage>) -> Self {
        Self { receiver }
    }
    
    pub fn receiver(&self) -> &Receiver<DownloadMessage> {
        &self.receiver
    }
}
